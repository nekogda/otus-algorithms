from typing import List, TypeVar

T = TypeVar('T')


def insertion_sort(s: List[T]) -> List[T]:
    for idx in range(len(s)):
        tmp = s[idx]
        insertion_idx = idx - 1
        while tmp < s[insertion_idx] and insertion_idx >= 0:
            s[insertion_idx + 1] = s[insertion_idx]
            insertion_idx -= 1
        s[insertion_idx + 1] = tmp
    return s
