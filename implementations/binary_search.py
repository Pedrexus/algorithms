
def bisect_in(sorted_array, n):
    size = len(sorted_array)

    index = size // 2
    value = sorted_array[index]

    while len(sorted_array) > 1:
        sorted_array = sorted_array[:index] if value > n else sorted_array[index:]

        index = len(sorted_array) // 2
        value = sorted_array[index]

        if value == n:
            return True

    return sorted_array[0] == n
