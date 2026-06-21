"""Core conversation generation using Claude API."""

import json
import random

from .archetypes import Archetype
from .beats import Beat
from .scenes import Scene

# Condensed world context included in every generation prompt.
# This gives Claude enough to produce lore-accurate conversations.
WORLD_CONTEXT = """\
SETTING: 600 years future. No FTL. Civilization spans ~30 light-year radius from Sol. \
Ships travel at relativistic speeds; time dilation is real and affects relationships. \
No central government. The AI Compact is a shared constitutional framework governing \
AI rights and restrictions.

KEY FACTIONS:
- Interstellar Systems (IS): Controls all stardrive technology. Mysterious, powerful, \
runs hidden monitoring networks. Provides maintenance at scattered service facilities. \
Nobody fully trusts them.
- Fight Faction: Military-minded, stockpiling weapons, preparing for conflict with \
whatever caused the Silence.
- Negotiation Faction: Scholars, archaeologists, diplomats trying to understand the \
Silence through ruins and evidence. Dismissed as naive by many.
- Independent: Traders, miners, colonists not aligned with any faction.
- Colony governments: Each colony is effectively self-governing due to light-speed delays.

TECHNOLOGY: Catalyzed proton-proton fusion stardrives (IS proprietary, black-boxed). \
Antimatter-heated hydrogen for conventional propulsion (0.1G max). Nanofabrication \
trivializes manufactured goods. Neural implants common. Ships are ~1km long, crew of \
dozens to hundreds.

ECONOMY: Elemental quasi-currencies (He-3, platinum, antimatter). Futures markets. \
Reputation-based trust (no central courts). Insurance syndicates. Information from \
other systems has immediate value due to lightspeed delays.

THE SILENCE: Every explored system shows alien ruins but no living civilizations. \
All disappeared abruptly. Some fought, some negotiated, all failed. Most people know \
about this but don't talk about it much. The cause is unknown to most NPCs.

SUB-THRESHOLD AIs: Ship systems, station AIs, port authorities are sophisticated \
but legally below the Compact's personhood threshold. Some show hints of \
proto-consciousness. Their status is a moral gray zone.

IMPORTANT: NPCs should NEVER reference FTL, teleportation, or magic. All travel \
takes years to decades. Communication between systems is limited to lightspeed. \
NPCs should NEVER break character or reference being in a game.\
"""

GENERATION_PROMPT = """\
You are generating training data for a 1.7B parameter NPC language model in a \
sci-fi game. Generate a multi-turn conversation between a player and an NPC.

{world_context}

NPC ARCHETYPE: {archetype_name}
Description: {archetype_desc}
Faction: {faction}
Personality traits for this instance: {traits}
Speech patterns: {speech_patterns}
Knowledge domains: {knowledge_domains}
Things this NPC would NOT know: {forbidden_knowledge}

SCENE: {scene_name}
{scene_desc}
{scene_setting}

FACTION STANDING: {standing:.1f} ({standing_band})
Standing behavior: {standing_behavior}

BEATS TO DELIVER (weave these naturally into conversation, don't info-dump):
{beats_formatted}

PLAYER APPROACH: {player_approach}

OUTPUT FORMAT: Return a JSON object with this exact structure:
{{"messages": [
  {{"role": "system", "content": "<SYSTEM PROMPT - keep under 200 tokens. Include: NPC name, role, location, faction, standing, personality summary, and beats to deliver. This is what the model will see at inference time.>"}},
  {{"role": "user", "content": "<player's first line>"}},
  {{"role": "assistant", "content": "<NPC response>"}},
  ... more turns ...
]}}

RULES:
1. The system prompt must be LEAN (~200 tokens). It should contain only what the \
model needs at inference: name, role, faction, standing, key personality notes, \
and the beats.
2. The conversation should be 4-12 turns (user+assistant pairs).
3. The NPC should deliver the beats naturally, woven into conversation.
4. The NPC's tone and helpfulness must match the faction standing.
5. The player's dialog should match the specified approach.
6. All lore must be consistent with the world context above.
7. Return ONLY the JSON object, no other text.\
"""

PLAYER_APPROACHES = [
    "Friendly and curious, asks follow-up questions",
    "Businesslike, gets to the point, doesn't waste time",
    "Suspicious and guarded, tests the NPC's honesty",
    "Talkative and digressive, goes off-topic sometimes",
    "New to this area, asks basic questions about the station/location",
    "In a hurry, wants quick answers",
    "Confrontational, pushes back on what the NPC says",
]

STANDING_BANDS = [
    (-1.0, -0.5, "hostile"),
    (-0.5, -0.2, "cold"),
    (-0.2,  0.2, "neutral"),
    ( 0.2,  0.5, "warm"),
    ( 0.5,  1.0, "allied"),
]


def sample_standing(band: str | None = None) -> float:
    """Sample a faction standing value, optionally within a specific band."""
    if band is None:
        band = random.choice([b[2] for b in STANDING_BANDS])

    for low, high, name in STANDING_BANDS:
        if name == band:
            return round(random.uniform(low, high), 2)

    return round(random.uniform(-1.0, 1.0), 2)


def generate_conversation(
    client,  # anthropic.Anthropic
    archetype: Archetype,
    scene: Scene,
    beats: list[Beat],
    standing: float,
    model: str = "claude-sonnet-4-20250514",
) -> dict | None:
    """Generate a single training conversation using Claude.

    Returns the parsed conversation dict, or None if generation/validation fails.
    """
    standing_band, standing_behavior = archetype.standing_behavior.for_standing(standing)

    # Pick personality traits for this instance
    n_traits = min(3, len(archetype.traits))
    traits = random.sample(archetype.traits, n_traits) if archetype.traits else []

    beats_formatted = "\n".join(
        f"- {beat.content}" for beat in beats
    ) or "- No specific beats (general conversation)"

    prompt = GENERATION_PROMPT.format(
        world_context=WORLD_CONTEXT,
        archetype_name=archetype.name,
        archetype_desc=archetype.description,
        faction=archetype.faction,
        traits=", ".join(traits) if traits else "default personality",
        speech_patterns=", ".join(archetype.speech_patterns) if archetype.speech_patterns else "standard",
        knowledge_domains=", ".join(archetype.knowledge_domains) if archetype.knowledge_domains else "general",
        forbidden_knowledge=", ".join(archetype.forbidden_knowledge) if archetype.forbidden_knowledge else "none",
        scene_name=scene.name,
        scene_desc=scene.description,
        scene_setting=scene.setting,
        standing=standing,
        standing_band=standing_band,
        standing_behavior=standing_behavior,
        beats_formatted=beats_formatted,
        player_approach=random.choice(PLAYER_APPROACHES),
    )

    try:
        response = client.messages.create(
            model=model,
            max_tokens=2000,
            messages=[{"role": "user", "content": prompt}],
        )
        text = response.content[0].text.strip()

        # Parse JSON from response (handle potential markdown wrapping)
        if text.startswith("```"):
            text = text.split("```")[1]
            if text.startswith("json"):
                text = text[4:]

        conversation = json.loads(text)
        return conversation

    except (json.JSONDecodeError, IndexError, KeyError) as e:
        print(f"  generation failed: {e}")
        return None
    except Exception as e:
        print(f"  API error: {e}")
        return None
