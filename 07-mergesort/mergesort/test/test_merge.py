import pytest
from mergesort import merge
from typing import TypeVar, List
from random import shuffle, randrange

T = TypeVar('T')


datasets = [
    ([], []),
    ([1], [1]),
    ([2, 1], [1, 2]),
    ([3, 2, 1], [1, 2, 3]),
    ([4, 2, 3, 1], [1, 2, 3, 4]),
    ([1, 2, 3, 4], [1, 2, 3, 4]),
    ([5, 2, 2, 4], [2, 2, 4, 5]),
    ([2, 5, 4, 2], [2, 2, 4, 5]),
    (
        [0, 1, 2, 3, 24, 5, 6, 7, 8, 38, 10,
         11, 12, 13, 14, 15, 16, 17, 18, 19,
         20, 21, 22, 23, 9, 25, 26, 27, 28,
         29, 30, 31, 32, 33, 34, 35, 36, 37,
         4, 39, 40, 41, 42, 43, 44, 45, 46,
         47, 48, 49],
        list(range(50)),
    ),
]


merge_sorts = [
    getattr(merge, name) for name in merge.__dict__
    if name.startswith('merge_sort')
]


@pytest.mark.parametrize(
    'f, dataset, expected',
    (
        (f, dataset, expected)
        for f in merge_sorts
        for dataset, expected in datasets
    )
)
def test_merge(f, dataset, expected):
    assert f(dataset.copy()) == expected


@pytest.mark.parametrize(
    'f', (f for f in merge_sorts)
)
def test_merge_stability(f):
    class A(int):
        ...
    ds = [A(3), A(2), A(2), A(1)]
    ds_expected = [id(ds[idx]) for idx in (3, 1, 2, 0)]
    ds_result = [id(t) for t in f(ds)]
    assert ds_result == ds_expected


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

    def __init__(self, start=20, stop=100_001):
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
    'f, dataset_name, dataset_value',
    (
        (f, dsn, dsv)
        for dsn, dsv in Datasets().items()
        for f in merge_sorts
    )
)
def test_bench_mergesorts(benchmark, f, dataset_name, dataset_value):
    result = benchmark.pedantic(
        f, iterations=1, rounds=5,
        setup=lambda: ((dataset_value.copy(),), {}))
    assert result == sorted(dataset_value.copy())
