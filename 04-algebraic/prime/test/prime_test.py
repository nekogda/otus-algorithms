import pytest
from prime import prime


primes = [
    getattr(prime, name)
    for name in prime.__dict__ if name.startswith('prime_')
]


@pytest.mark.parametrize(
    "f, test_input, expected", [
        (f, test_input, expected) for test_input, expected in
        [
            (2, [2]),
            (3, [2, 3]),
            (50, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]),
        ] for f in primes
    ]
)
def test_prime(f, test_input, expected):
    assert expected == f(test_input)


@pytest.mark.parametrize(
    'f', primes
)
def test_prime_bench(benchmark, f):
    benchmark(f, 50_000)
