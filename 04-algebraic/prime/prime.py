import ctypes


def prime_divider(n: int) -> [int]:
    """
    3. Алгоритмы поиска кол-ва простых чисел до N
    3a. Через перебор делителей.
    """
    primes = []
    is_prime = True

    for i in range(2, n + 1):
        is_prime = True
        for j in range(2, i):
            if i % j == 0:
                is_prime = False
                break
        if is_prime:
            primes.append(i)

    return primes


def prime_divider_optimized(n: int) -> [int]:
    """
    3. Алгоритмы поиска кол-ва простых чисел до N
    3b. Оптимизация перебора делителей.
    """
    primes = []
    is_prime = True

    if n >= 2:
        primes.append(2)

    for i in range(3, n + 1, 2):
        is_prime = True
        for j in range(3, int(i**0.5) + 1, 2):
            if i % j == 0:
                is_prime = False
                break
        if is_prime:
            primes.append(i)

    return primes


def prime_sieve(n: int) -> [int]:
    """
    3. Алгоритмы поиска кол-ва простых чисел до N
    3c. Решето Эратосфена.
    """

    primes = [False, False, *([True] * (n - 1))]

    for p in range(3, n, 2):
        if not primes[p]:
            continue
        start = p * p
        if start > n:
            break

        for k in range(start, n + 1, p):
            if primes[k] and k % p == 0:
                primes[k] = False

    return [
        2,
        *[prime for prime, valid in enumerate(primes) if prime % 2 and valid]
    ]


def prime_sieve_bin(n: int) -> [int]:
    """
    3. Алгоритмы поиска кол-ва простых чисел до N
    3d. Решето Эратосфена с битовой матрицей, по 64 значения в одном int
    """

    size = 64
    num, remainder = divmod(n + 1, size)
    if remainder:
        num += 1

    primes = (ctypes.c_int64 * num)(*([0xFF] * num))
    primes[0] = 0xFFFFFFFFFFFFFFFC

    for p in range(3, n, 2):
        byte_idx, bit_offset = divmod(p, size)
        if not primes[byte_idx] & (1 << bit_offset):
            continue
        start = p * p
        if start > n:
            break
        for k in range(start, n + 1, p):
            byte_idx, bit_offset = divmod(k, size)
            if primes[byte_idx] & (1 << bit_offset) and k % p == 0:
                primes[byte_idx] = primes[byte_idx] ^ (1 << bit_offset)

    # Prepare result
    counter = 0
    result = [2]

    for idx in range(n + 1):
        counter += 1
        if not idx & 1:
            continue
        byte_idx, bit_offset = divmod(idx, size)
        if primes[byte_idx] & (1 << bit_offset):
            result.append(counter - 1)

    return result
