import pytest
from shsort import shsort
from insort import insort
from random import randrange, shuffle
from typing import TypeVar, List

T = TypeVar('T')


shsorts = [
    getattr(shsort, name) for name in shsort.__dict__
    if name.startswith('shell_')
]

insorts = [
    getattr(insort, name) for name in insort.__dict__
    if name.startswith('insertion_')
]

step_gens = [
    getattr(shsort, name) for name in shsort.__dict__
    if name.startswith('_step_gen_')
]


TEST_DATA = [
    ([], []),
    ([1], [1]),
    ([2, 1], [1, 2]),
    ([1, 2], [1, 2]),
    ([3, 1, 2], [1, 2, 3]),
    ([4, 3, 2, 1], [1, 2, 3, 4]),
]


@pytest.mark.parametrize(
    'f, seq, expected, stepgen',
    [
        (f, *a, s)
        for f in shsorts
        for a in TEST_DATA
        for s in step_gens
    ]
)
def test_shsort(f, seq, expected, stepgen):
    assert f(seq, stepgen) == expected


@pytest.mark.parametrize(
    'f, seq, expected',
    [
        (f, *a)
        for f in insorts
        for a in TEST_DATA
    ]
)
def test_insort(f, seq, expected):
    assert f(seq) == expected


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
    'f, step_gen, dataset_name, dataset_value',
    (
        (f, s, dsn, dsv)
        for dsn, dsv in Datasets().items()
        for f in shsorts
        for s in step_gens
    )
)
def test_bench_shsorts(benchmark, f, step_gen, dataset_name, dataset_value):
    _ = benchmark.pedantic(
        f, iterations=1, rounds=5,
        setup=lambda: ((dataset_value.copy(), step_gen), {}))


@pytest.mark.parametrize(
    'f, dataset_name, dataset_value',
    (
        (f, dsn, dsv)
        for dsn, dsv in Datasets().items()
        for f in insorts
    )
)
def test_bench_insorts(benchmark, f, dataset_name, dataset_value):
    _ = benchmark.pedantic(
        f, iterations=1, rounds=3,
        setup=lambda: ((dataset_value.copy(),), {}))
