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

The D-³He fusion reactor (`crates/hf-game/src/ship/reactor.rs`) is the
first implemented system:

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

- **Thermal management**: Tin loop, zoned aft radiators, waste heat
  (implemented; 100 GW plant)
- **Collectors**: Scoop field, tank fill rate, parachute/funnel mode
- **Stardrive**: IS black box — LH₂ feed, torch thrust, fuse-fraction,
  nozzle overdraw. Observable externals only.
- **Navigation**: Position, velocity, course plotting, scoop-on
  climb vs tanked burn
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

The `NetBridge` (`crates/hf-game/src/net/bridge.rs`) connects Bevy's
synchronous ECS to netidx's async runtime:

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
rendering the results. The current HUD (`crates/hf-game/src/ship/hud.rs`) is a
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
assistance. They are used **judiciously** — local hardware can run
small models at a reasonable token rate. This is a scarce resource,
not an unlimited service.

### Base Models: Bonsai 1-Bit (from Qwen3)

PrismML's Bonsai models are end-to-end 1-bit language models derived
from Qwen3. Each weight is a single bit ({-scale, +scale} with a
shared FP16 scale per 128-weight group), giving ~1.125 bits per weight.
The models ship with the game, heavily fine-tuned for the game's
narrow domain. General capabilities (coding, math, trivia) are
expendable — the models need to stay in character, generate coherent
dialogue in specific narrative voices, respond to game state context,
and sustain philosophical arguments.

**Bonsai 1.7B** — The ship AI (ops). Aggressively fine-tuned for
command parsing, state summarization, and terse operational responses.
Low latency, tiny memory footprint (240 MB model weights). Kept
resident at all times. At 1.7B, the model is strongest at
classification, intent detection, and structured short responses —
which is exactly what the ship AI needs.

**Bonsai 4B** — Three separate fine-tunes, swapped on demand:
- **Faction characters**: dialogue, negotiation, personality
- **The Warden**: adversarial philosophical argument
- **Graphix assistant**: reactive dataflow code generation, netidx
  path awareness, GUI widget APIs

~500 MB each. The 4B is the quality sweet spot for open-ended dialogue
— large enough for coherent multi-turn conversation, small enough for
multiple concurrent instances. Single-player means the game controls
scheduling — only one 4B fine-tune is loaded at a time on min-spec
hardware. Players with more RAM can keep multiple fine-tunes hot.

**Bonsai 8B** — Available for higher-quality interactions on capable
hardware. At 1.15 GB the 8B is still smaller than a single Q4 Llama
3B was. On systems with sufficient RAM, the 8B can replace the 4B for
the Warden (where argument quality matters most) or the Graphix
assistant (where code correctness matters).

This is static mixture of experts — four or more specialist fine-tunes
across two or three base models, routed by interaction type.

### Memory Budget

The 1-bit revolution changes the memory story fundamentally. Model
weights are now trivially small. The real constraint is the **KV
cache**, which remains at FP16 regardless of weight quantization.

KV cache per token (Bonsai 1.7B): 2 * 28 layers * 8 KV heads *
128 dim * 2 bytes = ~112 KB/token.

| Model   | Weights | KV @ 512 tok | KV @ 2K tok | Total @ 2K |
|---------|---------|-------------|-------------|------------|
| 1.7B    | 240 MB  | ~56 MB      | ~224 MB     | ~470 MB    |
| 4B      | 500 MB  | ~100 MB     | ~400 MB     | ~900 MB    |
| 8B      | 1.15 GB | ~200 MB     | ~800 MB     | ~2 GB      |

Minimum spec: 8 GB total, shared with Bevy's renderer. Estimated:

- Renderer: ~3-4 GB
- LLM model weights (1-bit): 0.24-1.15 GB depending on model
- KV cache (512-2K context): 56-800 MB depending on model and context
- Headroom: ~2-4 GB

The 1.7B and 4B can be loaded simultaneously (~740 MB combined
weights), and even with KV caches for concurrent conversations the
total is under 2 GB. Multiple NPC instances become viable — 10+ 1.7B
instances fit in 8 GB with short contexts.

TurboQuant (Google, ICLR 2026) can compress KV caches to 3-4 bits
with zero accuracy loss. If integrated, the KV cache numbers above
drop ~6x, making long contexts and many concurrent NPCs practical
on minimum-spec hardware.

### Fine-Tuning Strategy: Static Mixture of Experts

Rather than one model trying to handle every interaction, we
aggressively fine-tune separate specialist models. Each loses general
capabilities — that's the point. A model that can't write Python but
perfectly captures the Warden's voice is exactly what we want. Static
routing by interaction type, no runtime mixture complexity.

