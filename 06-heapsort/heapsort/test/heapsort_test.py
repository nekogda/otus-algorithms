import pytest
from heapsort.heapsort import heapsort as hs
from heapsort.heapsort import heapify
import heapsort.heapsort
from typing import List, TypeVar
from random import shuffle, randrange

T = TypeVar('T')


drowns = [
    (getattr(heapsort.heapsort, name), f'{name}')
    for name in heapsort.heapsort.__dict__
    if name.startswith('drown_')
]

dataset = [
    ([], []),
    ([1], [1]),
    ([3, 2, 1], [1, 2, 3]),
    ([1, 3, 2, 4], [1, 2, 3, 4]),
    ([4, 1, 2, 2], [1, 2, 2, 4]),
    ([2, 1, 2, 4], [1, 2, 2, 4]),
    ([16, 5, 15, 7, 17, 13, 4, 12, 8, 1, 14, 6, 10, 9, 2, 19, 0, 3, 18, 11],
     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]),
]


dataset_remove = [
    # test_data, idx, expected, expected_heap
    ([], None, None, None),
    ([1], 0, 1, []),
    ([3, 2, 1], 0, 3, [2, 1]),
    ([1, 3, 2, 4], 0, 4, [3, 1, 2]),
    ([4, 1, 2, 2], 0, 4, [2, 1, 2]),
    ([2, 1, 2, 4], 0, 4, [2, 1, 2]),
    (
        [19, 18, 15, 16, 17, 13, 9, 12, 8, 11, 14, 6, 10, 4, 2, 7, 0, 3, 5, 1],
        0, 19,
        [18, 17, 15, 16, 14, 13, 9, 12, 8, 11, 1, 6, 10, 4, 2, 7, 0, 3, 5],
    ),
    # from the middle
    ([1, 3, 2, 4], 2, 2, [4, 3, 1]),
    ([4, 1, 2, 2], 2, 2, [4, 2, 1]),
    ([2, 1, 2, 4], 2, 2, [4, 2, 1]),
    # from the end
    ([1, 3, 2, 4], 3, 1, [4, 3, 2]),
    ([4, 1, 2, 2], 3, 1, [4, 2, 2]),
    ([2, 1, 2, 4], 3, 1, [4, 2, 2]),
]


@pytest.mark.parametrize(
    'f, drown_name, test_data, expected',
    (
        (f, drown_name, test_data, expected)
        for f, drown_name in drowns
        for test_data, expected in dataset
    )
)
def test_base(f, drown_name, test_data, expected):
    assert hs(test_data, f) == expected


@pytest.mark.parametrize(
    'test_data, idx, expected, expected_heap',
    (
        (test_data, idx, expected, expected_heap)
        for test_data, idx, expected, expected_heap in dataset_remove
    )
)
def test_remove(test_data, idx, expected, expected_heap):
    if len(test_data) == 0:
        with pytest.raises(ValueError):
            heapsort.heapsort.remove(test_data, idx)
        return
    heapify(test_data)
    assert heapsort.heapsort.remove(test_data, idx) == expected
    assert test_data == expected_heap


def test_remove_raises():
    with pytest.raises(IndexError):
        heapsort.heapsort.remove([1, 2, 3], -1)

    with pytest.raises(IndexError):
        heapsort.heapsort.remove([1, 2, 3], 3)


def shuffle_pct(s: List[T], count: int, pct=True):
    if pct:
        if count == 100:
            shuffle(s)
            return s
        count = int(len(s) / 100 * count)
    if not count:
        raise ValueError()
    indexes = []
    while len(indexes) < count:
        x = randrange(0, len(s))
        if x not in indexes:
            indexes.append(x)
    shuffle(indexes)
    for i in range(len(indexes)):
        s[indexes[i]], s[indexes[i-1]] = s[indexes[i-1]], s[indexes[i]]
    return s


class Datasets:

    _data = {}

    def __init__(self, start=20, stop=55000):
        if self.__class__._data:
            return

        while start <= stop:
            self.__class__._data[f'{start}_rnd_5'] = shuffle_pct(
                list(range(start)), 5, None
            )
            self.__class__._data[f'{start}_rnd_10_pct'] = shuffle_pct(
                list(range(start)), 10
            )
            self.__class__._data[f'{start}_rnd_100_pct'] = shuffle_pct(
                list(range(start)), 100
            )
            self.__class__._data[f'{start}_rnd_R'] = list(
                reversed(range(start))
            )
            if start <= 10240:
                start *= 2
            else:
                start += 10240

    def items(self):
        return self.__class__._data.items()


@pytest.mark.parametrize(
    'f, drown_name, dataset_name, dataset_value',
    (
        (f, drown_name, dsn, dsv)
        for dsn, dsv in Datasets().items()
        for f, drown_name in drowns
    )
)
def test_bench_heapsorts(
        benchmark, f, drown_name, dataset_name, dataset_value
):
    result = benchmark.pedantic(
        hs, iterations=1, rounds=5,
        setup=lambda: ((dataset_value.copy(), f), {}))
    assert result == sorted(dataset_value.copy())
