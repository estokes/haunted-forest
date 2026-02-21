# Game Architecture

How the game's technical systems work, from ship simulation to player
interface to LLM integration.


## Design Principles

The game is built as **layered simulations connected by a reactive data
fabric**. Ship systems are physics simulations. Their state is exposed
through an in-process publish/subscribe hierarchy (netidx). Interfaces
are built in a reactive dataflow language (Graphix) that subscribes to
and controls ship systems through netidx. LLMs — used judiciously —
provide key character interactions and player-triggered AI assistance.

The player interacts with the game the same way the ship's own systems
do — through netidx. There is no privileged "game UI" layer. The HUD,
engineering consoles, navigation displays, and player-written scripts
all connect to the same data hierarchy. This means anything the game
can display, the player can script.

**Testing**: This architecture enables isolated component testing.
Any system that interacts only through netidx can be tested
independently — publish test inputs, verify outputs, without running
the full game. Ship systems, Graphix programs, and control logic can
all be validated in isolation.


## Layer 1: Ship Systems Simulation

Ship systems are Bevy ECS systems running physics simulations at fixed
timesteps.

### What Exists

The D-³He fusion reactor (`src/ship/reactor.rs`) is the first
implemented system:

- **Reactor state**: temperature (MK), pressure, density, field
  strength, fuel rate, cooling rate, containment integrity, power
  output, efficiency, status, mode, target power
- **Physics**: fuel injection → fusion heating → radiation losses →
  active cooling → pressure → containment degradation. Runs in
  `FixedUpdate`.
- **Auto control**: PID-like feedback loop targeting a power setpoint.
  Adjusts fuel injection and field strength. Runs in Shutdown mode
  targeting zero power.
- **Manual control**: Player sets fuel rate and cooling rate directly.
- **Sensor noise**: Published values include deterministic noise
  (0.2% baseline, higher during degraded containment) to simulate
  imperfect instrumentation.

### Future Systems

Each system follows the same pattern — physics simulation exposing
state and accepting control through netidx:

- **Thermal management**: Tin coolant loops, endcap radiator
  temperatures, waste heat balance
- **Conventional propulsion**: Antimatter-heated hydrogen, thrust
  vectoring, fuel consumption
- **Stardrive**: Black-boxed but with observable externals — scoop
  collection rate, thrust, fuel flow
- **Navigation**: Position, velocity, course plotting, stardrive
  catch calculations
- **Power distribution**: Load balancing across ship systems,
  priority allocation
- **Life support**: Atmosphere, temperature, cold sleep pod status
- **Weapons**: Targeting, capacitor charge, ammunition
- **Sensors**: Passive and active, with range and resolution dependent
  on power allocation


## Layer 2: netidx — The Data Fabric

All ship system state is published through an in-process netidx
hierarchy. netidx is a publish/subscribe system with native support
for writable values — subscribers can write back to published paths,
enabling bidirectional control.

### The Bridge

The `NetBridge` (`src/net/bridge.rs`) connects Bevy's synchronous ECS
to netidx's async runtime:

- **Publish**: Ship systems publish state every frame at paths like
  `/ship/reactor/plasma/temperature`
- **Publish writable**: Control inputs are published as writable
  values at paths like `/ship/reactor/control/fuel_rate`. External
  subscribers (Graphix programs, the player) can write to these.
- **Subscribe**: UI systems subscribe to paths and receive updates
  via `drain_updates()`
- **Write round-trip**: Writes from subscribers arrive as
  `WriteEvent`s via `drain_writes()`, applied by the simulation
  next frame

The bridge runs a tokio runtime on a background thread. Communication
uses crossbeam channels. Publisher updates are batched on 16ms
intervals.

### The Hierarchy

The netidx path hierarchy mirrors the ship's physical structure:

```
/ship/
    reactor/
        plasma/
            temperature     (f64, MK, read-only)
            pressure        (f64, read-only)
            density         (f64, read-only)
            field_strength  (f64, T, read-only)
            containment     (f64, 0-1, read-only)
        output/
            power           (f64, GW, read-only)
            efficiency      (f64, 0-1, read-only)
        control/
            fuel_rate       (f64, 0-1, writable)
            cooling_rate    (f64, MW, writable)
            target_power    (f64, GW, writable)
            mode            (ReactorMode, writable)
        status              (ReactorStatus, read-only)
    thermal/
        ...
    propulsion/
        ...
    navigation/
        ...
    sensors/
        ...
    weapons/
        ...
```

Every value in this hierarchy is observable and (where writable)
controllable from Graphix, from LLM-driven agents, or from the
player's own scripts. The game's built-in interfaces use the same
paths as player-written code — no hidden APIs.


## Layer 3: Graphix — Reactive Interfaces and Scripting

Graphix is a reactive dataflow language for stream processing and UIs.
It integrates netidx natively — subscribing to published values and
writing to writable ones are first-class operations. Full source:
`~/proj/graphix/`.

### Runtime Integration

Graphix runs as a tokio task on the same async runtime as the
netbridge. The integration follows the `graphix-shell` pattern
(see the `graphix-shell` crate):

