import pytest
from gcd import gcd


gcds = [
    getattr(gcd, name) for name in gcd.__dict__ if name.startswith('gcd_')
]


@pytest.mark.parametrize(
    "f, test_input, expected", [
        (f, input, expected) for input, expected in
        [
            ((10, 5), 5),
            ((0, 0), 0),
            ((10, 10), 10),
            ((125, 25), 25),
            ((125, 17), 1),
        ] for f in gcds
    ]
)
def test_gcd(f, test_input, expected):
    assert expected == f(*test_input)


@pytest.mark.parametrize(
    'f', gcds
)
def test_gcd_bench(benchmark, f):
    benchmark(f, 123456789, 12)
