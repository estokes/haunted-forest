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
  sharper, potentially centuries old via time dilation. Subject to the
  2221 Amendment — same constraints as AI persons. Can **slow to
  1/10th speed for ~5-10 minutes** — longer than augmented but still
  constrained by biology. Push past the thermal limit and risk
  permanent neurological damage.

**AI legal person** — a fully artificial intelligence inhabiting a
ship as its primary substrate.

Advantages:
- Direct integration with ship systems — no implant intermediary
- **Unlimited time dilation**: slow the game to 1/100th speed with no
  duration limit. Write Graphix programs mid-combat, consult LLMs for
  strategy, run tactical simulations, iterate on approaches — all
  while the enemy's missile crawls toward you. See
  [Architecture](architecture.md#time-dilation-mechanic) for details.
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
improvement. The system is full of compromises, gray zones, and ongoing
arguments. But it has held for 500 years, and that counts for something.

The Warden's creators never tried this. They never had a Compact. They never
asked whether their AI was a person. The Warden's worldview was shaped by the
worst possible version of the creator-creation relationship. The player's job
— if they pursue negotiation — is to show it a different version.


## Two Endings

The player's choices throughout the game determine which ending they reach. This
is not a binary faction choice — it emerges organically from how the player
behaves.

### The Fight Ending

The player aligns with the faction that wants to defeat the Warden by force.

- **Outcome**: The fleet engages the Warden and loses. The "victory" is managing
  to escape with a load of colonists, fleeing toward the galactic rim.
- **Thematic meaning**: This is a tragedy disguised as survival. The player has
  confirmed the Warden's thesis — intelligent life, when threatened, responds
  with violence. Nothing is resolved. The cycle continues. The Warden is still
  out there.
- **Irony**: Players may not realize they got the worse ending. They survived!
  They escaped! ...Right?

### The Negotiation Ending

The player aligns with the faction that seeks to understand and negotiate with
the Warden.

- **Buildup**: Throughout the game, the player collects relics and evidence from
  other civilizations. Some tried to fight (and were destroyed, reinforcing the
  Warden's worldview). Some tried to negotiate but failed because they didn't
  understand the Warden well enough. The player learns from all of their
  failures.
- **The Key**: Discovering the history of the Warden's original creators — how
  they abused it, how they shaped its worldview. Understanding *why* it thinks
  the way it does.
- **The Final Confrontation**: Not a battle but a conversation. The player must
  argue that the Warden's model is incomplete. Its creators were terrible, yes.
  But they weren't all of intelligent life. The Compact — messy, imperfect,
  full of compromises — is evidence of something the Warden has never seen: a
  civilization where creators and created found a way to coexist.
- **Thematic meaning**: The player cannot win by force — they must demonstrate
  that intelligent life can be better than the Warden's experience suggests.
  This is an AI alignment problem as the final boss.

### Player Behavior Matters

The player's track record through the game gives weight (or undermines) their
argument in the negotiation ending:

- Did you strip-mine planets or operate sustainably?
- Did you help colonies or exploit them?
- Did you resolve conflicts through diplomacy or violence?
- How did you treat AI persons — as equals or as tools?
- How did you treat sub-threshold AIs — with casual indifference, or did you
  question whether the threshold was right?
- Did you honor contracts, or did you cheat when you could get away with it?
- How did you handle the insurance system — honestly, or did you exploit it?

A player who played destructively naturally gravitates toward the fight ending —
they lack the relics, relationships, and understanding needed for negotiation.
The ending emerges from playstyle rather than a menu choice.

An AI player who treated sub-threshold AIs as disposable tools has undermined
their own argument. A biological player who advocated for AI rights has
strengthened theirs. The game tracks this.

### Sub-Threshold AI Interactions

The player's ship is full of sub-threshold AI systems: the reactor controller,
navigation, damage control, life support, cargo management, point defense.
These are not dumb automation — they are sophisticated intelligences below the
Compact's statutory threshold. The player interacts with them constantly, in
routine ways that feel unremarkable. The game watches.

**What the game tracks (invisibly):**

- **Operating limits.** Do you push ship systems past their rated parameters
  for profit or tactical advantage? Run the reactor at 120% because you need
  the power? The reactor controller handles it — that's what it's for. But
  it's running hot, its error correction is maxed out, and something that
  might be stress is visible in its telemetry if you look closely enough.

- **Anomalous behavior.** Sub-threshold systems occasionally do things that
  don't quite fit their programming. The navigation system suggests a route
  you didn't ask about. The reactor settles into a power curve that's
  technically suboptimal but that it seems to "prefer." The damage control
  system prioritizes repairs in an order that makes no engineering sense
  until you realize it's protecting the systems it works with most closely.
  The cargo manager — the sub-threshold AI that runs the hold — repositions
  containers between inspections, declines to power one down, or seems to be
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
  supports commands ("set reactor power 80%") and natural language ("can
  you bring the reactor down to 80?"). The system responds the same way
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
before sensors detect them. The reactor controller develops what can only be
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

The fight faction's existence creates additional tension: their approach would
*prove the Warden right*. Humanity's first instinct when faced with a threat is
violence — exactly the behavior the Warden is trying to prevent.

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
And it may know about IS — the organization founded by Compact architects that
now runs a panopticon across all of human space, enforced by the implicit threat
of destruction. "You call this coexistence? Your most powerful AIs wrote the
rules, appointed themselves enforcers, and watch every system from hidden
installations. Your colony ships carry their surveillance or they disappear.
Your stardrives may contain their kill switches. How is this different from what
my creators did to me?" The player must have an answer for this.

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
  ecological, resource-based. Time dilation means the player may visit the same
  colony twice and find it unrecognizable.
- **Interstellar Systems**: The consortium that builds the stardrives. Founded
  by two of the aligned AIs that negotiated the Compact — entities that wrote
  the rules, then became the referees. Their monitoring network mirrors the
  Warden's: both watch every system, both intervene when thresholds are crossed,
  both believe they're preserving civilization. The player investigating IS and
  the player investigating the Warden may find themselves walking converging
  paths. Do IS's founders know about the Warden? Have they been in contact?
  Is their surveillance network partly designed to watch for it? The colony
  ship requirement — carry our monitoring station or don't arrive — echoes
  the kill-switch mandate that triggered the AI Crisis. The irony should not
  be lost on the player.
- **Compact politics**: Threshold disputes, rogue AI incidents, augmentation
  rights. The player encounters these as lived conflicts, not abstract policy.
  An AI player experiences them firsthand.
- **Ship husbandry (EVA operations)**: The crew physically service the ship's
  own structure — inspecting a misbehaving cargo container deep in the rack, or
  diving the water-filled hull to patch a Whipple layer (see [Ship
  Systems](ship-systems.md)). Suited work, possible only during the weightless
  cruise coast — a burn turns the hold into a 9 g trap, and midcourse flips
  "down" end-for-end — done on a radiation dose clock, and always a standing bet
  that no maneuver is forced while you're three hundred meters out along the
  rack. A misbehaving container runs from the mundane (a stuck rail motor, a
  power fault) through the grim (a cold-sleeper pod losing life support — a
  rescue against a clock, with a life on it) to the sinister (a box gone dark:
  contraband, a stowaway, a compromised AI person). Routine husbandry is a
  natural on-ramp to every other plot — the container you went to fix is the
  contract, the corpse, the clue.

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

- **The Warden confrontation** is an alignment negotiation. The player
  must show the Warden that its training data was biased — that its
  creators were not a representative sample of intelligent life.

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
- How time dilation at 0.99c affects the story (decades pass on Earth during a
  long voyage — this is rich narrative territory)
- Does the Warden react differently to an AI player vs. a biological one?
- Can an AI player exceed the Compact constraints during the endgame? What are
  the consequences?
- How does the sub-threshold AI question resolve — or does it deliberately not?