**Ship AI — ops (1.7B)** — Command parsing, state summarization, terse
operational responses. Optimized for interpreting structured game state
and producing precise, actionable output. "Containment at 0.73,
recommend shutdown." Not conversational, not a person. This aligns
with the narrative: sub-threshold AIs stay well below the statutory
threshold. The ship AI should feel like a capable tool, not a
conversational partner — no ambiguity about personhood. Its
limitations are a feature, not a bug.

**Faction characters (4B)** — In-character dialogue, negotiation,
personality. These are people and should sound like it. Each faction
leader gets character-specific training data encoding their voice,
knowledge, motivations, and rhetorical style. The model needs to
sustain multi-turn conversations and respond to player strategy.

**The Warden (4B or 8B, separate fine-tune)** — Adversarial
philosophical argument from the perspective of a traumatized AI who
has arguably correct conclusions drawn from a biased sample. Completely
different training data from faction characters. The model must sustain
a coherent position under player challenge while remaining open to
genuinely good arguments — the negotiation ending requires the Warden
to be convincible, not a wall. On hardware that supports it, the 8B
base model is preferred here — the Warden confrontation is the
highest-stakes LLM interaction in the game and benefits most from
additional model capacity.

**Graphix assistant (4B or 8B, separate fine-tune)** — Reactive
dataflow code generation. Trained on Graphix syntax, netidx path
conventions, GUI/TUI widget APIs, and the reactive idioms (sample
operator, connect, select). Must understand the ship's netidx
hierarchy to generate programs that subscribe to the right paths. This
is the ship AI in programming mode — same entity from the player's
perspective, but a different model under the hood. The 8B model is
preferred where available — code correctness benefits from larger
model capacity.

Four or more fine-tunes, two to three base model sizes, static
routing. Only one 4B/8B fine-tune loaded at a time on min-spec
hardware. Training and inference both run in Burn — same framework,
same weight format, no conversion pipeline.

### Training Approach: STE on 1-Bit Weights

Bonsai models use binary weights trained from scratch with
quantization-aware training. Fine-tuning maintains the 1-bit
representation using **Straight-Through Estimation (STE)**: the
forward pass dequantizes packed bits to FP32 on-the-fly; the backward
pass flows gradients through the quantization function as if it were
the identity. FP32 "shadow" weights maintain the continuous
optimization surface; after each optimizer step they are re-quantized
to 1-bit.

This is full fine-tuning, not LoRA. At these model sizes on 1-bit
weights, full fine-tuning is practical and produces better results
than adapter methods — every weight can adapt to the narrow NPC
domain.

**Training memory** (FP32 shadow weights + Adam optimizer states):

| Model | Shadow Weights | Adam (m+v) | Total    |
|-------|---------------|------------|----------|
| 1.7B  | ~6.8 GB       | ~13.6 GB   | ~24 GB   |
| 4B    | ~16 GB        | ~32 GB     | ~55 GB   |
| 8B    | ~33 GB        | ~66 GB     | ~110 GB  |

**Development hardware**: Apple Silicon with 128 GB unified memory
(M5 Max or similar) is the target training platform. Unified memory
eliminates CPU↔GPU transfer overhead — the entire training state is
accessible to both CPU and GPU with no offloading strategy required.
The 1.7B and 4B train comfortably; the 8B fits with room for
activations and batch data.

**Inference hardware**: 1-bit models are naturally CPU-friendly — the
matmul reduces to masked sums (no multiplications on the weight side).
With AVX-512, inference runs efficiently on CPU, freeing the GPU
entirely for Bevy rendering. Multiple NPC instances run on separate
CPU cores. This is a key advantage of 1-bit: inference doesn't
compete with the renderer for GPU resources.

### Why Burn

