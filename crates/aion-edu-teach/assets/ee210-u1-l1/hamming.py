"""Hamming(7,4) single-error correction — STARTER (syndrome not implemented).

4 data bits -> 7-bit codeword with 3 parity bits, so any single-bit error can be
both detected AND corrected. Positions 1..7 laid out as [p1, p2, d1, p4, d2, d3, d4].
The syndrome (recomputed parities read as a binary number) is the 1-based position
of the flipped bit, or 0 if none. Fix `syndrome`.
"""


def encode(data):
    d1, d2, d3, d4 = data
    p1 = d1 ^ d2 ^ d4
    p2 = d1 ^ d3 ^ d4
    p4 = d2 ^ d3 ^ d4
    return [p1, p2, d1, p4, d2, d3, d4]


def syndrome(code):
    # Should recompute each parity over its position-set and combine the three
    # check bits as s1 + 2*s2 + 4*s4 -> the 1-based error position (0 if clean).
    # BUG: always reports "no error".
    return 0


def decode(code):
    s = syndrome(code)
    fixed = code[:]
    if s != 0:
        fixed[s - 1] ^= 1
    return [fixed[2], fixed[4], fixed[5], fixed[6]]  # data bits at positions 3,5,6,7
