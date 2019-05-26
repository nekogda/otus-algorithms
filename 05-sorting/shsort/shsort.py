from typing import List, Callable, TypeVar

T = TypeVar('T')


def _step_gen_shell(n: int) -> List[int]:
    steps = []
    while n > 1:
        n //= 2
        steps.append(n)
    return steps


def _step_gen_hibbard(n: int) -> List[int]:
    steps = []
    for i in range(1, n):
        step = 2 ** i - 1
        if step > n:
            break
        steps.append(step)

    steps.reverse()
    return steps


def _step_gen_pratt71(n: int) -> List[int]:
    steps = []
    for i in range(1, n):
        step = (3 ** i - 1) // 2
        if step > n//3:
            break
        steps.append(step)

    steps.reverse()
    return steps


def _step_gen_sedgewick86(n: int) -> List[int]:
    steps = [1]
    for i in range(1, n):
        step = 4 ** i + 3 * 2 ** (i - 1) + 1
        if step > n:
            break
        steps.append(step)

    steps.reverse()
    return steps


def _sort_group(s: List[T], grp_start: int, step: int) -> None:
    for idx in range(grp_start + step, len(s), step):
        tmp = s[idx]
        insertion_idx = idx - step
        while tmp < s[insertion_idx] and insertion_idx >= grp_start:
            s[insertion_idx + step] = s[insertion_idx]
            insertion_idx -= step
        s[insertion_idx + step] = tmp


def shell_sort(s: List[T], steps: Callable[[int], List[int]]) -> List[T]:
    for step in steps(len(s)):
        for grp_start in range(step):
            _sort_group(s, grp_start, step)
    return s
