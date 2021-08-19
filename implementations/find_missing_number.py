"""Find Missing Number in Array

...

"""


def find_missing(array):
    s = sorted(array)
    m = s[0]

    for x in s:
        if x == m:
            continue
        elif x == m + 1:
            m += 1
        else:
            return m + 1
