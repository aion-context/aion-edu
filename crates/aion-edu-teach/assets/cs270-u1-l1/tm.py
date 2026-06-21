"""A Turing machine — STARTER (the step function is incomplete).

A configuration is (tape: dict mapping position -> symbol, head: int, state: str).
A transition table maps (state, symbol) -> (new_symbol, move, new_state) where
move is +1 (right) or -1 (left). The blank symbol is '_'. Implement `step` to apply
one transition (write the symbol, move the head, change state); `run` steps until
the state is 'HALT'. With a correct step, INCREMENT_TABLE turns '1011' into '1100'.
"""
BLANK = "_"


def step(tape, head, state, table):
    sym = tape.get(head, BLANK)
    if (state, sym) not in table:
        return tape, head, "HALT"
    new_sym, move, new_state = table[(state, sym)]
    # BUG: writes nothing and never moves the head.
    return tape, head, new_state


def run(tape, head, state, table, max_steps=10000):
    for _ in range(max_steps):
        if state == "HALT":
            break
        tape, head, state = step(tape, head, state, table)
    return tape, head, state


def to_tape(s):
    return {i: c for i, c in enumerate(s)}


def tape_str(tape):
    keys = [k for k, v in tape.items() if v != BLANK]
    if not keys:
        return ""
    lo, hi = min(keys), max(keys)
    return "".join(tape.get(i, BLANK) for i in range(lo, hi + 1))


# Binary incrementer: walk to the right end, then carry leftward.
INCREMENT_TABLE = {
    ("GOTO_END", "0"): ("0", +1, "GOTO_END"),
    ("GOTO_END", "1"): ("1", +1, "GOTO_END"),
    ("GOTO_END", "_"): ("_", -1, "CARRY"),
    ("CARRY", "1"): ("0", -1, "CARRY"),
    ("CARRY", "0"): ("1", 0, "HALT"),
    ("CARRY", "_"): ("1", 0, "HALT"),
}
