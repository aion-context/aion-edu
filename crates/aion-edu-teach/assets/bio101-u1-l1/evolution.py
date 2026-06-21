"""Selection and allele frequencies — STARTER.

Two alleles A (frequency p) and a (frequency q = 1 - p). Under random mating the
genotype frequencies are Hardy-Weinberg: AA = p², Aa = 2pq, aa = q². With relative
fitnesses (w_AA, w_Aa, w_aa), one generation of selection changes p to
    p' = (p²·w_AA + p·q·w_Aa) / w_bar,
    w_bar = p²·w_AA + 2pq·w_Aa + q²·w_aa.
Fix `hardy_weinberg` (the heterozygote) and `next_p` (the heterozygote term).
"""


def hardy_weinberg(p):
    q = 1 - p
    # (AA, Aa, aa).  BUG: forgets the factor of 2 on the heterozygote.
    return (p * p, p * q, q * q)


def mean_fitness(p, w):
    aa_, ab_, bb_ = hardy_weinberg(p)
    w_aa, w_ab, w_bb = w
    return aa_ * w_aa + ab_ * w_ab + bb_ * w_bb


def next_p(p, w):
    q = 1 - p
    w_aa, w_ab, _w_bb = w
    w_bar = mean_fitness(p, w)
    # p' = (p²·w_AA + p·q·w_Aa) / w_bar.  BUG: drops the heterozygote contribution.
    return (p * p * w_aa) / w_bar
