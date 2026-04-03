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
}

/// The Bevy-side handle to the netidx bridge.
#[derive(Resource)]
pub struct NetBridge {
    to_net: Sender<ToNet>,
    from_net: Receiver<SubUpdate>,
    writes_from_net: Receiver<WriteEvent>,
}

impl NetBridge {
    pub fn start(infra: &super::NetInfra) -> Self {
        let (to_net_tx, to_net_rx) = unbounded::<ToNet>();
        let (from_net_tx, from_net_rx) = unbounded::<SubUpdate>();
        let (writes_tx, writes_rx) = unbounded::<WriteEvent>();
        let publisher = infra.publisher();
        let subscriber = infra.subscriber();
        infra.handle().spawn(bridge_task(
            to_net_rx,
            from_net_tx,
            writes_tx,
            publisher,
            subscriber,
        ));
        Self {
            to_net: to_net_tx,
            from_net: from_net_rx,
            writes_from_net: writes_rx,
        }
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
    /// back as a WriteEvent via `drain_writes` (if the published value is writable).
    pub fn write(&self, path: Path, value: Value) {
        let _ = self.to_net.send(ToNet::Write { path, value });
    }

    /// Drain all pending subscription updates. Call once per frame.
    pub fn drain_updates(&self) -> impl Iterator<Item = SubUpdate> + '_ {
        self.from_net.try_iter()
    }

    /// Drain all pending write events (from subscribers writing to our published values).
    pub fn drain_writes(&self) -> impl Iterator<Item = WriteEvent> + '_ {
        self.writes_from_net.try_iter()
    }
}

/// The tokio-side bridge task that owns netidx publisher/subscriber.
async fn bridge_task(
    from_bevy: Receiver<ToNet>,
    to_bevy: Sender<SubUpdate>,
    writes_to_bevy: Sender<WriteEvent>,
    publisher: Publisher,
    subscriber: netidx::subscriber::Subscriber,
) {
    log::info!("netidx bridge started");
    let mut pubs: HashMap<Path, Val> = HashMap::new();
    let mut sub_paths: HashMap<SubId, Path> = HashMap::new();
    let mut sub_dvals: HashMap<Path, netidx::subscriber::Dval> = HashMap::new();
    let (sub_tx, mut sub_rx): (UpdateChan, _) =
        futures::channel::mpsc::channel(4096);
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
                    let _ = writes_to_bevy.send(WriteEvent {
                        path: wr.path,
                        value: wr.value,
                    });
                }
            }
        }
    }
}
