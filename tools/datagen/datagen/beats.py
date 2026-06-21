"""Load and select dialog beats from TOML files."""

from pathlib import Path
from dataclasses import dataclass
import random

try:
    import tomllib
except ImportError:
    import tomli as tomllib


@dataclass
class Beat:
    id: str
    category: str
    content: str
    variables: list[str]
    difficulty: str  # easy, medium, hard


def load_beats(path: Path) -> list[Beat]:
    """Load beats from a TOML file."""
    with open(path, "rb") as f:
        data = tomllib.load(f)

    beats = []
    for b in data.get("beats", []):
        beats.append(Beat(
            id=b["id"],
            category=b["category"],
            content=b["content"],
            variables=b.get("variables", []),
            difficulty=b.get("difficulty", "medium"),
        ))
    return beats


def load_all_beats(directory: Path) -> dict[str, list[Beat]]:
    """Load all beat TOML files, keyed by category."""
    categories: dict[str, list[Beat]] = {}
    for path in sorted(directory.glob("*.toml")):
        beats = load_beats(path)
        for beat in beats:
            categories.setdefault(beat.category, []).append(beat)
    return categories


def select_beats(
    categories: dict[str, list[Beat]],
    high_affinity: list[str],
    medium_affinity: list[str],
    low_affinity: list[str],
    count: int = 2,
    world_state: dict[str, str] | None = None,
) -> list[Beat]:
    """Select beats weighted by archetype affinity.

    High-affinity categories are 3x as likely, medium 2x, low 1x.
    Variables in beat content are filled from world_state.
    """
    pool: list[tuple[Beat, int]] = []
    for cat, beats in categories.items():
        if cat in high_affinity:
            weight = 3
        elif cat in medium_affinity:
            weight = 2
        elif cat in low_affinity:
            weight = 1
        else:
            continue
        for beat in beats:
            pool.append((beat, weight))

    if not pool:
        return []

    # Weighted sampling without replacement
    selected = []
    remaining = list(pool)
    for _ in range(min(count, len(remaining))):
        weights = [w for _, w in remaining]
        total = sum(weights)
        r = random.random() * total
        cumulative = 0
        for i, (beat, w) in enumerate(remaining):
            cumulative += w
            if r <= cumulative:
                selected.append(beat)
                remaining.pop(i)
                break

    # Fill variables from world_state
    if world_state:
        filled = []
        for beat in selected:
            content = beat.content
            for var in beat.variables:
                if var in world_state:
                    content = content.replace(f"{{{var}}}", world_state[var])
            filled.append(Beat(
                id=beat.id,
                category=beat.category,
                content=content,
                variables=beat.variables,
                difficulty=beat.difficulty,
            ))
        return filled

    return selected
