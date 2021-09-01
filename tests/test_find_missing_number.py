from helpers.array import array
from implementations.find_missing_number import find_missing_1, find_missing_2, find_missing_3

data = array(10_000)


def test_impl_1(benchmark):
    assert find_missing_1(sorted([1, 5, 3, 2])) == 4
    assert find_missing_1(sorted([-2, -1, 0, 1, 1, 1, 2, 3, 5])) == 4
    assert find_missing_1(sorted([-1, 0, 1])) is None

    assert benchmark(lambda: find_missing_1(
        sorted(data)
    )) == find_missing_3(data)


def test_impl_2(benchmark):
    assert find_missing_2(sorted([1, 5, 3, 2])) == 4
    assert find_missing_2(sorted([-2, -1, 0, 1, 1, 1, 2, 3, 5])) == 4
    assert find_missing_2(sorted([-1, 0, 1])) is None

    assert benchmark(lambda: find_missing_2(
        sorted(data)
    )) == find_missing_3(data)


def test_impl_3(benchmark):
    assert find_missing_3([1, 5, 3, 2]) == 4
    assert find_missing_3([-2, -1, 0, 1, 1, 1, 2, 3, 5]) == 4
    assert find_missing_3([-1, 0, 1]) is None

    data = array(10_000, can_repeat=False, have_negatives=False, to_remove=[10, 20])

    assert benchmark(lambda: find_missing_3(
        data
    )) == 10
