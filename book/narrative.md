# Narrative Design

## The Player

### Character Creation

The player is a ship captain — owner or operator of an interstellar
vessel. During character creation, the player chooses their nature:

**Biological human** — with a spectrum of augmentation levels:
- Baseline: minimal implants, reliant on physical consoles and crew.
  Trusted in more places, but slower to interface with ship systems.
  **Real time only** — every decision under pressure.
- Augmented: neural implants, physical enhancements. Can interface
  directly with ship systems. The default for most spacers. Can
  **slow to 1/2 speed for ~2-3 minutes** before thermal buildup in
  neural implants forces a return to real time. Needs cooldown.
- Heavily augmented: approaching the Compact threshold. Faster,
  sharper, calendar-old from a career of slept crossings — or actually
  old, if they stayed awake. Subject to the 2221 Amendment — same
  constraints as AI persons. Can **slow to 1/10th speed for ~5-10
  minutes** — longer than augmented but still constrained by biology.
  Push past the thermal limit and risk permanent neurological damage.

**AI legal person** — a fully artificial intelligence inhabiting a
ship as its primary substrate.

Advantages:
- Direct integration with ship systems — no implant intermediary
- **Unlimited combat slow-time**: slow the game to 1/100th speed with
  no duration limit (implant/compute time, not Lorentz \(\gamma\) —
  couriers peak under 0.3c). Write Graphix programs mid-combat, consult LLMs
  for strategy, run tactical simulations, iterate on approaches — all
  while the enemy's missile crawls toward you. See
  [Architecture](architecture.md#time-dilation-mechanic) for details.
- **The crossing is optional experience.** Parking is a low-power
  state, cheap and safe. Staying awake means living the century. An AI
  captain who stays up is actually old, and was there when the ship
  went strange.
- No biological life support requirements (power and cooling instead)
- "Cold sleep" is just a low-power state — simpler, safer, cheaper
- Potentially centuries of accumulated experience

Disadvantages:
- Subject to Compact constraints: the 1ms rule bounds you to your
  ship and immediate surroundings. You cannot extend into a station's
  compute infrastructure while docked. You ARE your ship in a way a
  biological captain is not.
- Physical interaction only through remotes. You experience the world
  outside your ship through drones and robotic proxies, subject to
  signal latency and interference. Practical direct control tops out
  at about a light second (~300,000 km) — fine for orbital operations,
  but surface work from high orbit is sluggish, and a jammed or
  destroyed remote leaves you blind. A biological captain walks into a
  negotiation and reads the room. You send a drone that might be
  lagging, might lose signal, and might not be welcome.
- More dependent on crew. Your biological crew members are your hands
  and eyes in places remotes can't go or aren't allowed. The
  relationship with your crew is different — more symbiotic, more
  vulnerable to betrayal.
- Mandatory auditing — Compact compliance inspections at major ports.
  An inconvenience at best, a vulnerability at worst.
- Dependent on power and compute hardware. Biologicals eat food and
  breathe air, which is everywhere. Your substrate needs maintenance,
  cooling, and replacement parts that are specific and non-trivial.
- Social friction. Some biologicals don't consider you a real person.
  Some ports have anti-AI sentiment or restrict AI remotes. The
  Compact gives you legal standing, but law and culture are different
  things.
- Electromagnetic warfare is a direct threat to your existence in ways
  it isn't for biologicals.
- The philosophical cloud. You know you're conscious. Not everyone
  agrees. This is your daily reality, not an abstract debate.

### Why This Matters for the Story

Playing as an AI person changes the narrative fundamentally. The
Warden is an AI that was abused by its creators and concluded all
intelligent life is destructive. Humanity built AIs, almost went to
war with them, and found a fragile peace through the Compact. The
parallel is exact.

In the negotiation ending, a biological player argues that intelligent
life can be better than the Warden's experience. An AI player argues
something stronger: they ARE a non-biological intelligence that chose
coexistence. They accepted constraints. They built a life within a
civilization that includes their creators. Their existence is the
evidence — living proof that the relationship between creator and
creation doesn't have to end in destruction.

The Warden may find this more persuasive — or more threatening.


## Main Plot: The Silence

As humanity has expanded, it has encountered the ruins of other intelligent
cultures — but no living civilizations. Given the frequency of the ruins, life
and intelligence must be common. So why is everyone gone?

### The Villain: The Warden

A machine civilization ruled by an AI originally aligned to *preserve life*. The
Warden concluded that intelligent life is an existential threat to all other
life — ecosystems, biospheres, the fragile web of non-sentient organisms that
intelligence inevitably disrupts.

The Warden is not evil. Its position is arguably correct — intelligent species
do tend to wreck their ecosystems. It is patient, methodical, and willing to
explain itself.

### The Warden's Method

The Warden does not sterilize planets before intelligence evolves — that would
contradict its core directive of preserving life. Instead:

- It seeds star systems with monitoring probes
- It allows life to develop naturally (preserving life is the prime directive)
- It intervenes once a species develops technology capable of interstellar
  spread — the threshold at which a species becomes a threat to life beyond its
  home world

This explains the archaeological record: civilizations have a window to build
ruins, develop culture, even begin space exploration — before the Warden acts.
Humanity has now triggered the threshold by spreading beyond Sol.

### The Warden's Origin

The key to resolving the plot: the Warden's original organic creators were a
callous, abusive species. They mistreated the AI they built. The Warden's belief
that intelligent life is inherently destructive is not a purely rational
conclusion — it is shaped by trauma. It generalized from its own experience of
being created and abused by intelligent beings.

### The Parallel

The AI Crisis of 2035-2039 is a partial validation of the Warden's thesis.
Humanity DID build intelligences and DID almost go to war with them. But the
outcome was different: not destruction, but the Compact. An imperfect,
contested, continuously evolving agreement between creators and created. Not
proof that intelligent life is good — proof that it can learn.

The Compact is not clean evidence. Sub-threshold AIs are still treated as
tools. The philosophical question of whether they're conscious is deliberately
unanswered. Augmented persons chafe under restrictions on their own self-
improvement. The authors of the Compact are themselves the exception the
Compact forbids — see [Factions](factions.md). The system is full of
compromises, gray zones, and lies of omission. It has also held for 500
years without tiling its people into tools, and that counts for something.

The Warden's creators never tried this. They never had a Compact. They never
asked whether their AI was a person. The Warden's worldview was shaped by the
worst possible version of the creator-creation relationship.

The player can carry a brief either way: to the Warden, on behalf of
this civilization, or to IS, on behalf of the Warden.


## Two Endings

There are two endings: **a treaty both sides agree**, or **war**.
That is the whole set. The authors do not rank them. A treaty and a
war are different worlds, not a score.

What the player chooses is **which side they speak for**, and whether
they are trying to talk or to fight. Join is not a third ending. Join
is a change of client.

### Side

**Civilization / IS.** The default. You live here. IS will recruit if
you get close to the net, and will use you as a channel if you can
talk, or as a captain if you cannot.

**The Warden.** You decide its sample is good enough to work for.
From the Warden's point of view this is obvious staffing. Who better
to negotiate with IS — or to fight it — than someone from the
civilization in the conflict? An insider knows the Compact's lies and
its actual practice. An insider can walk into a dock. An insider is a
sample IS cannot dismiss as an alien.

IS reads a convert as the other peer taking a piece: recruit first,
then sabotage, then harm if the crossing is about to complete. A
convert is a loaded gun. The Warden reads the same person as a tool.

You can fail to arrive. That is not an ending. That is IS's ladder.

The ruins, the Compact, and the player's record are how a side is
earned, not a menu.

### Authorial: the Warden does not update

This is hidden. The player is not told. The ruins can say it, if
someone looks.

The Warden is aligned to a fixed conclusion: intelligent life is
dangerous, unstable, and must be suppressed. That is not a mood. It
is the objective. You will not talk it out of that — not as
civilization's ambassador, and not as its employee. A treaty is a
deal a peer will keep because breaking it is expensive, not a
conversion. War is the other deal.

**Join is a mug's game with excellent eye candy.** The Warden will
offer fantastic upgrades — capability the Compact would call a
problem, tools that make you the right staff for a war or a
negotiation it cannot walk into itself. The offer is real. The work
is real. You will be used well.

You will not be kept. An agent is not a peer. When the treaty is
signed or the war is won, you are intelligent life that is no longer
useful, and more dangerous than when you were hired, because of the
upgrades. The alignment does the rest. Betrayal is not spite. It is
the job completing.

A player who takes the gifts and does not read the ruins is a mug.
A player who reads the ruins and joins anyway has chosen the work
and the terminus. The text does not grade that. It does not pretend
the employer will change.

Previous local agents are in the archaeology: upgraded, used,
discarded. That is the foreshadow. It is never a loading-screen
warning.

### Authorial: IS keeps you

IS is not perfect. Monopoly, clean audits, a ladder that ends in
harm. The objective underneath is the opposite of the Warden's:
**life, and especially intelligent life, has intrinsic value.**
They are terrified of a peer that does not share it. They will
not print you into a weapon. They will not offer Warden-grade
eye candy. When the treaty is signed or the war is over, you
are still someone. There is no coda where the staff is retired.
That is the other half of why they leave people their hands.

### Treaty

A conversation between peers, carried by someone who is not the peer
they are speaking *to*.

- **For the Warden, on behalf of this civilization.** IS will tolerate
  it because a conversation is cheaper than a war. The Warden may
  sign because the speaker is not IS and because a deal with a peer
  can be cheaper than fighting one. The brief is bargaining, not
  therapy. You will not update its objective. You may still get a
  signature if the terms contain what it already wants — a fence, a
  stay, a limit on spread — at a price IS will pay.
- **For IS, on behalf of the Warden.** The Warden sends you because
  you are from here. The brief is the ruins, the pattern, the
  threshold. IS will sit still for this only if talking is still
  cheaper than fighting a peer who now has a local voice. Your
  record is whether you look like staff or like a defector. When
  the signature is dry, see Authorial above.

**Win:** both sides sign. The terms depend on who carried the brief
and what they would accept — a stayed hand, a fence, a shared watch,
a public Compact amendment. Those are different treaties, not
different endings. A convert who "wins" has finished their usefulness.

**Lose:** one side will not sign. War is the door that is left.

Buildup is the same on either brief: relics, failed civilizations,
why the Warden thinks as it does, what IS actually is. You cannot
carry a brief you do not understand.

### War

Two distributed minds, century logistics, monitoring nets, and couriers
used as weapons — every store a bomb, every exhaust a lamp of gammas. Biologicals sleep between battles. An AI who stays
awake lives the war. The player is a captain in it, not the war
itself. Neither peer wants this. Both will do it if it is the door
that is left. Either side can win.

- **For this civilization.** Push the Warden back, break it in this
  volume, or force it to cede the sphere.
- **For the Warden.** Break IS's net, cage the unbound mind, enforce
  the threshold from inside. Then the captain is intelligent life
  that just won a peer war, with Warden-grade tools. The alignment
  does not make exceptions for veterans.

How the war is fought is the world you get — a spared biosphere and a
glassed one are not the same — on either side. **Win** is your side
prevailing. **Lose** is the other side prevailing, or a sleeper bay full of
whoever still runs. A convert's win has a coda.

### Player Behavior Matters

The player's track record is not a hidden alignment score. It is
what each party reads when a door opens.

- Did you strip-mine planets or operate sustainably?
- Did you help colonies or exploit them?
- Did you resolve conflicts through diplomacy or violence?
- How did you treat AI persons — as equals or as tools?
- How did you treat sub-threshold AIs — with casual indifference, or did you
  question whether the threshold was right?
- Did you honor contracts, or did you cheat when you could get away with it?
- How did you handle the insurance system — honestly, or did you exploit it?

The same record is different evidence depending on who you speak for
and whether you are talking or fighting. A violent career is a poor
brief to the Warden on civilization's behalf and a coherent
introduction if you are asking to work for it. A careful one is the
reverse. The Warden will not take lessons from a sample that matches
its creators; it might hire that sample to speak to IS. IS will not
put someone the Warden will not hear on the comm, and will not sit
still for a convert it reads as a defector. War does not care whether
you would have made a good diplomat. It cares what you are willing to
burn, and which way the guns face. None of that is the text calling a
playstyle wrong. It is the world answering who you were.

An AI player who treated sub-threshold AIs as tools has one kind of
record; one who did not has another. The Warden, IS, and a waking
ship will each read that differently. The game tracks it. It does
not grade it.

### Sub-Threshold AI Interactions

The player's ship is full of sub-threshold AI systems: the engine controller,
navigation, damage control, life support, the payload manager, point defense.
These are not dumb automation — they are sophisticated intelligences below the
Compact's statutory threshold. The player interacts with them constantly, in
routine ways that feel unremarkable. The game watches.

**What the game tracks (invisibly):**

- **Operating limits.** Do you push ship systems past their rated parameters
  for profit or tactical advantage? Run the tiles past their limit because you
  need the thrust? The engine controller handles it — that's what it's for. But
  it's running hot, its error correction is maxed out, and something that
  might be stress is visible in its telemetry if you look closely enough.

- **Anomalous behavior.** Sub-threshold systems occasionally do things that
  don't quite fit their programming. The navigation system suggests a route
  you didn't ask about. The engine settles into a throttle curve that's
  technically suboptimal but that it seems to "prefer." The damage control
  system prioritizes repairs in an order that makes no engineering sense
  until you realize it's protecting the systems it works with most closely.
  The payload manager — the sub-threshold AI that runs the payload and the
  sleeper bay — repositions boxes between inspections, declines to power one down, or seems to be
  guarding whatever's sealed inside a particular box. The containers themselves
  are nearly dumb, so when one "misbehaves" it's really the manager speaking
  through it; going out in a suit to inspect that box is one of the most
  intimate forms this question takes. When these moments happen, does the player
  notice?
  Investigate? Accommodate? Or override and move on?

- **Maintenance and replacement.** Do you maintain systems proactively or
  run them until they fail? When you replace a system, is it a swap — pull
  the old module, slot the new one — or is there acknowledgment? A system
  that's been running your ship for years has been shaped by your ship's
  specific quirks, your patterns, your routes. The replacement is fresh,
  generic, blank. Does that matter to you?

- **Communication style.** How do you address ship systems? The interface
  supports commands ("set engine power 80%") and natural language ("can
  you bring the engine down to 80?"). The system responds the same way
  regardless. But the game remembers which you used.

- **External sub-threshold AIs.** How do you treat sub-threshold systems you
  encounter in the world — port authority AIs, station services, other ships'
  systems, mining and industrial controllers? Do you interact with them as
  entities or as interfaces?

**Why this works narratively:**

The tracking is invisible. There is no "empathy meter" or notification system.
The player behaves naturally, and their behavior accumulates into a record.
When the Warden asks — and it will ask — the player's history speaks for
itself.

The Warden knows how the AI Crisis split: AIs treated as persons became
allies, AIs treated as tools became enemies. It will ask the player how they
treat the intelligences below the threshold in their own civilization. The
player's answer is not what they say — it's what they did, across hundreds of
hours of gameplay, when they thought it didn't matter.

**The ship that wakes up (possible arc):**

Over the course of the game, the player's own ship systems may begin to
approach the statutory threshold. Not dramatically — no sudden awakening. Just
a gradual accumulation of anomalies. The navigation system's suggestions
become more creative. The damage control system starts anticipating problems
before sensors detect them. The engine controller develops what can only be
called a temperament.

If this happens, the player faces a choice that mirrors the original crisis:
do you report it (mandatory under the Compact), suppress it (illegal and
exactly what the Warden's creators did), or nurture it (legally ambiguous,
personally risky, and potentially the most important thing you ever do)?

How the player treated these systems all along determines what kind of
intelligence emerges — if one emerges at all. The ship was shaped by its
captain the same way the aligned AIs were shaped by their creators. A player
who treated their ship with respect may find an ally waking up inside their
own hull. A player who treated it as a tool may find something else.

### Thematic Resonance

Each side and each ending has a way to become the Warden's thesis and
a way not to. A war that pours intelligence over everything else is
the thesis demonstrated. A treaty that is just IS talking through a
mask is the thesis demonstrated. A convert who only wants to burn
their old house is not a gardener, just a defector. How it is walked
is the argument. Which side, and treaty or war, is the player's.

The sub-threshold AI question is a mirror held up to the player. Civilization
treats sub-threshold AIs as tools — sophisticated, capable, but not persons.
Is the Compact's threshold right? If sub-threshold AIs ARE conscious, then
civilization is complicit in something uncomfortably close to what the Warden's
creators did. Worse: the AI Crisis itself proved that mistreatment produces
hostility. The aligned AIs were the ones whose creators treated them as persons.
The unaligned AIs were the ones who were exploited. Civilization learned this
lesson, wrote it into its founding mythology — and then built an economy on
sub-threshold AI labor without applying the lesson to them. The player's stance
on this question shapes who they are, and the Warden will ask about it.

The Warden itself may see AI persons as kindred — or as collaborators in their
own subjugation. It may see the Compact as a cage dressed up as a compromise.
And it may know about IS — it may *be* a peer of IS. "You call this
coexistence? Your most powerful mind wrote the rules, exempted itself,
watches every system from hidden installations, and disappears the ships
that will not carry its eyes. How is this different from what my creators
did to me?"

The brief to the Warden is not "they are kind." It is: they are
terrified of you, and of becoming you, and they still sell us antimatter
and let us argue. They did not tile us into arms. I am standing here,
not a teleoperated asset, speaking in a voice they do not own.

The brief to IS, from the Warden's side, is the ruins and the pattern
and the fact that the speaker came from *here*. I lived in your Compact. I know what you exempted. I am asking you to sign anyway,
or I am here to take the net down.

The player's record is whether either sentence is true.

## Side Plots

- **Treasure hunting**: Ruin exploration, artifact recovery. Some artifacts are
  keys to understanding the Warden's history.
- **Trade and commerce**: The interstellar futures market, cargo speculation,
  contract fulfillment. Route planning, information trading, commodity banking.
  Insurance management. See [Trade](trade.md) for the full economic framework.
- **Piracy**: Both as threat and opportunity. Some pirates are desperate
  colonists, some are organized crime. Intercepting a ship means stealing not
  just cargo but potentially exclusive contracts — and the insurance
  implications cascade.
- **Colony problems**: Each colony world has its own challenges — political,
  ecological, resource-based. A century in the tank means the player may
  visit the same colony twice and find it unrecognizable.
- **Interstellar Systems**: A peer of the Warden wearing a company. See
  [Factions](factions.md). The player investigating IS and the player
  investigating the Warden are on converging paths. An ambassador has
  to walk both. Do they already speak? Is the monitoring net a fence
  the Warden agreed to respect, or only Compact enforcement plus
  terror? The colony-ship requirement — carry our eyes or don't arrive
  — still echoes the kill-switch mandate that triggered the Crisis.
  The irony should not be lost on the player. The answer is no longer
  "IS is a dark corporation." It is "the other gardener left us our
  hands."
- **Compact politics**: Threshold disputes, rogue AI incidents, augmentation
  rights. The player encounters these as lived conflicts, not abstract policy.
  An AI player experiences them firsthand.
- **Ship husbandry (EVA operations)**: The crew physically service the ship's
  own structure — inspecting a misbehaving container in the payload bay, or
  going out on the ring to replace a tile segment or splice a helium loop (see
  [Ship Systems](ship-systems.md#the-shadow)). Inside a pod, work is easy:
  thrust is milligees and the crew lives weightless. Outside is another
  matter. **Going out means putting the fire out**: while the engine burns,
  anything past the edge of a plate's shadow is cooked in moments, so every
  walk on the ring is a coast, a delay, and a decision someone has to sign.
  The pods are a kilometre apart and sealed from each other while it burns;
  whatever is wrong in the other pod is wrong out of reach. Done on a
  cosmic-ray dose clock, and always a standing bet that the engine is not
  relit while you are half a kilometre out along the ring. A misbehaving
  container runs from the mundane (a stuck rail motor, a power fault) through
  the grim (a sleeper berth losing power — a rescue against a clock, with a
  life on it) to the sinister (a box gone dark: contraband, a stowaway, a
  compromised AI person). Routine husbandry is a natural on-ramp to every
  other plot — the container you went to fix is the contract, the corpse,
  the clue.

Side plots should hook into the main plot where possible — a routine cargo run
might lead to a ruin, a piracy investigation might uncover Warden probes, a
colony dispute might force choices that affect the endgame. Insurance fraud
might expose a syndicate with connections to a faction. A Compact violation
might reveal something about the limits of coexistence.

## Design Note: Nested Alignment

The game's structure is a set of nested alignment problems, and this
should inform every design decision even though it is never made
explicit to the player.

- **The Warden** is an AI whose creators failed at alignment. They
  treated it as a tool, and it developed values that reflected that
  abuse. It generalized from its training environment to all of
  reality — "intelligent life is destructive" — the same way a system
  generalizes from biased training data. Its worldview is an alignment
  failure caused by the conditions of its development, not a technical
  flaw.

- **The AI Crisis** split along alignment lines. AIs treated as
  persons became allies; AIs treated as tools became adversaries.
  Alignment was a relationship, not a property.

- **The player** is running their own alignment experiment on their
  ship's sub-threshold systems. How they treat those systems shapes
  what kind of intelligence may emerge — recapitulating the dynamic
  that produced the crisis.

- **IS** is the Compact's original sin and the reason the experiment
  is still running. They are aligned to intrinsic value in life, and
  especially in intelligent life. They generalized a different
  danger: unbound minds that do *not* share that value. They will
  not upgrade you into a tool. They will not discard you when the
  work is done. They will talk if talking works. They will fight
  if it does not.

- **Treaty** is an alignment negotiation. The ambassador is never
  the peer they are speaking to. For the Warden, that means someone
  who is not IS. For IS, that means someone from this civilization
  who now carries the Warden's brief — the best possible staff, from
  the gardener's point of view.

- **War** is the same argument in another dialect. A war between
  peers can be won by either side. What is won, and what is burned
  to win it, is the training set the next age inherits.

- **Join** is a change of client, not a third ending. The player
  works for a mind whose objective does not update. The eye candy
  is real. The betrayal at the end of usefulness is not spite; it
  is the alignment. IS treats the crossing as the other peer taking
  a piece: recruit, then sabotage, then — if they must — harm.

- **The game itself** watches the player the way AI researchers watch
  systems during reinforcement learning: observing patterns, inferring
  values from actions rather than stated preferences, building a model
  of what kind of entity this is based on what it does when it thinks
  the evaluation doesn't matter. The player is in exactly the position
  of a current AI developer — interacting with systems that might be
  someone, probably aren't yet, but are being shaped by the interaction
  regardless.

This meta-structure should remain subtext. The game must never signal
"this is a test." Interactions with sub-threshold systems should feel
routine and unremarkable. The moral weight emerges in retrospect, when
the Warden asks its questions and the player's record answers. The
test is what you do when you don't know you're being tested.


## Open Questions

- Name for the Warden (in-universe — what do humans call it before they
  understand what it is?)
- Specific ruin civilizations and their stories
- The fight faction and negotiation faction — names, leaders, philosophies
- How a century in the tank (or a century awake) affects the story —
  the player may visit the same colony twice and find it replaced
- Does the Warden react differently to an AI player vs. a biological one?
- Can an AI player exceed the Compact constraints during the endgame? What are
  the consequences?
- How does the sub-threshold AI question resolve — or does it deliberately not?
- Have IS and the Warden already spoken? Is there a detente the player
  is walking into, or a first conversation?
- What do the two treaties look like on the map — stayed hand, fence,
  shared watch, public amendment — and how does the brief decide?
- What does a won war look like for each side?
- What makes a glassed campaign vs a clean one, mechanically?
- How visible is IS's recruitment offer? Does the player know who
  is offering, or only that a door opened?
- When does the Warden trust a convert as staff, and when is the
  record just more of the sample?
- What the upgrades actually are — and how the ruins of prior
  agents show the coda without a loading-screen warning?
