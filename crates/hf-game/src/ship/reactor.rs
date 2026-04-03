use bevy::prelude::*;
use arcstr::ArcStr;
use netidx::{path::Path, publisher::{FromValue, Value}};
use crate::net::NetBridge;

/// Accumulated time for periodic debug logging.
#[derive(Resource)]
pub struct ReactorDebugTimer(pub f64);

/// Log reactor state every ~2 seconds to diagnose dynamics.
pub fn reactor_debug_log(reactor: Res<Reactor>, time: Res<Time>, mut timer: ResMut<ReactorDebugTimer>) {
    timer.0 += time.delta_secs_f64();
    if timer.0 < 2.0 {
        return;
    }
    timer.0 = 0.0;
    let r = &*reactor;
    let burn_rate = if r.temperature > 50.0 {
        (r.temperature / 300.0).min(1.0) * r.density * 0.15
    } else {
        0.0
    };
    let fusion_heating = burn_rate * 200.0;
    let radiation_loss = (r.temperature / 100.0).powi(2) * 2.0;
    let active_cooling = r.cooling_rate * 0.02;
    info!(
        "REACTOR | T={:.1}MK dens={:.3} fuel={:.3} field={:.1}T press={:.3} cont={:.2} | \
         burn={:.4} heat={:.1} rad={:.1} cool={:.1} net={:.1} | \
         eff={:.2} power={:.1}GW target={:.0}GW",
        r.temperature, r.density, r.fuel_rate, r.field_strength,
        r.pressure, r.containment,
        burn_rate, fusion_heating, radiation_loss, active_cooling,
        fusion_heating - radiation_loss - active_cooling,
        r.efficiency, r.power_output, r.target_power,
    );
}

/// Fusion reactor plasma physics state.
/// This is a toy model: the plasma wants to expand, the magnetic confinement
/// field holds it in. Fuel injection drives the reaction. If the field can't
/// contain the pressure, containment degrades and bad things happen.
#[derive(Resource)]
pub struct Reactor {
    /// Plasma temperature in millions of Kelvin. Fusion-relevant range: 50-300 MK.
    pub temperature: f64,
    /// Plasma pressure in arbitrary units. Driven by temperature * density.
    pub pressure: f64,
    /// Fuel density in the chamber (0.0 - 1.0 normalized).
    pub density: f64,
    /// Magnetic confinement field strength in Tesla.
    pub field_strength: f64,
    /// Fuel injection rate (0.0 - 1.0 normalized). Higher = more fuel per tick.
    pub fuel_rate: f64,
    /// Heat removal rate from cooling systems (MW).
    pub cooling_rate: f64,
    /// Containment integrity (0.0 - 1.0). Below 0.5 triggers warnings.
    /// At 0.0 the reactor is offline and the ship takes damage.
    pub containment: f64,
    /// Energy output in GW.
    pub power_output: f64,
    /// Thermal efficiency ratio (0.0 - 1.0).
    pub efficiency: f64,
    /// Reactor operational status.
    pub status: ReactorStatus,
    /// Whether control inputs come from the automatic control loop or manual.
    pub mode: ReactorMode,
    /// Target power output in GW (used by the auto control loop).
    pub target_power: f64,
    /// RNG state for sensor noise.
    noise_phase: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactorStatus {
    Offline,
    Starting,
    Nominal,
    Warning,
    Critical,
}

impl ReactorStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Starting => "starting",
            Self::Nominal => "nominal",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "offline" => Some(Self::Offline),
            "starting" => Some(Self::Starting),
            "nominal" => Some(Self::Nominal),
            "warning" => Some(Self::Warning),
            "critical" => Some(Self::Critical),
            _ => None,
        }
    }
}

impl std::fmt::Display for ReactorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<ReactorStatus> for Value {
    fn from(s: ReactorStatus) -> Value {
        Value::String(ArcStr::from(s.as_str()))
    }
}

impl FromValue for ReactorStatus {
    fn from_value(v: Value) -> anyhow::Result<Self> {
        let s = ArcStr::from_value(v)?;
        Self::parse(&s).ok_or_else(|| anyhow::anyhow!("unknown reactor status: {s}"))
    }

