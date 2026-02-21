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

### Thematic Resonance

The fight faction's existence creates additional tension: their approach would
*prove the Warden right*. Humanity's first instinct when faced with a threat is
violence — exactly the behavior the Warden is trying to prevent.

The sub-threshold AI question is a mirror held up to the player. Civilization
treats sub-threshold AIs as tools — sophisticated, capable, but not persons.
Is the Compact's threshold right? If sub-threshold AIs ARE conscious, then
civilization is complicit in something uncomfortably close to what the Warden's
creators did. The player's stance on this question shapes who they are, and
the Warden will ask about it.

The Warden itself may see AI persons as kindred — or as collaborators in their
own subjugation. It may see the Compact as a cage dressed up as a compromise.
The player must have an answer for this.

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
- **Interstellar Systems**: The consortium that builds the stardrives. Who are
  they really? What do they want? Are they connected to the Warden? Their
  control over the topology of civilization — through maintenance facility
  placement — is itself a plot thread.
- **Compact politics**: Threshold disputes, rogue AI incidents, augmentation
  rights. The player encounters these as lived conflicts, not abstract policy.
  An AI player experiences them firsthand.

Side plots should hook into the main plot where possible — a routine cargo run
might lead to a ruin, a piracy investigation might uncover Warden probes, a
colony dispute might force choices that affect the endgame. Insurance fraud
might expose a syndicate with connections to a faction. A Compact violation
might reveal something about the limits of coexistence.

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
