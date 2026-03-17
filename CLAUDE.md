# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Haunted Forest is a sci-fi space game built with Bevy (0.18) and Rust (edition 2024). The player operates a starship through layered simulations connected by a reactive data fabric. Ship systems are physics simulations whose state is exposed through netidx (in-process pub/sub), with interfaces built in Graphix (a reactive dataflow language). There is no privileged game UI layer — everything connects through the same netidx hierarchy.

## Build & Run

```bash
cargo build          # build (uses opt-level="s" + thin LTO in dev)
cargo run            # run the game
cargo check          # type-check without building
cargo clippy         # lint
cargo test           # run tests
cargo test <name>    # run a single test by name
```

**Path dependencies**: The project depends on local checkouts of `netidx` (at `../netidx/`) and `graphix` (at `../graphix/`). These must be present for compilation. The path deps must match graphix's workspace to ensure type identity across crates.

## Architecture

### Four-Layer Design

1. **Ship Systems (Bevy ECS)** — Physics simulations running at `FixedUpdate`. Each system publishes state and accepts control through netidx. The reactor (`src/ship/reactor.rs`) is the first implemented system.

2. **netidx (Data Fabric)** — In-process pub/sub hierarchy at paths like `/ship/reactor/plasma/temperature`. Values can be read-only or writable (bidirectional control). The `NetBridge` (`src/net/bridge.rs`) connects Bevy's sync ECS to netidx's async runtime via crossbeam channels.

3. **Graphix (Reactive UI/Scripting)** — Reactive dataflow language for building interfaces and player scripts. Runs as a tokio task sharing the same runtime and netidx pub/sub as the NetBridge. Integration in `src/graphix/`.

4. **LLM (Character Interactions)** — Not yet implemented. Will use Burn (Rust ML framework) with wgpu backend for local model inference and fine-tuning. Runs on a separate thread with crossbeam channel token streaming.

### Key Resource Flow

`NetPlugin` must be added before `GraphixPlugin` (Graphix needs the `NetInfra` resource). Plugin order in `main.rs` matters.

```
NetPlugin  →  inserts NetInfra + NetBridge resources
GraphixPlugin  →  reads NetInfra, inserts GraphixRuntime
PlayerPlugin  →  first-person camera + WASD movement
ShipPlugin  →  reactor simulation, HUD, keyboard controls
```

### Async Bridge Pattern

The project bridges Bevy (sync, single-threaded ECS) to tokio (async) using:
- A tokio `Runtime` owned by `NetInfra` (lives as a Bevy `Resource`)
- `crossbeam_channel` for sync↔async communication
- The bridge task polls commands from Bevy every 16ms and batches publisher updates
- Graphix uses a tokio mpsc → crossbeam bridge for events back to Bevy

### netidx Path Convention

Ship state is published under `/ship/<system>/<subsystem>/<value>`. Writable control paths go under `.../control/`. The hierarchy mirrors the ship's physical structure. See `book/architecture.md` for the full path tree.

## Source Layout

- `src/main.rs` — App setup, plugin registration, `GameState` enum
- `src/net/` — `NetInfra` (tokio runtime + netidx), `NetBridge` (pub/sub/write bridge)
- `src/graphix/` — Graphix runtime integration, stdlib package registration
- `src/ship/` — Ship systems: `reactor.rs` (physics + auto-control + publish), `bridge.rs` (3D bridge room), `hud.rs` (reactor HUD via netidx subscriptions)
- `src/player/` — First-person controller (camera, movement, cursor grab)
- `book/` — World bible and design docs (setting, architecture, ship systems, narrative)

## World & Design Docs

The `book/` directory contains the game's world bible and technical design docs. `book/architecture.md` is the authoritative reference for the game's technical design — read it before making architectural decisions. `book/ship-systems.md` covers the physics and engineering that the simulations model.

## Writing Graphix Code — Language Reference

Graphix is NOT in the training set. This section is the authoritative
reference for writing `.gx` files. Read the full docs in `book/src/`
and examples in `book/src/examples/` when you need more detail.

Type and syntax check a Graphix program: `graphix --check $file`

### Basics

Expression-oriented: everything evaluates to a value. The last
expression in a file or block is its value. Statements end with `;`
inside blocks.

