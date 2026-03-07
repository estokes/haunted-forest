mod bridge;

use anyhow::Result;
use bevy::prelude::*;
use netidx::{publisher::Publisher, subscriber::Subscriber, InternalOnly};
use tokio::runtime::Runtime;

pub use bridge::*;

/// Shared async infrastructure for netidx. Owns the tokio runtime and
/// InternalOnly instance, providing Publisher/Subscriber clones to
/// both NetBridge and GraphixPlugin.
#[derive(Resource)]
pub struct NetInfra {
    runtime: Runtime,
    _net: InternalOnly,
    publisher: Publisher,
    subscriber: Subscriber,
}

impl NetInfra {
    fn new() -> Result<Self> {
        let runtime = Runtime::new()?;
        let net = runtime.block_on(InternalOnly::new())?;
        let publisher = net.publisher().clone();
        let subscriber = net.subscriber().clone();
        Ok(Self { runtime, _net: net, publisher, subscriber })
    }

    pub fn publisher(&self) -> Publisher {
        self.publisher.clone()
    }

    pub fn subscriber(&self) -> Subscriber {
        self.subscriber.clone()
    }

    pub fn handle(&self) -> &tokio::runtime::Handle {
        self.runtime.handle()
    }
}

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        let infra = NetInfra::new().expect("failed to initialize netidx");
        let bridge = NetBridge::start(&infra);
        app.insert_resource(infra).insert_resource(bridge);
    }
}
