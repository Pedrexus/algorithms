from numbers import Real

import numpy as np


def array(
        n: int,
        limit: Real = 1,
        have_negatives: bool = True,
        can_repeat: bool = True,
        to_remove: list[Real] = (),
        seed: int | None = None
) -> list[int]:
    rng = np.random.default_rng(seed=seed)

    limit = abs(int(limit))
    limit = limit if n < limit else n

    low = -limit if have_negatives else 0
    high = +limit

    if can_repeat:
        array = rng.integers(low=low, high=high, size=n)
    else:
        array = rng.permutation(np.arange(low, high))[:n]

    if to_remove:
        array = np.delete(array, np.in1d(array, to_remove))

    return list(array)


def min_max(array: list[Real]) -> (Real, Real):
    _min = float('inf')
    _max = -_min

    for x in array:
        if x < _min:
            _min = x

        if x > _max:
            _max = x

    return _min, _max