```graphix
// line comments
/// doc comments (only in .gxi interface files, before val/type/mod)

// let bindings
let x = 42
let x: i64 = 42                  // optional type annotation
let (a, b) = (1, 2)              // destructuring
let {x, y} = point               // struct destructuring
let rec f = |n| ...               // recursive binding

// blocks — create scope, evaluate to last expr
let result = {
  let tmp = compute();
  tmp + 1
}

// semicolons separate exprs in blocks; last expr has no semicolon
```

### Types

Structural typing — two types with the same shape are the same type.

```graphix
// primitives
bool  string  bytes  null
i8 i16 i32 i64  u8 u16 u32 u64  f32 f64  decimal
datetime  duration
v32 v64  z32 z64                  // variable-width integers

// composite
Array<i64>                        // array
Map<string, i64>                  // map
(i64, string)                     // tuple (2+ elements)
{x: f64, y: f64}                 // struct
`Tag | `Tag(i64, string)          // variant (backtick prefix)
[i64, string]                     // union/set type (either)
[i64, null]                       // option type (value or null)
Error<`MyErr>                     // error
&i64                              // reference
fn(i64) -> string                 // function
fn(i64) -> string throws `E      // function that throws

// type aliases
type Point = {x: f64, y: f64}
type Maybe<'a> = ['a, null]
type List<'a> = [`Cons('a, List<'a>), `Nil]   // recursive

// type variables: 'a, 'b, etc.
// constraints: 'a: Number, 'a: Int, 'a: Float
// type sets: Number, Int, SInt, UInt, Float, Real
```

### Literals

```graphix
42  3.14  true  false  null
"hello [name]!"                   // string interpolation with []
"escape \[ \] \n \t \\ \""       // escaped brackets, standard escapes
r'raw string, only \\ and \' '   // raw string (single quotes)
[1, 2, 3]                        // array
{"a" => 1, "b" => 2}             // map
(1, "two", 3.0)                  // tuple
{x: 10, y: 20}                   // struct
`Foo  `Bar(42)  `Baz("hi", 3)   // variants
datetime:"2020-01-01T00:00:00Z"
duration:1.0s  duration:500.ms  duration:100.ns
```

### Operators (by precedence, highest first)

```
*  /  %                           // multiply, divide, modulo
+  -                              // add, subtract
<  >  <=  >=                      // comparison
==  !=                            // equality
&&                                // logical and
||                                // logical or
~                                 // sample (lowest binary)
```

Unary: `!x` (not), `&x` (reference), `*x` (dereference)
Postfix: `x?` (propagate error), `x$` (error→never, logs warning)

All binary operators are left-associative.

### Access & Indexing

```graphix
s.field                           // struct field
t.0  t.1                         // tuple index
a[i]  a[-1]                      // array index (negative from end)
a[2..]  a[..4]  a[1..3]          // array slice (end exclusive)
m{"key"}                          // map access (returns Result)
module::name                      // module path
```

### Functions

```graphix
// lambda syntax: |args| body
let f = |x| x + 1
let g = |x, y| x + y
let h = |x: i64, y: i64| -> i64 x + y

// polymorphic with constraints
let add = 'a: Number |x: 'a, y: 'a| -> 'a x + y

