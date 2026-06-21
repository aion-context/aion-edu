"""Boolean algebra and tautology — STARTER (the laws of thought).

A formula over variables is a function f(env) -> bool, where env maps each
variable name to True/False. `all_envs(vars)` yields every assignment.
is_tautology(f, vars): f is True under EVERY assignment. is_satisfiable: True
under SOME. Fix `is_tautology` and `is_satisfiable`.
"""
import itertools


def all_envs(variables):
    for bits in itertools.product([False, True], repeat=len(variables)):
        yield dict(zip(variables, bits))


def is_tautology(f, variables):
    # True iff f holds under EVERY assignment.
    # BUG: returns f of only the first assignment (return inside the loop).
    for env in all_envs(variables):
        return f(env)


def is_satisfiable(f, variables):
    # True iff f holds under SOME assignment.
    # BUG: returns f of only the first assignment.
    for env in all_envs(variables):
        return f(env)


def is_contradiction(f, variables):
    return not is_satisfiable(f, variables)
