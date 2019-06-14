from typing import List, TypeVar, Callable


T = TypeVar('T')


def swap(s: List[T], i: int, j: int) -> None:
    s[i], s[j] = s[j], s[i]


def drown_rec(s: List[T],
              root_idx: int,
              heap_len: int,
              ) -> None:

    max_idx = root_idx
    left_idx = root_idx * 2 + 1
    right_idx = root_idx * 2 + 2

    if left_idx < heap_len and s[left_idx] > s[max_idx]:
        max_idx = left_idx
    if right_idx < heap_len and s[right_idx] > s[max_idx]:
        max_idx = right_idx
    if max_idx != root_idx:
        swap(s, root_idx, max_idx)
        drown_rec(s, max_idx, heap_len)


def drown_it(s: List[T],
             root_idx: int,
             heap_len: int,
             ) -> None:

    while True:
        max_idx = root_idx
        left_idx = root_idx * 2 + 1
        right_idx = root_idx * 2 + 2

        if left_idx < heap_len \
           and s[left_idx] > s[max_idx]:
            max_idx = left_idx
        if right_idx < heap_len \
           and s[right_idx] > s[max_idx]:
            max_idx = right_idx
        if max_idx != root_idx:
            swap(s, root_idx, max_idx)
            root_idx = max_idx
        else:
            break


def heapify(s: List[T],
            drown: Callable[[List[T], int], None] = drown_rec,
            ) -> None:
    last_parent = len(s) // 2 - 1
    for root_idx in range(last_parent, -1, -1):
        drown(s, root_idx, len(s))


def _sort_elements(s: List[T],
                   drown: Callable[[List[T], int], None] = drown_rec,
                   ):
    for i in range(len(s)):
        # swap first and last
        last_idx = len(s) - 1 - i
        s[0], s[last_idx] = s[last_idx], s[0]
        # reduce len by one
        heap_len = len(s) - i - 1
        # drown first element to its new position
        drown(s, 0, heap_len)


def heapsort(s: List[T],
             drown: Callable[[List[T], int], None] = drown_rec,
             ) -> List[T]:
    heapify(s, drown)
    _sort_elements(s, drown)
    return s


def remove(heap: List[T], idx: int) -> T:
    if len(heap) == 0:
        raise ValueError("heap must be non empty")

    if idx < 0 or idx >= len(heap):
        raise IndexError("index out of range")

    if idx == len(heap) - 1:
        return heap.pop()

    result = heap[idx]
    heap[idx] = heap.pop()
    drown_rec(heap, idx, len(heap))
    return result
