# The Cockpit

A pilot in a liquid-filled capsule, behind a BCI, can be shown anything.
That freedom is the problem: a display that could be anything has to be
something, and the wrong something kills the pilot. This chapter fixes
what the BCI shows: a cockpit. Not as nostalgia but because people (and
the AI persons who fly beside them) reason fast in space and remember
places faster than they remember menus. A pilot who knows the fuel
master is at the left hip reaches for it without looking, at 20 g, in
the dark, with a missile inbound. A pilot who knows it is three levels
down an MFD page does not.

It applies to every craft flown through a BCI: the fighter (see
[Small Craft](small-craft.md)), shuttles, gunboats, the courier's helm
stations. A baseline human, without the implants, flies from physical
consoles laid out the same way, so a skill learned on one is a skill on
the other.

## Location Is Identity

The one rule everything else serves: **a control's place is what it is.**

- Panels never rearrange themselves. Nothing moves to make room;
  nothing reflows because a window opened.
- A panel shows its top-level functions. The pilot **pulls it up**, like
  a drawer, for the next level of detail, and it unfolds in place,
  without moving its neighbours. Pull again for the level below.
- In an emergency the system **points; it does not move.** The master
  caution names the panel and throws a cue toward it. At most a ghost
  copy floats forward, while the original stays where the hand expects
  it.

For example, the engine and fuel panel sits at the left hip: the fuel
master, the turbopump and turbogenerator start, the engine mode. Pulled
up, it shows the fuel system as it is built, not as a list: the tanks,
the valves and the pumps where they are in the airframe, each valve a
switch, each line showing its flow. Pulled up again at a pump, it shows
that pump's speed, inlet pressure, bearing temperature and its own
controls.

The levels are not invented for the display. **The panel tree is the
netidx tree.** The hierarchy at `/fighter/fuel/...` mirrors the
aircraft's physical structure (see [Architecture](architecture.md)), so
pulling a panel up walks one level down the tree, and each panel is a
Graphix program subscribed to its subtree. The cockpit has no privileged
layer beneath it: it is the same data and the same language the pilot can
program.

## Reaching

The pilot's body cannot move at 20 g. The pilot's virtual body can, so
**the cockpit works at any g**: reaching for a switch is a motor act the
BCI decodes, and motor plans are what practised hands are made of.

**The BCI writes back as well as reads.** A switch has a detent you feel
go over; a guarded switch resists until the guard is lifted; a knob
clicks through its positions; a stick has its feel. Without that feedback
reaching turns slow and uncertain, as it does on a touchscreen, and the
point of a cockpit is lost.

**A deliberate reach is the confirmation.** Commands "thought" without a
reach misfire: the BCI cannot tell an intention from an idle thought
well enough to trust it with a weapon or a valve. Thought and voice serve
for what is cheap to undo: a view, a pin, a query to the ship's AI.
Anything that commits fuel, a weapon or the airframe is a reach, and the
critical ones keep their guards.

## Frames

Every element lives in one of four frames, and the pilot can move any
element between them.

| Frame | Fixed to | Holds, by default |
|---|---|---|
| World | the world outside | targets, terrain, the flight-path marker, threat rings, waypoints |
| Aircraft | the airframe | the panels; the cockpit itself |
| Body | the pilot's torso | the hip and thigh panels, so the hands find them however the head turns |
| Head | the pilot's view | pinned values |

**The canopy is synthetic.** The capsule has no windows. The aircraft's
all-round sensors (see [Sensors](sensors.md)) give a view in every
direction, through the airframe as if it were not there. Panels in the
aircraft frame fade when the pilot looks through them at something
outside, and come back when the pilot looks at them.

**Pinning.** Any value on any panel can be pinned to the head frame,
where it stays in the same part of the view wherever the pilot looks:
turbine inlet temperature and fuel flow, say, through a hot climb. **A
pin is a watch.** It carries a rule (a limit, a rate, a trend) and it
sits dim at the edge of vision until the rule trips, then it brightens,
moves into the view and, if it is serious, sounds. What a pilot means by
"watch TIT" is "tell me when TIT does something", and the pin does that,
so the pilot does not have to stare at it.

