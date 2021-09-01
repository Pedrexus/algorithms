"""Find Missing Number in Array

    Given an array of integers, find the lowest missing number in the array between
min and max value.

Examples:

    array = [1, 5, 3, 2]
    missing = 4

    array = [-2, -1, 0, 1, 1, 1, 2, 3, 5]
    missing = 4

    array = [-1, 0, 1]
    missing = None
"""
from helpers.array import min_max
from implementations.binary_search import bisect_in


def find_missing_1(sorted_array):
    """Sorting implementation

    Complexity:

        Time: `sorted` complexity = O(n log n)
        Space: ~2n

    Limitations:
        - array must be sorted
    """
    m = sorted_array[0]

    for x in sorted_array:
        if x == m:
            continue
        elif x == m + 1:
            m += 1
        else:
            return m + 1


def find_missing_2(sorted_array):
    """Sorting implementation

        Complexity:

            Time: `sorted` complexity + n log n = O(n log n)
            Space: ~2n (copies array)

        Limitations:
            - array must be sorted
        """
    for n in range(sorted_array[0], sorted_array[-1]):
        if not bisect_in(sorted_array, n):
            return n


def find_missing_3(array):
    _min, _max = min_max(array)

    # try:
    #     return min(x for x in range(_min + 1, _max) if x not in set(array))
    # except ValueError:
    #     return

    # _min_missing = float('inf')
    if all_missing := (set(range(_min + 1, _max)) - set(array)):  # O(n) - hashset can perform O(1) "in"
        return min(all_missing)
