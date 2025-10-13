import numpy as np

SEED = 0
RNG = np.random.default_rng(SEED)


def exp(mean, sample_size=1):
    if sample_size == 1:
        return RNG.exponential(mean)
    else:
        return RNG.exponential(mean, sample_size)


def normal(mean, dev, sample_size=1):
    if sample_size == 1:
        return RNG.normal(mean)
    else:
        return RNG.normal(mean, sample_size)


def uniform(mean, dev, sample_size=1):
    if sample_size == 1:
        return RNG.uniform(mean)
    else:
        return RNG.uniform(mean, sample_size)


def erlang(mean, dev, sample_size=1):
    if sample_size == 1:
        return RNG.gamma(mean)
    else:
        return RNG.gamma(mean, sample_size)
