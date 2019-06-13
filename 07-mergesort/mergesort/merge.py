from typing import TypeVar, List, Generator, Tuple
from threading import Thread
from operator import lt, ge
from bisect import bisect_left, bisect_right


T = TypeVar('T')
MINRUN = 64
GALLOPING = 7


def insertion_sort(s: List[T], begin, end) -> List[T]:
    for idx in range(begin, end):
        tmp = s[idx]
        insertion_idx = idx - 1
        while tmp < s[insertion_idx] and insertion_idx >= begin:
            s[insertion_idx + 1] = s[insertion_idx]
            insertion_idx -= 1
        s[insertion_idx + 1] = tmp
    return s


def merge_sort_insertion(s: List[T]) -> List[T]:
    cp = s[:]
    split_merge_insertion(0, len(s), s, cp)
    return s


def split_merge_insertion(begin: int, end: int, s: List[T], cp) -> None:
    if end - begin < 16:
        insertion_sort(s, begin, end)
        return
    middle = begin + (end - begin) // 2
    split_merge_insertion(begin, middle, s, cp)
    split_merge_insertion(middle, end, s, cp)
    merge_base((begin, middle), (middle, end), s, cp)


def merge_sort_threaded(s: List[T]) -> List[T]:
    cp = s[:]
    t1 = Thread(target=split_merge_base, args=(0, len(s)//2, s, cp))
    t2 = Thread(target=split_merge_base, args=(len(s)//2, len(s), s, cp))
    t1.start()
    t2.start()
    t1.join()
    t2.join()
    merge_base((0, len(s)//2), (len(s)//2, len(s)), s, cp)
    return s


def merge_sort_base(s: List[T]) -> List[T]:
    cp = s[:]
    split_merge_base(0, len(s), s, cp)
    return s


def split_merge_base(begin: int, end: int, s: List[T], cp) -> None:
    if end - begin < 2:
        return
    middle = begin + (end - begin) // 2
    split_merge_base(begin, middle, s, cp)
    split_merge_base(middle, end, s, cp)
    merge_base((begin, middle), (middle, end), s, cp)


def merge_base(lpart, rpart, s, cp) -> None:
    lb, le = lpart
    rb, re = rpart
    # copy
    for i in range(lb, re):
        cp[i] = s[i]

    for i in range(lb, re):
        if rb == re or (lb != le and cp[lb] <= cp[rb]):
            s[i] = cp[lb]
            lb += 1
            continue
        if lb == le or (rb != re and cp[rb] < cp[lb]):
            s[i] = cp[rb]
            rb += 1


def merge_sort_tim(s: List[T]) -> List[T]:
    stack = []
    if len(s) < 2:
        return s
    for run in get_runs(s):
        stack.append(run)
        merge_collapse(s, stack)
    merge_collapse(s, stack, last_run=True)
    return s


class Run:
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __len__(self):
        return self.end - self.start

    def __repr__(self):
        return f'Run(start={self.start}, end={self.end})'

    def __lt__(self, other):
        return len(self) < len(other)

    def __gt__(self, other):
        return len(self) > len(other)

    def __le__(self, other):
        return len(self) <= len(other)

    def __ge__(self, other):
        return len(self) >= len(other)

    def __eq__(self, other):
        return len(self) == len(other)


def get_minrun(s: List[T], start: int, stop: int) -> 'Run':
    run_len = stop - start
    remaining = MINRUN - run_len
    if remaining >= len(s) - stop:
        end = len(s)
    else:
        end = stop + remaining
    return Run(start, end)


def reverse(s: List[T], start: int, stop: int) -> None:
    for i in range((stop - start) // 2):
        s[start + i], s[stop - 1 - i] = s[stop - 1 - i], s[start + i]


def get_run(s: List[T], start: int) -> 'Run':
    assert start >= 0 and start < len(s)
    current = start + 1
    # Corner case. When Run size is 1-2 at the end of 's'
    if len(s) - current < 2:
        run = Run(start, len(s))
        insertion_sort(s, run.start, run.end)
        return run

    # Check if sequence in ascending order
    if s[current] >= s[current - 1]:
        # all next elements must be greater or equal then previous
        op = ge
        desc = False
    # If not, sequence in descending order
    else:
        # all next elements must be less then previous
        op = lt
        desc = True

    current += 1

    run = None
    # Main loop. Collecting run.
    while current < len(s):
        if op(s[current], s[current - 1]):
            current += 1
        # Need more elements up to minrun?
        elif current - start < MINRUN:
            run = get_minrun(s, start, current)
            insertion_sort(s, run.start, run.end)
            desc = False
            break
        # We have collected num of elements >= MINRUN.
        else:
            break
    # Reverse ordered sequence if we find them in descending order
    if desc:
        reverse(s, start, current)
    return run or Run(start, current)


def get_runs(s: List[T]) -> Generator['Run', None, None]:
    start = 0
    while True:
        run = get_run(s, start)
        yield run
        if run.end == len(s):
            break
        start = run.end


def move(s: List[T], tmp: List[T], hi_move: bool,
         rb: int, re: int, lb: int, le: int, i: int) -> int:
    if hi_move:
        if rb == re or (lb != le and tmp[re - 1] < s[le - 1]):
            s[i] = s[le - 1]
            return -1
        elif lb == le or (rb != re and s[le - 1] <= tmp[re - 1]):
            s[i] = tmp[re - 1]
            return 1
    else:
        if rb == re or (lb != le and tmp[lb] <= s[rb]):
            s[i] = tmp[lb]
            return 1
        elif lb == le or (rb != re and s[rb] < tmp[lb]):
            s[i] = s[rb]
            return -1


def move_galloping(
        src: List[T], dst: List[T],
        dst_start: int, src_start: int, src_stop: int, step: int
) -> Tuple[int, int]:
    assert step == 1 or step == -1
    op = lt if step == 1 else ge

    while op(src_start, src_stop):
        dst[dst_start] = src[src_start]
        dst_start += step
        src_start += step

    if step == -1:
        src_start += 1
    return (dst_start, src_start)


def merge_lo(s: List[T], run_a: 'Run', run_b: 'Run') -> 'Run':
    tmp = s[run_a.start:run_a.end]
    lb, le = 0, len(tmp)
    rb, re = run_b.start, run_b.end
    run_a_counter = 0
    run_b_counter = 0

    i = run_a.start
    while i < run_b.end:
        if run_a_counter > GALLOPING:
            if rb != re:
                idx = bisect_left(tmp, s[rb])
            else:
                idx = le
            i, lb = move_galloping(
                src=tmp, dst=s,
                dst_start=i,
                src_start=lb,
                src_stop=idx,
                step=1,
            )
            run_a_counter = 0
            run_b_counter = 0
        elif run_b_counter > GALLOPING:
            if lb != le:
                idx = bisect_left(s, tmp[lb], run_b.start, run_b.end)
            else:
                idx = re
            i, rb = move_galloping(
                src=s, dst=s,
                dst_start=i,
                src_start=rb,
                src_stop=idx,
                step=1,
            )
            run_a_counter = 0
            run_b_counter = 0

        if rb == re and lb == le:
            break

        if move(s, tmp, False, rb, re, lb, le, i) > 0:
            lb += 1
            run_a_counter += 1
            run_b_counter = 0
        else:
            rb += 1
            run_a_counter = 0
            run_b_counter += 1
        i += 1
    return Run(run_a.start, run_b.end)


def merge_hi(s: List[T], run_a: 'Run', run_b: 'Run') -> 'Run':
    tmp = s[run_b.start:run_b.end]
    lb, le = run_a.start, run_a.end
    rb, re = 0, len(tmp)
    run_a_counter = 0
    run_b_counter = 0

    i = run_b.end - 1
    while i >= run_a.start:
        if run_a_counter > GALLOPING:
            if rb != re:
                idx = bisect_right(s, tmp[re - 1], run_a.start, run_a.end)
                if idx == run_a.end:
                    idx = lb
            else:
                idx = lb
            i, le = move_galloping(
                src=s, dst=s,
                dst_start=i,
                src_start=le - 1,
                src_stop=idx,
                step=-1,
            )
            run_a_counter = 0
            run_b_counter = 0
        elif run_b_counter > GALLOPING:
            if lb != le:
                idx = bisect_right(tmp, s[le - 1])
                if idx == run_b.end:
                    idx = rb
            else:
                idx = rb

            i, re = move_galloping(
                src=tmp, dst=s,
                dst_start=i,
                src_start=re - 1,
                src_stop=idx,
                step=-1,
            )
            run_a_counter = 0
            run_b_counter = 0

        if rb == re and lb == le:
            break

        if move(s, tmp, True, rb, re, lb, le, i) < 0:
            le -= 1
            run_a_counter += 1
            run_b_counter = 0
        else:
            re -= 1
            run_a_counter = 0
            run_b_counter += 1

        i -= 1
    return Run(run_a.start, run_b.end)


def check_invariants(stack: List[T]) -> bool:
    """
    Return True if all invariants is OK, else False.
    """
    assert len(stack) >= 2
    if len(stack) == 2:
        run_c = Run(0, 0)
        run_a, run_b = stack[-2:]
    else:
        run_a, run_b, run_c = stack[-3:]

    if len(run_a) > len(run_b) + len(run_c) and \
       len(run_b) > len(run_c):
        return True
    else:
        return False


def merge_collapse(s: List[T], stack: List[T], last_run: bool = False) -> None:
    while len(stack) >= 2:
        if check_invariants(stack) and last_run is False:
            # Nothing to merge, just exit.
            break
        elif len(stack) == 2:
            run_c = None
        else:
            run_c = stack.pop()
        run_b = stack.pop()
        run_a = stack.pop()

        if run_c is None or run_a < run_c:
            if run_a < run_b:
                stack.append(merge_lo(s, run_a, run_b))
            else:
                stack.append(merge_hi(s, run_a, run_b))
            run_c is None or stack.append(run_c)
        else:
            stack.append(run_a)
            if run_c < run_b:
                stack.append(merge_hi(s, run_b, run_c))
            else:
                stack.append(merge_lo(s, run_b, run_c))