- `GXRt::new(publisher, subscriber)` takes netidx pub/sub at init
- The runtime spawns via `tokio::spawn()` as a background task
- Events flow back through `mpsc::Sender<GPooled<Vec<GXEvent>>>`
- The game's main loop demultiplexes Graphix events alongside netidx
  updates in its `tokio::select!`

Since both Graphix and the netbridge live on the same tokio runtime,
they share the publisher and subscriber instances. Graphix programs
subscribe to and write netidx paths directly — no bridge hop needed
for Graphix-to-netidx communication.

### Role in the Game

Graphix serves three purposes:

**1. All in-game interfaces.** Every display the player sees — the
HUD, engineering consoles, navigation charts, trade terminals, damage
reports — is a Graphix program subscribing to netidx paths and
rendering the results. The current HUD (`src/ship/hud.rs`) is a
transitional Rust implementation; the target is Graphix-based UI
throughout.

**2. Player scripting.** The player can write Graphix programs to
automate and optimize ship operations:

- Custom dashboards: subscribe to the values you care about, arrange
  them how you want
- Automation: "if reactor containment drops below 0.8, set mode to
  Shutdown" — written as a reactive dataflow, not an imperative script
- Analysis tools: aggregate sensor data, compute moving averages,
  detect anomalies
- Tactical programs: trajectory plotting, threat assessment, combat
  automation — subscribe to sensor feeds, compute intercepts, write
  commands to weapons and propulsion

**3. Market and trade tools.** Subscribe to market data feeds (also
exposed via netidx), build price models, evaluate contracts, simulate
trade routes.

### Why Graphix

A reactive dataflow language is the right model for this because ship
systems ARE dataflows. Values change continuously. The player's
programs react to those changes. This is not a scripting language
bolted onto a game — it's the same computational model the game itself
uses.

The netidx integration means player scripts have exactly the same
access and capabilities as the game's own systems. No artificial
limitations, no sandboxed "mod API." If a value is published, you can
read it. If it's writable, you can control it.


## Layer 4: LLM Integration

LLMs provide key character interactions and player-triggered AI
assistance. They are used **judiciously** — local hardware can run one
small model (Phi4-class) at a reasonable token rate on CPU. This is a
scarce resource, not an unlimited service.

### Architecture

A background thread owns the LLM context (`LlamaBackend`, `Model`,
`Context` from the `llama-cpp-2` crate). Communication with the game
uses crossbeam channels for token streaming — the same async pattern
as the netidx bridge.

```
Game thread                    LLM thread
    │                              │
    ├── prompt request ──────────► │
    │                              ├── token stream
    │ ◄──────── token ─────────── │
    │ ◄──────── token ─────────── │
    │ ◄──────── token ─────────── │
    │ ◄──────── complete ──────── │
    │                              │
```

One local model instance. Requests are queued and serialized.

### What Uses an LLM

LLMs are reserved for interactions that benefit from genuine language
understanding and generation:

**Complex character interactions**: When a conversation goes beyond
what a dialogue tree can handle — negotiation with a faction leader,
interrogating a prisoner, debating the Warden. Each character has a
system prompt defining personality, knowledge, and motivations.

**Player-triggered shipboard AI queries**: The player asks the ship AI
a direct question — "what's our fuel situation if we divert to Tau
Ceti?", "analyze this sensor anomaly", "help me write a Graphix
program to monitor containment." These are explicit requests, not
ambient NPC chatter.

**The Warden**: The final confrontation is a conversation. This is
the highest-stakes LLM interaction in the game.

### What Does NOT Use an LLM

Everything else is traditional game systems:

- **Routine NPC interactions**: Docking controllers, merchants,
  generic crew dialogue — scripted or template-based with variable
  substitution from game state.
- **Combat AI**: Deterministic or procedural. Enemy behavior is
  algorithmic, not LLM-driven.
- **Market simulation**: Mathematical models, not language models.
  Price curves, supply/demand, contract generation — all code.
- **Ship system automation**: Graphix programs, PID loops, rule-based
  control. The reactor's auto-control is a good example.
- **Ambient world behavior**: Colony events, faction movements, trade
  network dynamics — simulation, not generation.

### External API for Code Generation

Graphix code generation — helping the player write reactive dataflow
programs — benefits from a larger model than can run locally. A
Phi4-class model can handle simple queries; complex Graphix programs
need something more capable.

Options under consideration:
- **API calls to a capable model** (Claude, etc.) for Graphix code
  generation. This costs real money per query. The game may need a
  subscription model to cover API costs.
- **Local-only with limitations**: Ship the game with no API
  dependency. The local model handles what it can; the player writes
  the rest manually. Less magical, but self-contained.
- **Hybrid**: Local model for simple assistance, API for complex
  generation. Player opts in to API usage and pays for it.

This is an open business model question. The game design should work
without API access — LLM code generation is a convenience, not a
requirement. A skilled player can write Graphix directly.


## Time Dilation Mechanic