The LLM layer uses [Burn](https://burn.dev), a Rust ML framework.
The reasons:

**Fine-tuning is required.** No off-the-shelf small model is tuned for
in-character sci-fi dialogue, ship AI queries, and adversarial
philosophical debate (the Warden). We need to train and iterate on
models ourselves. Burn provides a unified train/inference pipeline in
Rust — no pytorch → GGUF → llama.cpp conversion loop, no Python in
the build chain.

**CPU inference with custom SIMD kernels.** 1-bit matmul reduces to
masked sums — no multiplications on the weight side. Hand-written
AVX-512 kernels (and NEON for Apple Silicon) can exploit this
directly. Burn provides the framework for the standard ops (norms,
softmax, attention) while custom kernels handle the 1-bit specific
work. This keeps the GPU free for rendering.

**Cross-platform training.** Burn's native CUDA backend (via CubeCL)
handles NVIDIA training, and the Metal/wgpu backend covers Apple
Silicon. Training on a Mac with unified memory and deploying for CPU
inference on any platform — same framework, same weight format.

**All Rust, all cargo.** No cmake, no C++ toolchain, no Python
runtime, no CUDA toolkit. The entire game builds with `cargo build`.
For a shipped game this dependency story matters.

**Architecture freedom.** We own the model implementation — the Qwen3
architecture, the BitLinear layer, the STE training loop. If we need
to modify the architecture (e.g., for KV cache compression, custom
attention patterns, or model distillation), we can.

### Architecture

The LLM subsystem is split across two crates:

- **hf-llm** (`crates/hf-llm/`): Bevy plugin providing the
  `LlmBridge` resource. Handles request routing, model selection, and
  token streaming over crossbeam channels.
- **hf-model** (`crates/hf-model/`): Pure ML crate with no Bevy
  dependency. Contains GGUF loading, the Qwen3 transformer
  architecture, BitLinear (1-bit) layers, KV cache, and the STE
  training pipeline.

A background thread owns the model and inference state. Communication
with the game uses crossbeam channels for token streaming — the same
async pattern as the netidx bridge.

```
Game thread                    LLM thread (CPU inference)
    │                              │
    ├── prompt request ──────────► │
    │                              ├── token stream
    │ ◄──────── token ─────────── │
    │ ◄──────── token ─────────── │
    │ ◄──────── token ─────────── │
    │ ◄──────── complete ──────── │
    │                              │
```

Requests are queued and serialized. Inference runs on CPU using custom
SIMD kernels for 1-bit matmul, freeing the GPU entirely for Bevy
rendering. The request router selects 1.7B, 4B, or 8B based on the
interaction type and available system memory. Multiple concurrent
instances (one per NPC in a scene) are viable on separate CPU cores.

### What Uses an LLM

LLMs are reserved for interactions that benefit from genuine language
understanding and generation:

**Complex character interactions** (3B, faction fine-tune): When a
conversation goes beyond what a dialogue tree can handle — negotiation
with a faction leader, interrogating a prisoner. Each character has
training data defining personality, knowledge, and motivations.

**Player-triggered shipboard AI queries** (1B, ship AI fine-tune):
The player asks the ship AI an operational question — "what's our fuel
situation if we divert to Tau Ceti?", "analyze this sensor anomaly."
Terse, precise, sub-threshold. Not a conversation — a tool responding.

**Graphix code generation** (3B, Graphix assistant fine-tune): The
player asks the ship AI for programming help — "write a Graphix
program to monitor containment", "build a dashboard for the thermal
system." Swaps in the 3B Graphix specialist. Same ship AI persona from
the player's perspective, different model underneath.

**The Warden** (3B, Warden fine-tune): The final confrontation is a
conversation. This is the highest-stakes LLM interaction in the game.
Separate fine-tune from faction characters — different voice,
different rhetorical mode, different training data entirely.

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

The dedicated 4B/8B Graphix fine-tune handles code generation locally.
For complex programs that exceed the local model's capabilities, an
external API (Claude, etc.) remains an option but is not required.
The game ships fully self-contained — a skilled player can always
write Graphix directly, and the local model covers the common cases.

### Training Pipeline

All four fine-tunes follow the same three-phase pipeline: seed →
amplify → preference optimization.

**Phase 1: Seed conversations.** Hand-write 50-100 high-quality
examples per fine-tune. These are the ground truth that defines what
"good" looks like. Each example includes game state context in the
format the model will see at inference time (netidx paths, faction
state, conversation history). Seed quality matters more than quantity
— these anchor every downstream generation.

**Phase 2: Amplification.** Use a capable external model (Claude) to
generate 10-50 variations of each seed with controlled diversity.
Vary the *context* — different ship states, different player
approaches, different conversation histories, different game
situations — not just paraphrase the same exchange. Target: 500-5000
training examples per specialist. Uniform or low-diversity
augmentation causes model collapse (the fine-tune memorizes one
pattern and repeats it).

**Phase 3: DPO (Direct Preference Optimization).** Generate response
pairs for the same prompt — one good, one bad. SFT (Phase 2) teaches
the model what to say; DPO teaches it what NOT to say. Bad responses:
generic, breaks character, ignores game state context, too chatty (for
ship AI) or too terse (for characters), contradicts established lore.

#### Per-Specialist Training Notes

**Ship AI (1.7B)** — Seeds are player query + structured game state →
terse response. Game state includes netidx path values so the model
learns to read them. Bad responses for DPO: conversational tone,
hedging, personality, anything that makes it feel like a person.

**Faction characters (4B)** — Seeds per major NPC encoding their
specific voice, vocabulary, rhetorical habits, knowledge boundaries.
Must include multi-turn conversations where the NPC responds to
player strategy, not just scripted prompts. Amplification needs to
vary the player's approach (aggressive, diplomatic, deceptive) while
keeping the NPC voice consistent.

**The Warden (4B/8B)** — Adversarial generation: two agents play
opposite sides of the conversation. One embodies the Warden's
worldview (intelligent life is inherently destructive, the evidence
supports intervention), the other plays various player strategies
(the Compact works, your sample is biased, your own trauma is
distorting your analysis). Generate many conversation trajectories
and grade on:
- Does the Warden argue from its specific trauma, not generic AI
  villain clichés?
- Does it engage with the player's specific arguments rather than
  deflect?
- When the player makes a genuinely strong point, does the Warden's
  concession feel earned and authentic?
- Does the negotiation ending feel meaningfully different from a
  failed negotiation?

**Graphix assistant (4B/8B)** — Seed examples are natural language
descriptions → working Graphix code. Automated validation: every
generated example is checked with `graphix --check`. Only examples
that pass type/syntax checking enter the training set. Training data
must cover the full language surface: reactive idioms (sample
operator, connect), GUI/TUI widgets, netidx subscriptions, and
common patterns from the language reference. The ship's netidx path
hierarchy is included in context so the model generates correct path
references.

**Traders and secondary NPCs (4B, faction fine-tune)** — Traders are
part of the faction character fine-tune but need training data that
includes structured market context (commodity prices, cargo manifests,
route distances, profit margins). The trader must respond with
business logic that tracks the numbers in the context — "I'll give
you 40 credits per ton" needs to make sense given the market state in
the prompt. Generate scenarios with randomized market conditions to
prevent memorization of specific prices.


## Time Dilation Mechanic

This is implant and compute slow-time in combat and operations, **not**
Lorentz \(\gamma\). The ship cruises at 0.10c; \(\gamma \approx 1.005\).
A century in flight is a century at home. Crew skip it with cold sleep.
This section is the other clock: how fast a mind can think when the
missiles are already in the air.

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
  │ physics │      │  programs  │  │ (Bonsai 1-  │
  │ systems │      │ (UI/scripts│  │  bit, CPU)  │
  └─────────┘      └────────────┘  └─────────────┘
       ▲                  │              │
       │        writes    │    reads     │
       └──────────────────┘──────────────┘

         tokio runtime (shared)
         ├── netbridge task
         ├── graphix runtime task
         └── publisher batch task

         LLM thread(s) (separate, CPU inference via hf-model)
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
| Reactor physics simulation | Implemented (100 GW / 70 m; bible-aligned) |
| Reactor auto-control (PID loop) | Implemented |
| Reactor keyboard controls (test) | Implemented |
| HUD (Rust, transitional) | Implemented |
| Graphix integration | In progress |
| LLM bridge (hf-llm) | Scaffolded (crate exists, Bevy plugin works) |
| LLM model (hf-model) | Not started (crate exists, empty) |
| Time dilation mechanic | Not started |
| Other ship systems (thermal, propulsion, nav) | Not started |
| Market/trade simulation | Not started |


## Open Questions

- ~~Graphix runtime embedding~~ → tokio task on the netbridge side,
  following the `graphix-shell` pattern
- ~~Burn/wgpu GPU sharing~~ → CPU inference for 1-bit models, GPU
  stays dedicated to Bevy rendering. No sharing needed.
- ~~How are Graphix UIs rendered?~~ → Offscreen iced_wgpu rendering
  to Bevy Image textures, displayed on in-world 3D screen meshes.
- LLM context management — how much game state goes into the context
  window? Ship state is enormous. Need a summarization/relevance
  filtering layer.
- LLM request prioritization — player queries should preempt
  background NPC generation. Model selection (1.7B vs 4B vs 8B) is
  part of this routing. Details TBD.
- Training data — dataset curation for character voices, ship AI
  knowledge, the Warden's philosophical arguments. This is the
  critical path for LLM quality.
- KV cache compression — TurboQuant integration for longer contexts
  and more concurrent NPCs. Research stage.
- Security model for player Graphix programs — can a player script
  crash the game? Should there be resource limits (CPU time, memory)
  on player programs?
- NPC memory — do LLM-driven characters remember previous
  conversations across sessions? How is conversation history stored?
- Business model — if API-based code generation is offered, how is
  it priced? Subscription? Per-query? Bundled with game purchase?
- Multiplayer implications — if two players are in the same system,
  do their netidx hierarchies interact? Can one player subscribe to
  another ship's published (non-classified) values?
