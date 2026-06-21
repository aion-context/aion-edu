"""Mastery check for logic101-u1-l1 — Boolean algebra and tautology."""
from boole import is_tautology, is_satisfiable, is_contradiction


def excluded_middle(env):
    return env["p"] or (not env["p"])


def contradiction(env):
    return env["p"] and (not env["p"])


def demorgan(env):
    p, q = env["p"], env["q"]
    return (not (p and q)) == ((not p) or (not q))


def both(env):
    return env["p"] and env["q"]   # satisfiable, not a tautology


def not_p(env):
    return not env["p"]            # satisfiable, not a tautology (true first env)


def test_excluded_middle_and_demorgan_are_tautologies():
    assert is_tautology(excluded_middle, ["p"])
    assert is_tautology(demorgan, ["p", "q"])


def test_contradiction():
    assert is_contradiction(contradiction, ["p"])
    assert not is_satisfiable(contradiction, ["p"])


def test_satisfiable_not_tautology():
    assert is_satisfiable(both, ["p", "q"])
    assert not is_tautology(both, ["p", "q"])
    assert is_satisfiable(not_p, ["p"])
    assert not is_tautology(not_p, ["p"])   # true on the first env, false later
