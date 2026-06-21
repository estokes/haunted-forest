use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use futures::StreamExt;
use netidx::{
    path::Path,
    publisher::{Publisher, Val, Value, WriteRequest},
    subscriber::{Event as SubEvent, SubId, UpdateChan, UpdatesFlags},
};
use poolshark::global::GPooled;
use std::collections::HashMap;
use std::marker::PhantomData;

/// A subscription update received from netidx.
pub struct SubUpdate {
    pub path: Path,
    pub value: Value,
}

/// A write request received from a subscriber writing to a published value.
pub struct WriteEvent {
    pub path: Path,
    pub value: Value,
}

/// Messages from Bevy to the tokio bridge.
enum ToNet {
    Publish { path: Path, value: Value },
    PublishWritable { path: Path, value: Value },
    Subscribe(Path),
    /// Write to a subscribed value (subscriber-side write, triggers WriteRequest on publisher).
    Write { path: Path, value: Value },
    /// Register a receiver for control writes whose path falls under `prefix`
    /// (e.g. `/ship/reactor/control`). Sent once at startup per system.
    RegisterWrites { prefix: Path, tx: Sender<WriteEvent> },
}

/// The canonical control prefix of a path — everything up to and including
/// `/control`, WITHOUT the trailing slash. netidx strips trailing slashes when
/// it canonicalizes a `Path`, so a system that registers `/ship/reactor/control/`
/// is keyed as `/ship/reactor/control`; this returns the matching key for an
/// incoming write like `/ship/reactor/control/target_power`.
fn control_prefix(path: &str) -> Option<&str> {
    let idx = path.find("/control/")?;
    Some(&path[..idx + "/control".len()])
}

/// The Bevy-side handle to the netidx bridge.
#[derive(Resource)]
pub struct NetBridge {
    to_net: Sender<ToNet>,
    from_net: Receiver<SubUpdate>,
}

impl NetBridge {
    pub fn start(infra: &super::NetInfra) -> Self {
        let (to_net_tx, to_net_rx) = unbounded::<ToNet>();
        let (from_net_tx, from_net_rx) = unbounded::<SubUpdate>();
        let publisher = infra.publisher();
        let subscriber = infra.subscriber();
        infra.handle().spawn(bridge_task(to_net_rx, from_net_tx, publisher, subscriber));
        Self { to_net: to_net_tx, from_net: from_net_rx }
    }

    /// Queue a value for publication. Non-blocking.
    pub fn publish(&self, path: Path, value: Value) {
        let _ = self.to_net.send(ToNet::Publish { path, value });
    }

    /// Queue a value for publication as writable. Subscribers can write back to this value.
    pub fn publish_writable(&self, path: Path, value: Value) {
        let _ = self.to_net.send(ToNet::PublishWritable { path, value });
    }

    /// Request a subscription to a path. Updates will arrive via `drain_updates`.
    pub fn subscribe(&self, path: Path) {
        let _ = self.to_net.send(ToNet::Subscribe(path));
    }

    /// Write to a subscribed value. The write goes through netidx and arrives
    /// back as a `WriteEvent` in the registered system's inbox.
    pub fn write(&self, path: Path, value: Value) {
        let _ = self.to_net.send(ToNet::Write { path, value });
    }

    /// Drain all pending subscription updates. Call once per frame.
    pub fn drain_updates(&self) -> impl Iterator<Item = SubUpdate> + '_ {
        self.from_net.try_iter()
    }

    /// Register to receive control writes under `prefix`'s `/control/` group.
    /// Each system gets its own typed `WriteInbox<S>`, so writes route to exactly
    /// one consumer — no shared queue to fan out and `continue` past, and a system
    /// can only ever drain its own writes.
    pub fn register_writes<S: Send + Sync + 'static>(&self, prefix: Path) -> WriteInbox<S> {
        let (tx, rx) = unbounded();
        // Derive the registration key the same way incoming writes are matched,
        // so a trailing slash can't desync the two.
        let key_str = control_prefix(prefix.as_ref()).map(|s| s.to_string());
        let key = key_str.map(Path::from).unwrap_or(prefix);
        let _ = self.to_net.send(ToNet::RegisterWrites { prefix: key, tx });
        WriteInbox { rx, _marker: PhantomData }
    }
}

