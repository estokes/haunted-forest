"""Quality validation for generated conversations."""


# Lore violations to check for
LORE_VIOLATIONS = [
    "faster than light",
    "FTL",
    "warp drive",
    "hyperspace",
    "hyperdrive",
    "teleport",
    "beam me",
    "as an AI",
    "as a language model",
    "I cannot",  # AI refusal patterns
    "I'm sorry, but",
    "I don't have personal",
]


def validate_conversation(conv: dict) -> list[str]:
    """Validate a generated conversation. Returns list of issues (empty = valid)."""
    issues = []

    messages = conv.get("messages", [])
    if not messages:
        issues.append("no messages")
        return issues

    # Check structure
    if messages[0].get("role") != "system":
        issues.append("first message must be system")

    roles = [m.get("role") for m in messages]
    if len(roles) < 3:
        issues.append(f"too few messages: {len(roles)} (need at least system + 1 turn)")

    # Check role alternation after system
    for i in range(1, len(roles) - 1):
        if roles[i] == roles[i + 1]:
            issues.append(f"consecutive {roles[i]} messages at positions {i},{i+1}")

    # Count turns (user+assistant pairs)
    turns = sum(1 for r in roles if r == "assistant")
    if turns < 2:
        issues.append(f"too few turns: {turns} (need at least 2)")
    if turns > 12:
        issues.append(f"too many turns: {turns} (max 12)")

    # Check system prompt length (rough token estimate: ~1.3 tokens per word)
    system_content = messages[0].get("content", "")
    word_count = len(system_content.split())
    token_estimate = int(word_count * 1.3)
    if token_estimate > 250:
        issues.append(f"system prompt too long: ~{token_estimate} tokens (max 250)")

    # Check for empty content
    for i, msg in enumerate(messages):
        if not msg.get("content", "").strip():
            issues.append(f"empty content at position {i}")

    # Check for lore violations in assistant messages
    for i, msg in enumerate(messages):
        if msg.get("role") != "assistant":
            continue
        content = msg.get("content", "").lower()
        for violation in LORE_VIOLATIONS:
            if violation.lower() in content:
                issues.append(f"lore violation at position {i}: '{violation}'")

    return issues
