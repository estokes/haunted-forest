"""CLI for NPC training data generation."""

import argparse
import json
import random
import sys
from pathlib import Path

from .archetypes import load_all_archetypes
from .beats import load_all_beats, select_beats
from .scenes import load_all_scenes
from .generate import generate_conversation, sample_standing, STANDING_BANDS
from .validate import validate_conversation


def cmd_generate(args):
    """Generate conversations for one or all archetypes."""
    base_dir = Path(__file__).parent.parent

    # Load resources
    archetypes = load_all_archetypes(base_dir / "archetypes")
    if not archetypes:
        print("error: no archetypes found in archetypes/", file=sys.stderr)
        sys.exit(1)

    all_beats = load_all_beats(base_dir / "beats")
    scenes = load_all_scenes(base_dir / "scenes")
    if not scenes:
        print("error: no scenes found in scenes/", file=sys.stderr)
        sys.exit(1)

    # Filter archetypes
    if args.archetype:
        if args.archetype not in archetypes:
            print(f"error: unknown archetype '{args.archetype}'", file=sys.stderr)
            print(f"available: {', '.join(archetypes.keys())}", file=sys.stderr)
            sys.exit(1)
        targets = {args.archetype: archetypes[args.archetype]}
    else:
        targets = archetypes

    import anthropic
    client = anthropic.Anthropic()
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    total_generated = 0
    total_failed = 0

    with open(output_path, "a") as f:
        for arch_name, archetype in targets.items():
            print(f"\n=== {arch_name} ===")

            # Generate across all standing bands
            per_band = max(1, args.count // len(STANDING_BANDS))

            for _, _, band_name in STANDING_BANDS:
                print(f"  standing band: {band_name} ({per_band} conversations)")

                for i in range(per_band):
                    standing = sample_standing(band_name)
                    scene = random.choice(scenes)

                    beats = select_beats(
                        all_beats,
                        archetype.beat_affinity_high,
                        archetype.beat_affinity_medium,
                        archetype.beat_affinity_low,
                        count=args.beats,
                        world_state=args.world_state,
                    )

                    conv = generate_conversation(
                        client,
                        archetype,
                        scene,
                        beats,
                        standing,
                        model=args.model,
                    )

                    if conv is None:
                        total_failed += 1
                        continue

                    issues = validate_conversation(conv)
                    if issues:
                        print(f"    [{i+1}] REJECTED: {'; '.join(issues)}")
                        total_failed += 1
                        continue

                    f.write(json.dumps(conv) + "\n")
                    f.flush()
                    total_generated += 1
                    print(f"    [{i+1}] ok (standing={standing:.2f}, scene={scene.name}, "
                          f"{len(conv['messages'])} messages)")

    print(f"\ndone: {total_generated} generated, {total_failed} failed/rejected")
    print(f"output: {output_path}")


def cmd_validate(args):
    """Validate an existing JSONL file."""
    path = Path(args.file)
    if not path.exists():
        print(f"error: file not found: {path}", file=sys.stderr)
        sys.exit(1)

    total = 0
    valid = 0
    with open(path) as f:
        for i, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            total += 1
            try:
                conv = json.loads(line)
            except json.JSONDecodeError as e:
                print(f"  line {i}: JSON parse error: {e}")
                continue

            issues = validate_conversation(conv)
            if issues:
                print(f"  line {i}: {'; '.join(issues)}")
            else:
                valid += 1

    print(f"\n{valid}/{total} valid conversations")


def cmd_stats(args):
    """Print statistics about a JSONL conversation file."""
    path = Path(args.file)
    if not path.exists():
        print(f"error: file not found: {path}", file=sys.stderr)
        sys.exit(1)

    total = 0
    total_turns = 0
    total_words = 0
    standing_counts: dict[str, int] = {}

    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                conv = json.loads(line)
            except json.JSONDecodeError:
                continue

            total += 1
            messages = conv.get("messages", [])
            turns = sum(1 for m in messages if m.get("role") == "assistant")
            total_turns += turns

            words = sum(len(m.get("content", "").split()) for m in messages)
            total_words += words

            # Try to extract standing from system prompt
            system = messages[0].get("content", "") if messages else ""
            for band_name in ["hostile", "cold", "neutral", "warm", "allied"]:
                if f"({band_name})" in system.lower():
                    standing_counts[band_name] = standing_counts.get(band_name, 0) + 1
                    break

    print(f"conversations: {total}")
    print(f"total turns: {total_turns} (avg {total_turns/max(total,1):.1f})")
    print(f"total words: {total_words} (avg {total_words/max(total,1):.0f})")
    print(f"standing distribution: {standing_counts}")


def main():
    parser = argparse.ArgumentParser(description="NPC training data generation")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # generate
    gen = subparsers.add_parser("generate", help="Generate conversations")
    gen.add_argument("--archetype", help="Generate for a specific archetype (default: all)")
    gen.add_argument("--count", type=int, default=25,
                     help="Conversations per archetype (distributed across standing bands)")
    gen.add_argument("--beats", type=int, default=2,
                     help="Number of beats per conversation")
    gen.add_argument("--output", default="../../data/dialog/generated.jsonl",
                     help="Output JSONL file path")
    gen.add_argument("--model", default="claude-sonnet-4-20250514",
                     help="Claude model to use for generation")
    gen.set_defaults(func=cmd_generate, world_state=None)

    # validate
    val = subparsers.add_parser("validate", help="Validate a JSONL file")
    val.add_argument("file", help="JSONL file to validate")
    val.set_defaults(func=cmd_validate)

    # stats
    st = subparsers.add_parser("stats", help="Print statistics about a JSONL file")
    st.add_argument("file", help="JSONL file to analyze")
    st.set_defaults(func=cmd_stats)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