/// A per-system inbox for control writes. The `S` parameter makes
/// `WriteInbox<Reactor>` a distinct type from `WriteInbox<Thermal>`, so the
/// compiler guarantees a system drains only its own writes.
#[derive(Resource)]
pub struct WriteInbox<S: Send + Sync + 'static> {
    rx: Receiver<WriteEvent>,
    _marker: PhantomData<fn() -> S>,
}

impl<S: Send + Sync + 'static> WriteInbox<S> {
    pub fn drain(&self) -> impl Iterator<Item = WriteEvent> + '_ {
        self.rx.try_iter()
    }
}

/// The tokio-side bridge task that owns netidx publisher/subscriber.
async fn bridge_task(
    from_bevy: Receiver<ToNet>,
    to_bevy: Sender<SubUpdate>,
    publisher: Publisher,
    subscriber: netidx::subscriber::Subscriber,
) {
    log::info!("netidx bridge started");
    let mut pubs: HashMap<Path, Val> = HashMap::new();
    let mut sub_paths: HashMap<SubId, Path> = HashMap::new();
    let mut sub_dvals: HashMap<Path, netidx::subscriber::Dval> = HashMap::new();
    // control prefix -> the owning system's write inbox. Built at startup from
    // RegisterWrites; a write that matches no prefix is logged, not dropped silently.
    let mut write_routes: HashMap<String, Sender<WriteEvent>> = HashMap::new();
    let (sub_tx, mut sub_rx): (UpdateChan, _) = futures::channel::mpsc::channel(4096);
    let (write_tx, mut write_rx) =
        futures::channel::mpsc::channel::<GPooled<Vec<WriteRequest>>>(256);
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(16));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut batch = publisher.start_batch();
                let mut batch_dirty = false;
                while let Ok(msg) = from_bevy.try_recv() {
                    match msg {
                        ToNet::Publish { path, value } => {
                            match pubs.get(&path) {
                                Some(val) => {
                                    val.update(&mut batch, value);
                                    batch_dirty = true;
                                }
                                None => {
                                    match publisher.publish(path.clone(), value) {
                                        Ok(val) => { pubs.insert(path, val); }
                                        Err(e) => log::error!("publish error: {e}"),
                                    }
                                }
                            }
                        }
                        ToNet::PublishWritable { path, value } => {
                            match pubs.get(&path) {
                                Some(val) => {
                                    val.update(&mut batch, value);
                                    batch_dirty = true;
                                }
                                None => {
                                    match publisher.publish(path.clone(), value) {
                                        Ok(val) => {
                                            publisher.writes(val.id(), write_tx.clone());
                                            pubs.insert(path, val);
                                        }
                                        Err(e) => log::error!("publish error: {e}"),
                                    }
                                }
                            }
                        }
                        ToNet::Subscribe(path) => {
                            if sub_dvals.contains_key(&path) {
                                continue;
                            }
                            let dval = subscriber.subscribe(path.clone());
                            dval.updates(UpdatesFlags::empty(), sub_tx.clone());
                            let id = dval.id();
                            sub_paths.insert(id, path.clone());
                            sub_dvals.insert(path, dval);
                        }
                        ToNet::Write { path, value } => {
                            if let Some(dval) = sub_dvals.get(&path) {
                                dval.write(value);
                            } else {
                                log::warn!("write to unsubscribed path: {path}");
                            }
                        }
                        ToNet::RegisterWrites { prefix, tx } => {
                            // Registration rides the 16ms tick; writes require a
                            // prior subscribe, and registration happens at Startup,
                            // so the route is always in place before any write.
                            write_routes.insert(prefix.as_ref().to_string(), tx);
                        }
                    }
                }
                if batch_dirty {
                    batch.commit(None).await;
                }
            }
            Some(mut batch) = sub_rx.next() => {
                for (id, event) in batch.drain(..) {
                    if let SubEvent::Update(value) = event {
                        if let Some(path) = sub_paths.get(&id) {
                            let _ = to_bevy.send(SubUpdate {
                                path: path.clone(),
                                value,
                            });
                        }
                    }
                }
            }
            Some(mut batch) = write_rx.next() => {
                for wr in batch.drain(..) {
                    match control_prefix(wr.path.as_ref()).and_then(|k| write_routes.get(k)) {
                        Some(tx) => {
                            let _ = tx.send(WriteEvent { path: wr.path, value: wr.value });
                        }
                        None => log::warn!("unrouted control write: {}", wr.path),
                    }
                }
            }
        }
    }
}