    fn get(v: Value) -> Option<Self> {
        let s = ArcStr::get(v)?;
        Self::parse(&s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactorMode {
    Auto,
    Manual,
    Shutdown,
}

impl ReactorMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Manual => "manual",
            Self::Shutdown => "shutdown",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(Self::Auto),
            "manual" => Some(Self::Manual),
            "shutdown" => Some(Self::Shutdown),
            _ => None,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Auto => Self::Manual,
            Self::Manual => Self::Shutdown,
            Self::Shutdown => Self::Auto,
        }
    }
}

impl std::fmt::Display for ReactorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<ReactorMode> for Value {
    fn from(m: ReactorMode) -> Value {
        Value::String(ArcStr::from(m.as_str()))
    }
}

impl FromValue for ReactorMode {
    fn from_value(v: Value) -> anyhow::Result<Self> {
        let s = ArcStr::from_value(v)?;
        Self::parse(&s).ok_or_else(|| anyhow::anyhow!("unknown reactor mode: {s}"))
    }

    fn get(v: Value) -> Option<Self> {
        let s = ArcStr::get(v)?;
        Self::parse(&s)
    }
}

impl Default for Reactor {
    fn default() -> Self {
        Self {
            temperature: 100.0,     // 100 MK — stable operating temp
            pressure: 0.5,
            density: 0.3,
            field_strength: 8.0,    // Tesla
            fuel_rate: 0.3,
            cooling_rate: 50.0,     // MW
            containment: 1.0,
            power_output: 0.0,
            efficiency: 0.0,
            status: ReactorStatus::Nominal,
            mode: ReactorMode::Auto,
            target_power: 200.0,    // 200 GW default target
            noise_phase: 0.0,
        }
    }
}

/// Cheap deterministic noise from multiple sine waves at irrational frequencies.
/// Different seeds give different noise character.
fn sensor_noise(phase: f64, seed: f64, amplitude: f64) -> f64 {
    let n = (phase * 1.7 + seed).sin() * 0.5
        + (phase * 3.1 + seed * 2.3).sin() * 0.3
        + (phase * 7.9 + seed * 0.7).sin() * 0.2;
    n * amplitude
}

/// Higher frequency jitter that appears during degraded conditions.
/// More erratic, suggesting instability.
fn instability_noise(phase: f64, seed: f64, severity: f64) -> f64 {
    let n = (phase * 13.7 + seed).sin() * 0.4
        + (phase * 23.1 + seed * 1.1).sin() * 0.3
        + (phase * 37.3 + seed * 3.7).sin() * 0.2
        + (phase * 59.0 + seed * 0.3).sin() * 0.1;
    n * severity
}

/// Tick the reactor physics. Runs at fixed timestep.
pub fn reactor_physics(mut reactor: ResMut<Reactor>, time: Res<Time>) {
    let dt = time.delta_secs_f64();
    let r = reactor.as_mut();
    if r.status == ReactorStatus::Offline {
        return;
    }
    r.noise_phase += dt;
    // fuel injection increases density
    r.density = (r.density + r.fuel_rate * 0.1 * dt).clamp(0.0, 1.0);
    // density decays as fuel is consumed by the reaction
    let burn_rate = if r.temperature > 50.0 {
        (r.temperature / 300.0).min(1.0) * r.density * 0.15
    } else {
        0.0
    };
    r.density = (r.density - burn_rate * dt).max(0.0);
    // temperature: rises from fusion, drops from radiation (T^2) and active cooling
    let fusion_heating = burn_rate * 200.0;
    // radiation loss scales with T^2 — hot plasma radiates much faster (toy Stefan-Boltzmann)
    let radiation_loss = (r.temperature / 100.0).powi(2) * 2.0;
    let active_cooling = r.cooling_rate * 0.02;
    r.temperature = (r.temperature + (fusion_heating - radiation_loss - active_cooling) * dt)
        .max(0.0);
    // pressure is driven by temperature and density
    r.pressure = r.temperature * r.density * 0.01;
    // containment: if pressure exceeds what the field can handle, it degrades
    let field_capacity = r.field_strength;
    if r.pressure > field_capacity {
        let overpressure = (r.pressure - field_capacity) / field_capacity;
        r.containment = (r.containment - overpressure * 0.1 * dt).max(0.0);
    } else {
        r.containment = (r.containment + 0.02 * dt).min(1.0);
    }
    // energy output: proportional to burn rate and temperature, gated by containment
    r.efficiency = if r.temperature > 50.0 && r.containment > 0.1 {
        let temp_eff = ((r.temperature - 50.0) / 200.0).clamp(0.0, 1.0);
        temp_eff * r.containment
    } else {
        0.0
    };
    r.power_output = burn_rate * r.efficiency * 12000.0;
    // status
    r.status = if r.containment <= 0.0 {
        r.temperature *= 0.5;
        r.fuel_rate = 0.0;
        ReactorStatus::Offline
    } else if r.containment < 0.3 {
        ReactorStatus::Critical
    } else if r.containment < 0.7 || r.pressure > field_capacity * 0.9 {
        ReactorStatus::Warning
    } else {
        ReactorStatus::Nominal
    };
}

