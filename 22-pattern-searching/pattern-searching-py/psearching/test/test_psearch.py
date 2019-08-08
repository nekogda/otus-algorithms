import pytest
from typing import List, Tuple
import psearching.psearch as ps

base_ds = [
    ("", "", 0),
    ("a", "", 0),
    ("a", "a", 0),
    ("a", "b", None),
    ("qwerty", "q", 0),
    ("qwerty", "e", 2),
    ("qwerty", "y", 5),
    ("hello", "he", 0),
    ("---abcd---", "abcd", 3),
    ("Hoola-Hoola girls like Hooligans", "Hooligan", 23),
    ("abcabeabcabcabd", "abcabd", 9),
    ("aaa", "a", 0),
    ("abacabacak-abacak", "abacak", 4),
    ("abacabac", "abacak", None),
]

algorithms = [ps.BM, ps.BMH, ps.KMP]


@pytest.mark.parametrize(
    'test_case, alg',
    ((test_case, alg) for test_case in base_ds for alg in algorithms)
)
def test_base(test_case, alg):
    text, pattern, expected = test_case
    assert alg(text, pattern).find() == expected


def prep_from_file(file_name: str) -> List[Tuple[str, List[int]]]:
    with open(file_name, 'r') as fd:
        dataset = []
        for i, line in enumerate(fd):
            if i == 0:
                continue
            (text, pattern, expected) = line.split('\t')
            expected = [] if len(expected.strip()) == 0 else [
                int(n) for n in expected.split(' ')
            ]
            dataset.append((text, pattern, expected))
        return dataset


DATASET_FNAME = "psearching/string_matching_test_cases-31272-751472.tsv"
fdataset = prep_from_file(DATASET_FNAME)


@pytest.mark.parametrize(
    'fdataset, alg',
    (
        (fdataset, alg)
        for alg in algorithms
    )
)
def test_dataset(fdataset, alg):
    for case in fdataset:
        text, pattern, expected = case
        assert list(alg(text, pattern)) == expected


bench_ds = [
    ('start', ['a' * 100_000, 'baaaaaaaaa']),
    ('middle', ['a' * 100_000, 'aaaaabaaaaa']),
    ('end', ['a' * 100_000, 'aaaaaaaaab']),
    ('full_suff', ['----------bcd' + '------bcd' * 100_000, 'abcd......abcd']),
    ('part_suff', ['.................Mabcd' + '.........Mabcd' * 100_000,
                   '...Zabcd..Xabcd..Xabcd']),
]


@pytest.mark.parametrize(
    'name, ds, alg',
    ((name, ds, alg) for name, ds in bench_ds for alg in algorithms)
)
def test_bench_bm(benchmark, name, ds, alg):
    text, pattern = ds
    benchmark(alg(text, pattern).find)
    assert True