// labeled args (# prefix) — go before positional args at call site
// if no default is provided then the labeled arg isn't optional.
let greet = |#greeting = "hello", name| "[greeting], [name]!"
greet(#greeting: "hi", "world")   // "hi, world!"
greet("world")                    // "hello, world!" (default used)

// variadic args (only usable by built-ins)
let f = |@args: i64| args         // args is Array<i64>

// calling
f(1)  g(1, 2)  module::func(x)
```

### Select — Pattern Matching (only control flow construct)

```graphix
select expr {
  pattern => result,
  pattern if guard => result,     // guard condition
  _ => default                    // wildcard
}

// type matching
select x {
  i64 as n => n + 1,
  string as s => str::len(s),
  null as _ => 0
}

// variant matching
select food {
  `Apple => "fruit",
  `Carrot => "vegetable",
  `Other(name) => name
}

// destructuring
select pair {
  (0, y) => y,
  (x, 0) => x,
  (x, y) => x + y
}

// struct matching
select point {
  {x: 0, y} => y,                // exact match
  {x, ..} => x                   // partial (needs type annotation)
}

// array slice patterns
select arr {
  [x, rest..] => x,              // head + tail
  [init.., x] => x,              // init + last
  [a, b, c] => a + b + c,        // exact length
  [] => 0                         // empty
}

// named capture
select val {
  x@ `Some(inner) => use_both(x, inner),
  _ => default
}
```

**Key**: unselected arms are put to sleep (subscriptions paused, no
computation). First matching arm wins.

### Sample Operator (`~`)

Returns right side's value when left side produces an event.

### Connect — Reactive Update (`<-`)

The ONLY way to create cycles. Schedules an update for the NEXT cycle.

```graphix
let x = 0
x <- x + 1                       // infinite counter: 0, 1, 2, ...

// conditional update
let count = {
  let x = 0;
  select x {
    n if n < 10 => x <- n ~ x + 1,
    _ => never()                  // stop
  };
  x
}

// event-driven update
let name = ""
text_input(#on_input: |v| name <- v, &name)
```

```graphix
let clock = time::timer(duration:1.s, true)
let counter = 0
counter <- clock ~ counter + 1 // increment on each tick

// in callbacks: sample current state at event time
#on_press: |click| println(click ~ "clicked at [counter]")
```

### Error Handling

```graphix
// create and propagate
error(`NotFound("missing"))?

// try-catch
// try-catch always evaluates to the last expression in try
// even if there is an error
try {
  risky_op()?;
  another_op()?
} catch(e) => handle(e)

// ? propagates to nearest catch
// $ swallows error (logs warning, returns never)
a[100]$                           // won't crash, just skips
```

### References

```graphix
let v = 42
let r = &v                        // create reference
*r                                // dereference (read)
*r <- new_value                   // update through reference
```

References are critical for UI — widgets take `&` params so
fine-grained updates propagate without rebuilding the whole tree.

### Modules & Imports

```graphix
use array                         // bring module into scope
use gui::text                     // specific item
array::map(xs, f)                 // qualified access
map(xs, f)                        // after `use array`

mod mymod;                        // declare file-based submodule
```

File layout: `foo.gx` (impl), `foo.gxi` (interface, optional).
For directories: `foo/mod.gx`, `foo/mod.gxi`.

### Interface Files (`.gxi`)

Declare a module's public API. Items not in the interface are private.
`type`, `mod`, and `use` from the interface apply to the implementation
automatically — don't duplicate them in the `.gx` file.

```graphix
// math.gxi
/// Add two numbers
val add: fn(i64, i64) -> i64;

/// Subtract
val sub: fn(i64, i64) -> i64;

type Constants = { pi: f64, e: f64 };
val constants: Constants;

mod utils;                        // export a submodule
```

```graphix
// math.gx — types/mods from .gxi are already in scope
let add = |a, b| a + b;
let sub = |a, b| a - b;
let constants = { pi: 3.14159265359, e: 2.71828182845 };
let internal_helper = |x| x * 2  // not in interface → private
```

Doc comments (`///`) are only valid in `.gxi` files, before `val`,
`type`, or `mod` declarations. They are a syntax error in `.gx` files.

### Abstract Types

Declare a type in the interface without `= definition` to hide its
representation. Users can't construct or pattern match on it — they
must use exported functions.

```graphix
// counter.gxi
type Counter;                     // opaque — no definition exposed
val make: fn(i64) -> Counter;
val get: fn(Counter) -> i64;
val increment: fn(#trig: Any, &Counter) -> null;
```

```graphix
// counter.gx
type Counter = i64;               // concrete definition stays private
let make = |x: i64| -> Counter x;
let get = |c: Counter| -> i64 c;
let increment = |#trig: Any, c: &Counter| -> null { *c <- trig ~ *c + 1; null }
```

Abstract types can be parameterized (`type Box<'a>;`) and constrained
(`type NumBox<'a: Number>;`). The implementation must have matching
parameters and constraints.

### Standard Library Quick Reference

**Always available (core)**: `print`, `println`, `dbg`, `log`,
`cast<T>(x)`, `error(v)`, `is_err(v)`, `filter(pred, v)`,
`filter_err(v)`, `count(v)`, `once(v)`, `uniq(v)`, `sum(v)`,
`product(v)`, `min(v)`, `max(v)`, `mean(v)`, `and(a,b)`, `or(a,b)`,
`all(v)`, `queue(v)`, `hold(v)`, `take(n,v)`, `skip(n,v)`,
`throttle(dur,v)`, `never()`, `seq(start,end)`

**array**: `map`, `filter`, `filter_map`, `fold`, `flatten`, `find`,
`find_map`, `concat`, `push`, `push_front`, `window(#n, trigger, val)`,
`len`, `iter`, `iterq`, `sort`, `enumerate`, `zip`, `unzip`

**str**: `contains`, `starts_with`, `ends_with`, `trim`, `replace`,
`split`, `rsplit`, `to_upper`, `to_lower`, `concat`, `join`, `len`,
`sub`, `parse`

**map**: `map`, `filter`, `filter_map`, `fold`, `len`, `get`, `insert`,
`remove`, `iter`, `iterq`

**time**: `timer(timeout, repeat)`

**re**: `is_match`, `find`, `captures`, `split`, `splitn`

**rand**: `rand`, `pick`, `shuffle`

**fs**: `read_all`, `read_all_bin`, `write_all`, `write_all_bin`,
`watch`, `watch_full`, `readdir`, `metadata`, `is_file`, `is_dir`,
`tempdir`, `join_path`, `create_dir`, `remove_dir`, `remove_file`,
`set_global_watch_parameters`

### GUI Patterns (iced-based)

Programs return `Array<&Window>`. Widget args are mostly `&` references.

```graphix
use gui;
use gui::text;
use gui::column;
use gui::button;

let clicked = false;

let col = column(
    #spacing: &20.0,
    #padding: &`All(40.0),
    #halign: &`Center,
    #width: &`Fill,
    &[
        text(#size: &24.0, &"Hello!"),
        button(
            #on_press: |c| clicked <- c ~ true,
            #padding: &`All(10.0),
            &text(&"Click me")
        ),
        text(&"Clicked: [clicked]")
    ]
);

[&window(#title: &"My App", #theme: &`CatppuccinMocha, &col)]
```

**GUI widgets**: `window`, `text`, `button`, `text_input`, `checkbox`,
`toggler`, `radio`, `slider`, `progress_bar`, `pick_list`,
`column`, `row`, `container`, `scrollable`, `stack`, `space`, `rule`,
`tooltip`, `canvas`, `chart`, `image`, `mouse_area`, `keyboard_area`,
`text_editor`, `clipboard`

**Layout enums**: `` `Fill ``, `` `Shrink ``, `` `Fixed(f64) ``

**Padding**: `` `All(f64) ``, `` `Axis({x: f64, y: f64}) ``, `` `Each({top: f64, right: f64, bottom: f64, left: f64}) ``

### TUI Patterns (ratatui-based)

Programs return a single TUI widget. `input_handler` wraps widgets to
capture keyboard events.

```graphix
use tui;
use tui::list;
use tui::block;
use tui::text;
use tui::input_handler;

let selected = 0;
let items = [line("Apple"), line("Banana"), line("Cherry")];

let handle_event = |e: Event| -> [`Stop, `Continue] select e {
    `Key(k) => select k.kind {
        `Press => select k.code {
            k@`Up if selected > 0 => {
                selected <- (k ~ selected) - 1;
                `Stop
            },
            k@`Down if selected < 2 => {
                selected <- (k ~ selected) + 1;
                `Stop
            },
            _ => `Continue
        },
        _ => `Continue
    },
    _ => `Continue
};

input_handler(
    #handle: &handle_event,
    &block(
        #border: &`All,
        #title: &line("Pick a fruit"),
        &list(
            #highlight_style: &style(#fg: `Black, #bg: `Yellow),
            #selected: &selected,
            &items
        )
    )
)
```

**TUI text helpers**: `line("text")`, `span("text")`,
`style(#fg: Color, #bg: Color, #add_modifier: [Modifier])`

**TUI widgets**: `block`, `paragraph`, `list`, `table`, `tabs`,
`gauge`, `line_gauge`, `sparkline`, `bar_chart`, `canvas`, `chart`,
`calendar`, `browser`, `input_handler`

**Colors**: `` `Red ``, `` `Green ``, `` `Blue ``, `` `Yellow ``, `` `Cyan ``,
`` `Magenta ``, `` `White ``, `` `Black ``, `` `Rgb(u8,u8,u8) ``

### Key Reactive Idioms

```graphix
// timer-driven update
let clock = time::timer(duration:1.s, true)
let count = 0
count <- clock ~ count + 1

// sliding window of last N values
let data: Array<f64> = []
data <- array::window(#n: 60, new_val ~ data, cast<f64>(new_val)?)

// state that stops updating
select x {
  n if n < limit => x <- x + 1,
  _ => never()
}

// event callback updating state
#on_input: |v| name <- v
#on_toggle: |v| enabled <- v
#on_press: |click| counter <- click ~ (counter + 1)
```

### Gotchas

- `<-` schedules for NEXT cycle, not current. You won't see the new
  value until the next update round.
- `~` is required in callbacks to sample current state at event time.
  Without it, the callback captures the initial value.
- Tuples need 2+ elements: `(x)` is just grouping, not a 1-tuple.
- Blocks need 2+ elements: {x + 1} is a syntax error.
- Union types use `[]`: `[i64, null]` is "i64 or null", NOT an array.
  Array type is `Array<i64>`. Array literal `[1, 2]` is context-dependent.
- Variants always have backtick prefix: `` `Foo ``, `` `Bar(x) ``.
- Struct literal `{x, y}` is shorthand for `{x: x, y: y}`.
- Functional update: `{s with field: new_val}` — copies struct with changes.
- `select` must be exhaustive (cover all cases) with no dead arms.
- `never()` returns a value that never arrives — used to stop reactive loops.
- you must escape square brackets in string literals "[name] must be between \[0, 1\]"

## Using netidx — Rust API Reference

netidx is NOT well-represented in training data. This project uses in-process mode (`InternalOnly`). Network mode exists but is not covered here.

### Setup (InternalOnly)

```rust
use netidx::InternalOnly;
let net = InternalOnly::new().await?;  // spawns internal resolver on loopback
let publisher = net.publisher().clone();   // Arc-wrapped, cheap clone
let subscriber = net.subscriber().clone();
// Drop InternalOnly → kills internal resolver
// net.shutdown().await for graceful shutdown
```

### Publisher API

- `Publisher` — Arc-wrapped, cheap clone
- `Val` — handle to published value (wraps `Id`), unpublishes on Drop
- `UpdateBatch` — batched update handle
- `WriteRequest { id: Id, path: Path, client: ClId, value: Value, send_result: Option<SendResult> }`

Key methods:

```rust
// Read-only publish
publisher.publish(path: Path, init: T) -> Result<Val>  // T: TryInto<Value>

// Publish with flags
publisher.publish_with_flags(flags: PublishFlags, path: Path, init: T) -> Result<Val>

// Publish writable (subscribers can write back)
publisher.publish_with_flags_and_writes(
    flags: PublishFlags, path: Path, init: T,
    tx: Option<Sender<GPooled<Vec<WriteRequest>>>>
) -> Result<Val>

// Register write handler after publish
publisher.writes(id: Id, tx: Sender<GPooled<Vec<WriteRequest>>>)

// Batched updates
let mut batch = publisher.start_batch();
val.update(&mut batch, value);           // queue update (T: Into<Value>)
val.update_changed(&mut batch, value);   // only if different
batch.commit(None).await;                // send all atomically, consumes batch
```

PublishFlags: `USE_EXISTING`, `DESTROY_ON_IDLE` (free when no subscribers), `ISOLATED`, `PREFER_LOCAL`, `FORCE_LOCAL`

### Subscriber API

- `Subscriber` — Arc-wrapped, cheap clone
- `Dval` — durable subscription with auto-reconnect
- `Event` — `Update(Value)` | `Unsubscribed`
- `UpdatesFlags` — `BEGIN_WITH_LAST`, `STOP_COLLECTING_LAST`, `NO_SPURIOUS`
- `UpdateChan` = `Sender<GPooled<Vec<(SubId, Event)>>>` (from `futures::channel::mpsc`, NOT tokio)

```rust
let dval = subscriber.subscribe(path);   // returns Dval
dval.updates(flags, tx: UpdateChan);     // register for updates
dval.last() -> Event;                    // current value
dval.write(value) -> bool;              // write back to publisher
dval.id() -> SubId;
```

### Value Type

Dynamic enum from `netidx::publisher::Value` (re-exported from netidx-value):

- Integers: U8, I8, U16, I16, U32, V32 (varint), I32, Z32 (zigzag), U64, V64, I64, Z64
- Floats: F32, F64
- Other: Bool, Null, String(ArcStr), Bytes(PBytes), Array(ValArray), Map, Decimal, DateTime, Duration, Error(Arc\<Value\>), Abstract

Conversions via `FromValue` trait:

```rust
Value::from(42_f64)              // Into<Value>
Value::from("hello")             // &'static str
Value::from(ArcStr::from(s))     // runtime String

// FromValue trait — implemented for numeric types, ArcStr, etc.
f64::get(value) -> Option<f64>         // exact type match only
f64::from_value(value) -> Result<f64>  // with casting
```

### Path

`netidx::path::Path` — hierarchical, like `/ship/reactor/plasma/temperature`

### Gotchas

- `UpdateChan` uses `futures::channel::mpsc::Sender`, not tokio mpsc
- Write channels use `Sender<GPooled<Vec<WriteRequest>>>` — poolshark-pooled batches
- `batch.commit(timeout).await` consumes the batch (takes `self`)
- `Val` unpublishes on Drop — keep it alive as long as you want the value published
- `publisher.publish()` returns `Err` if path already published by this publisher

## Using poolshark — Rust API Reference

poolshark is NOT in the training set. It provides memory pools that reuse container allocations. Two strategies: local (thread-local, no atomics) and global (cross-thread, lock-free).

### When to Use Which

- Default: `LPooled<T>` — fastest, zero overhead, no atomics
- Cross-thread (producer-consumer): `GPooled<T>` — 1 word overhead, returns to origin pool
- Named shared pool: `Pool<T>` with `LazyLock`

### Local Pools (LPooled)

```rust
use poolshark::local::LPooled;

let mut v: LPooled<Vec<u8>> = LPooled::take();     // from pool or new
let mut v: LPooled<Vec<u8>> = LPooled::default();   // same thing
v.push(42);                                          // Deref/DerefMut
let inner: Vec<u8> = v.detach();                     // extract, skip pool
// On drop: cleared via reset() and returned to dropping thread's pool
```

Requires `T: IsoPoolable` (extends `Poolable` with layout-based reuse).

### Global Pools (GPooled, Pool)

```rust
use poolshark::global::{self, GPooled, Pool};
use std::sync::LazyLock;

let mut v: GPooled<Vec<u8>> = global::take();       // thread-local global pool

// Named static pool
static BUFFERS: LazyLock<Pool<Vec<u8>>> = LazyLock::new(|| Pool::new(1024, 4096));
let mut buf = BUFFERS.take();

// Orphan (not pooled)
let v = GPooled::orphan(Vec::new());
// On drop: returned to origin pool (not dropping thread's pool)
```

`Pool::new(max_capacity, max_elt_capacity)` — defaults `(1024, 1024)`. Objects exceeding `max_elt_capacity` are freed, not pooled.

### Poolable Trait

```rust
trait Poolable {
    fn empty() -> Self;
    fn reset(&mut self);         // clear for reuse
    fn capacity(&self) -> usize;
}
```

Built-in for: `Vec<T>`, `VecDeque<T>`, `String`, `HashMap<K,V>`, `HashSet<K>`, `IndexMap<K,V>`, `IndexSet<K>`, `Option<T: Poolable>`

### Gotchas

- `LPooled` = zero stack overhead, returns to dropping thread's pool
- `GPooled` = 1 word overhead, returns to origin (creating) pool
- Objects exceeding `max_elt_capacity` are freed, not pooled
- `detach()` extracts inner value — it won't be pooled
- netidx channels use `GPooled<Vec<...>>` for cross-thread event batches
