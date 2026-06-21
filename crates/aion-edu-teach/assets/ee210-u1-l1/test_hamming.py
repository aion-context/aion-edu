"""Mastery check for ee210-u1-l1 — Hamming(7,4)."""
from hamming import encode, syndrome, decode


def test_roundtrip_no_error():
    for data in [[0, 0, 0, 0], [1, 0, 1, 1], [1, 1, 1, 1], [0, 1, 0, 1]]:
        code = encode(data)
        assert syndrome(code) == 0
        assert decode(code) == data


def test_single_error_located_and_corrected():
    data = [1, 0, 1, 1]
    code = encode(data)
    for i in range(7):
        bad = code[:]
        bad[i] ^= 1
        assert syndrome(bad) == i + 1   # the syndrome is the 1-based flip position
        assert decode(bad) == data      # and the flip is corrected