## Gauges

**Needles and tapes, not digits, are the default.** A needle's angle
and a tape's position are read at a glance, in peripheral vision, without
reading anything: where it is, which way it is going, how fast. A digit
has to be read and compared. That is why round gauges outlived the
digital displays meant to replace them. Digits stay available, on a
gauge's face or as a pin, for when an exact number matters.

Drawn rather than built, the gauges do what real ones cannot:

- **Live limit arcs.** The green, yellow and red bands move with
  conditions: the turbine's red line slides with altitude and Mach, so
  the arc always shows the true limit, not the worst case.
- **The trend ghost.** A faint second needle where the value will be in
  ~10 s at its present rate. Rate and acceleration read without a second
  instrument.
- **Bugs.** Markers the pilot sets on a gauge by hand (a heading, a
  speed, a fuel state to turn home at), dragged into place with the
  virtual hand.
- **Real bezels.** A knob turns with the feel of a knob, through the
  BCI's feedback.

The look is a brass-and-glass cockpit of needles, tapes and guarded
switches, in a craft that may have no physical instrument in it at all.
It earns that look: it is the best display for the job, not a costume.

## The Standard Layout

The trap in a display that can be anything is that every pilot makes it
something different. So there is a **standard home layout**, the same in
every craft of a type, and every station holds the same functions in the
same place. It is what lets an instructor say "left hip, second switch",
lets a checklist work, and lets a pilot change aircraft.

**Personal changes are an overlay**: pins, bugs, extra gauges, a moved
panel. They are saved as a profile the pilot carries from craft to craft
and lays over the standard. The standard is always one gesture away, and
it is where the cockpit returns when an overlay fails (see below).

## The AI Beside You

An AI person flying with a human, as copilot or wingman's controller,
uses the same cockpit. When it works a control, the human sees a **ghost
hand** reach for the switch and throw it. The pilot keeps track of the
aircraft by seeing what is done, where it is done, rather than reading a
log afterward. A human in the back seat of a two-seat trainer has
always watched the instructor's hands; this is the same thing.

## Rewriting It in Flight

The cockpit can be changed while flying, by the pilot, at any depth.
Every panel is a Graphix program over the netidx tree, so changing one
is editing a program, and Graphix's reactive model runs the change on
the next cycle. Nothing restarts; the rest of the cockpit keeps flying.
The editor is itself a panel, a code slate that pulls out from the
right thigh.

This is the scene from *Gundam SEED* where Kira rewrites the Strike's OS
with the enemy in front of him, made something a player can do. Done
with slow time (see [Architecture](architecture.md#time-dilation-mechanic)),
it looks like this:

- **The world slows; it does not stop.** At 1/100 speed the missile
  still crawls toward you, and the clock is the drama.
- **The program is shown as a graph.** The editor draws the panel's
  dataflow as nodes and wires laid over the cockpit, with values moving
  along them. Rewire TIT to a new limit, change a valve's schedule, hang
  a new watch on the turbopump, and see the gauges change as each edit
  lands.
- **There are problems only a rewrite solves.** A sensor damaged into
  feeding garbage that the standard panel trusts; re-plumbing that panel
  to the backup estimate, or to a value computed from the others, is
  what saves you.
- **Your code can fail.** The rewrite is real Graphix, written in a
  fight. Half-finished programs have real bugs.

Two things keep it believable rather than suicidal:

- **Custom code writes only what the flight computer exposes as
  writable.** It can rewrite how the fuel system is shown and driven. It
  runs in the cockpit, never inside the flight-control loop that keeps
  an unstable airframe flying, and it cannot touch the control laws.
- **A broken panel falls back.** If a program errors or stalls, its
  panel returns to its last good version without being asked, and the
  standard layout is one gesture away.

Who can do this in a fight depends on how fast they think, which is
the time-dilation mechanic, and in multiplayer it is a question about
everyone in the fight (see
[Architecture](architecture.md#slow-time-in-multiplayer)).