/// Simple automatic control loop (layer 1).
/// Adjusts fuel rate and field strength to approach target power while
/// maintaining thermal stability.
pub fn reactor_auto_control(mut reactor: ResMut<Reactor>, time: Res<Time>) {
    let dt = time.delta_secs_f64();
    let r = reactor.as_mut();
    if r.mode == ReactorMode::Manual {
        return;
    }
    // in shutdown mode, drive to zero; in auto mode, track target_power
    let target = if r.mode == ReactorMode::Shutdown { 0.0 } else { r.target_power };
    // at density equilibrium: power ≈ fuel_rate * efficiency * 1200
    // use this to compute a feedforward fuel estimate, then smooth towards it
    let eff_est = r.efficiency.max(0.05);
    let target_fuel = (target / (eff_est * 1200.0)).clamp(0.0, 1.0);
    // smooth approach — fast enough to respond, slow enough to not overshoot
    let approach_rate = 0.5; // exponential decay rate (per second)
    r.fuel_rate += (target_fuel - r.fuel_rate) * approach_rate * dt;
    // temperature safety: if approaching limit, bias fuel_rate down
    let temp_limit = 250.0;
    if r.temperature > temp_limit * 0.85 {
        let urgency = ((r.temperature - temp_limit * 0.85) / (temp_limit * 0.15)).min(1.0);
        r.fuel_rate -= urgency * 0.1 * dt;
    }
    r.fuel_rate = r.fuel_rate.clamp(0.0, 1.0);
    // field tracking: maintain a comfortable margin above pressure
    let target_margin = 1.5;
    let field_error = (r.pressure + target_margin) - r.field_strength;
    let field_adjustment = (field_error * 0.5).clamp(-1.0, 2.0);
    r.field_strength = (r.field_strength + field_adjustment * dt).max(1.0);
    // safety override
    if r.containment < 0.5 {
        r.fuel_rate *= 1.0 - 0.5 * dt;
        r.field_strength += 2.0 * dt;
    }
}

