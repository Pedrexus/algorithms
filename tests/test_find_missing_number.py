from helpers.array import array
from implementations.find_missing_number import find_missing

def test_impl_1(benchmark):
    data = array(1000, have_negatives=False, can_repeat=False, to_remove=[10, 20], seed=1234)

    assert benchmark(lambda: find_missing(data)) == 10
