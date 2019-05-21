import pytest
from fib import fib


fibs = [
    getattr(fib, name) for name in fib.__dict__ if name.startswith('fib_')
]


@pytest.mark.parametrize(
    "f, test_input, expected", [
        (f, test_input, expected) for test_input, expected in
        [
            (1, 1),
            (2, 1),
            (3, 2),
            (10, 55),
        ] for f in fibs
    ]
)
def test_fib(f, test_input, expected):
    assert expected == f(test_input)


@pytest.mark.parametrize(
    'f', fibs
)
def test_fib_bench(benchmark, f):
    benchmark(f, 30)
