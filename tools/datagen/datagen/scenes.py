"""Load scene context definitions from TOML files."""

from pathlib import Path
from dataclasses import dataclass

try:
    import tomllib
except ImportError:
    import tomli as tomllib


@dataclass
class Scene:
    name: str
    description: str
    setting: str
    atmosphere: str


def load_scenes(path: Path) -> list[Scene]:
    """Load scenes from a TOML file."""
    with open(path, "rb") as f:
        data = tomllib.load(f)

    scenes = []
    for s in data.get("scenes", []):
        scenes.append(Scene(
            name=s["name"],
            description=s["description"],
            setting=s.get("setting", ""),
            atmosphere=s.get("atmosphere", ""),
        ))
    return scenes


def load_all_scenes(directory: Path) -> list[Scene]:
    """Load all scene TOML files."""
    scenes = []
    for path in sorted(directory.glob("*.toml")):
        scenes.extend(load_scenes(path))
    return scenes
