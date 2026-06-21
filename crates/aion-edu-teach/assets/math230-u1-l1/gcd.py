"""The Euclidean algorithm and modular inverses — STARTER.

gcd(a,b) = gcd(b, a mod b) until b = 0 (given below, correct). The EXTENDED
algorithm also finds x, y with a*x + b*y = gcd(a,b) (Bézout). Then if gcd(a,m) = 1,
x mod m is the modular inverse of a (a*x ≡ 1 mod m). Fix `ext_gcd` so its
coefficients satisfy the Bézout identity.
"""


def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a


def ext_gcd(a, b):
    # return (g, x, y) with a*x + b*y = g = gcd(a, b)
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = ext_gcd(b, a % b)
    # BUG: forgets to recombine the coefficients for this level.
    return (g, x1, y1)


def mod_inverse(a, m):
    g, x, _ = ext_gcd(a % m, m)
    if g != 1:
        return None
    return x % m