/// Publish reactor state to netidx with sensor noise.
/// Noise character varies by condition — nominal operations have gentle drift,
/// degraded conditions show higher frequency jitter and spikes.
pub fn reactor_publish(reactor: Res<Reactor>, bridge: Res<NetBridge>) {
    let base = "/ship/reactor";
    let t = reactor.noise_phase;
    // noise amplitude scales with how unstable things are
    let instability = (1.0 - reactor.containment).max(0.0);
    let base_noise = 0.002; // 0.2% baseline sensor noise
    let noisy = |val: f64, seed: f64| -> f64 {
        let n = sensor_noise(t, seed, val * base_noise)
            + instability_noise(t, seed, val * instability * 0.02);
        val + n
    };
    bridge.publish(
        Path::from(format!("{base}/plasma/temperature")),
        Value::from(noisy(reactor.temperature, 1.0)),
    );
    bridge.publish(
        Path::from(format!("{base}/plasma/pressure")),
        Value::from(noisy(reactor.pressure, 2.0)),
    );
    bridge.publish(
        Path::from(format!("{base}/plasma/density")),
        Value::from(noisy(reactor.density, 3.0)),
    );
    bridge.publish(
        Path::from(format!("{base}/plasma/field_strength")),
        Value::from(noisy(reactor.field_strength, 4.0)),
    );
    bridge.publish(
        Path::from(format!("{base}/plasma/containment")),
        Value::from(noisy(reactor.containment, 5.0)),
    );
    bridge.publish(
        Path::from(format!("{base}/output/power")),
        Value::from(noisy(reactor.power_output, 6.0)),
    );
    bridge.publish(
        Path::from(format!("{base}/output/efficiency")),
        Value::from(noisy(reactor.efficiency, 7.0)),
    );
    bridge.publish_writable(
        Path::from(format!("{base}/control/fuel_rate")),
        Value::from(reactor.fuel_rate),
    );
    bridge.publish_writable(
        Path::from(format!("{base}/control/cooling_rate")),
        Value::from(reactor.cooling_rate),
    );
    bridge.publish_writable(
        Path::from(format!("{base}/control/target_power")),
        Value::from(reactor.target_power),
    );
    bridge.publish_writable(
        Path::from(format!("{base}/control/mode")),
        Value::from(reactor.mode),
    );
    bridge.publish(
        Path::from(format!("{base}/status")),
        Value::from(reactor.status),
    );
}

/// Apply incoming write requests from netidx to reactor controls.
pub fn reactor_apply_controls(mut reactor: ResMut<Reactor>, bridge: Res<NetBridge>) {
    for write in bridge.drain_writes() {
        let path = write.path.as_ref();
        let Some(suffix) = path.strip_prefix("/ship/reactor/control/") else {
            continue;
        };
        match suffix {
            "fuel_rate" => {
                if let Some(v) = f64::get(write.value) {
                    reactor.fuel_rate = v.clamp(0.0, 1.0);
                }
            }
            "cooling_rate" => {
                if let Some(v) = f64::get(write.value) {
                    reactor.cooling_rate = v.clamp(0.0, 200.0);
                }
            }
            "target_power" => {
                if let Some(v) = f64::get(write.value) {
                    reactor.target_power = v.clamp(0.0, 1000.0);
                }
            }
            "mode" => {
                if let Some(mode) = ReactorMode::get(write.value) {
                    info!("reactor mode -> {mode}");
                    reactor.mode = mode;
                }
            }
            _ => {}
        }
    }
}

/// Keyboard controls for testing the netidx write round-trip.
/// Writes go through netidx (Bevy → bridge → netidx → WriteRequest → bridge → Bevy → Reactor).
pub fn reactor_keyboard_controls(
    keys: Res<ButtonInput<KeyCode>>,
    reactor: Res<Reactor>,
    bridge: Res<NetBridge>,
) {
    let base = "/ship/reactor/control";
    if keys.just_pressed(KeyCode::ArrowUp) {
        let new_target = (reactor.target_power + 50.0).min(1000.0);
        bridge.write(Path::from(format!("{base}/target_power")), Value::from(new_target));
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        let new_target = (reactor.target_power - 50.0).max(0.0);
        bridge.write(Path::from(format!("{base}/target_power")), Value::from(new_target));
    }
    if keys.just_pressed(KeyCode::KeyM) {
        bridge.write(
            Path::from(format!("{base}/mode")),
            Value::from(reactor.mode.next()),
        );
    }
    if reactor.mode == ReactorMode::Manual {
        if keys.just_pressed(KeyCode::ArrowRight) {
            let new_fuel = (reactor.fuel_rate + 0.05).min(1.0);
            bridge.write(Path::from(format!("{base}/fuel_rate")), Value::from(new_fuel));
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            let new_fuel = (reactor.fuel_rate - 0.05).max(0.0);
            bridge.write(Path::from(format!("{base}/fuel_rate")), Value::from(new_fuel));
        }
    }
}