The game simulation rate is adjustable based on character type. This
is the mechanical expression of computational superiority — not a
stat bonus, but a change in how the player experiences time-critical
situations.

| Character | Min speed | Duration limit |
|-----------|----------|----------------|
| Baseline human | 1× (real time) | — |
| Augmented human | 1/2× | ~2-3 minutes before thermal buildup in neural implants forces return to real time |
| Heavily augmented | 1/10× | ~5-10 minutes before thermal limits. Longer than augmented but still constrained by biology. |
| AI person | 1/100× | No limit. Pure computation, limited only by power and cooling — which the ship has in abundance. |

### What Slow-Time Enables

At real time, the game presents the same information to every
character type. But much of it scrolls by too fast to process — sensor
data, trajectory updates, market fluctuations, threat assessments.

At slower speeds, the player has time to:

- **Read and analyze** the full sensor picture — data that was always
  there but moving too fast for biological processing
- **Write Graphix programs** to automate responses, build tactical
  tools, create analysis dashboards — during combat, not before it
- **Consult LLMs** for strategy analysis, code generation, outcome
  modeling — ask the ship AI "what's our best evasion vector?" and
  get an answer before the missile arrives
- **Run simulations** — model different maneuvers, test Graphix
  programs against predicted outcomes, iterate on tactics
- **Coordinate** — issue detailed commands to crew, remotes, and
  automated systems with precision impossible in real time

The tools are available at every speed. The question is whether you
have time to use them.

### Thermal Limits for Biologicals

Augmented and heavily augmented characters achieve slow-time through
accelerated neural processing driven by their implants. This generates
waste heat in the brain — the same thermodynamic problem the ship's
reactor faces, at a much smaller and more delicate scale.

After a few minutes of accelerated cognition, biological tissue
approaches thermal damage thresholds. The implants force a return to
normal processing speed. A cooldown period is required before
acceleration is available again. Pushing past the thermal limit risks
permanent neurological damage.

The AI player has no such constraint. Their substrate is the ship's
compute infrastructure, cooled by the same liquid tin loops that cool
the reactor. They can stay in slow-time indefinitely. This is their
primary combat advantage — not that they think faster for a burst,
but that they can sustain it.


## Integration Model

Everything connects through netidx:

```
┌─────────────────────────────────────────────────┐
│                   netidx hierarchy               │
│                                                   │
│  /ship/reactor/...  /ship/nav/...  /market/...   │
└──────┬──────────────────┬──────────────┬─────────┘
       │                  │              │
  ┌────▼────┐      ┌──────▼─────┐  ┌────▼────────┐
  │  Bevy   │      │  Graphix   │  │    LLM      │
  │ physics │      │  programs  │  │  (1 local   │
  │ systems │      │ (UI/scripts│  │   instance) │
  └─────────┘      └────────────┘  └─────────────┘
       ▲                  │              │
       │        writes    │    reads     │
       └──────────────────┘──────────────┘

         tokio runtime (shared)
         ├── netbridge task
         ├── graphix runtime task
         └── publisher batch task

         LLM thread (separate, owns llama context)
```

The physics simulations publish state. Graphix programs subscribe to
that state and write control inputs. The LLM reads game state when
prompted and generates responses. The player writes Graphix programs;
the LLM can help write them.

There is no distinction between "game code" and "player code" at the
netidx level. The game's own auto-control systems (like the reactor's
PID loop) could be reimplemented by the player in Graphix. A
sufficiently motivated player could replace every built-in control
system with their own.


## Current Implementation Status

| Component | Status |
|-----------|--------|
| Bevy ECS framework | Running |
| NetBridge (publish, subscribe, writable, write) | Implemented |
| Reactor physics simulation | Implemented |
| Reactor auto-control (PID loop) | Implemented |
| Reactor keyboard controls (test) | Implemented |
| HUD (Rust, transitional) | Implemented |
| Graphix integration | Not started |
| LLM integration | Not started |
| Time dilation mechanic | Not started |
| Other ship systems (thermal, propulsion, nav) | Not started |
| Market/trade simulation | Not started |


## Open Questions

- ~~Graphix runtime embedding~~ → tokio task on the netbridge side,
  following the `graphix-shell` pattern
- LLM context management — how much game state goes into the context
  window? Ship state is enormous. Need a summarization/relevance
  filtering layer.
- LLM request prioritization — with one local model instance, how do
  we queue and prioritize competing requests? Player queries should
  preempt background NPC generation.
- Security model for player Graphix programs — can a player script
  crash the game? Should there be resource limits (CPU time, memory)
  on player programs?
- How are Graphix UIs rendered? Overlay on the 3D scene? Separate
  panels? In-universe screens on the bridge?
- NPC memory — do LLM-driven characters remember previous
  conversations across sessions? How is conversation history stored?
- Business model — if API-based code generation is offered, how is
  it priced? Subscription? Per-query? Bundled with game purchase?
- Multiplayer implications — if two players are in the same system,
  do their netidx hierarchies interact? Can one player subscribe to
  another ship's published (non-classified) values?
