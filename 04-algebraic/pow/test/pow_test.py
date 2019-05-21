import pytest
from pow import pow


powers = [
    getattr(pow, name) for name in pow.__dict__ if name.startswith('pow_')
]


@pytest.mark.parametrize(
    "f, test_input", [
        (f, test_input) for test_input in
        [
            (2, 2),
            (2, 3),
            (2, 5),
            (2, 7),
            (3, 8),
            (3, 9),
            (2, 10),
            (2, 11),
            (2, 12),
            (11, 11),
            (11, 12),
            (2, 64),
            (3, 64),
        ] for f in powers
    ]
)
def test_pow(f, test_input):
    assert test_input[0] ** test_input[1] == f(*test_input)


@pytest.mark.parametrize(
    'f', powers
)
def test_pow_bench(benchmark, f):
    benchmark(f, 1, 100_000_000)
