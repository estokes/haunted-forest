"""Load and manage NPC archetype definitions from TOML files."""

from pathlib import Path
from dataclasses import dataclass, field

try:
    import tomllib
except ImportError:
    import tomli as tomllib


@dataclass
class StandingBehavior:
    hostile: str   # -1.0 to -0.5
    cold: str      # -0.5 to -0.2
    neutral: str   # -0.2 to 0.2
    warm: str      #  0.2 to 0.5
    allied: str    #  0.5 to 1.0

    def for_standing(self, standing: float) -> tuple[str, str]:
        """Return (band_name, behavior_description) for a given standing value."""
        if standing <= -0.5:
            return ("hostile", self.hostile)
        elif standing <= -0.2:
            return ("cold", self.cold)
        elif standing <= 0.2:
            return ("neutral", self.neutral)
        elif standing <= 0.5:
            return ("warm", self.warm)
        else:
            return ("allied", self.allied)


@dataclass
class Archetype:
    name: str
    description: str
    faction: str
    traits: list[str]
    speech_patterns: list[str]
    knowledge_domains: list[str]
    forbidden_knowledge: list[str]
    beat_affinity_high: list[str]
    beat_affinity_medium: list[str]
    beat_affinity_low: list[str]
    standing_behavior: StandingBehavior


def load_archetype(path: Path) -> Archetype:
    """Load a single archetype from a TOML file."""
    with open(path, "rb") as f:
        data = tomllib.load(f)

    arch = data["archetype"]
    pers = data["personality"]
    cons = data.get("constraints", {})
    aff = data.get("beat_affinity", {})
    sb = data.get("standing_behavior", {})

    return Archetype(
        name=arch["name"],
        description=arch["description"],
        faction=arch.get("faction", "independent"),
        traits=pers.get("traits", []),
        speech_patterns=pers.get("speech_patterns", []),
        knowledge_domains=pers.get("knowledge_domains", []),
        forbidden_knowledge=cons.get("forbidden_knowledge", []),
        beat_affinity_high=aff.get("high", []),
        beat_affinity_medium=aff.get("medium", []),
        beat_affinity_low=aff.get("low", []),
        standing_behavior=StandingBehavior(
            hostile=sb.get("hostile", "Hostile and uncooperative"),
            cold=sb.get("cold", "Terse and unhelpful"),
            neutral=sb.get("neutral", "Professional and businesslike"),
            warm=sb.get("warm", "Friendly and helpful"),
            allied=sb.get("allied", "Enthusiastic and generous"),
        ),
    )


def load_all_archetypes(directory: Path) -> dict[str, Archetype]:
    """Load all archetype TOML files from a directory."""
    archetypes = {}
    for path in sorted(directory.glob("*.toml")):
        arch = load_archetype(path)
        archetypes[arch.name] = arch
    return archetypes
