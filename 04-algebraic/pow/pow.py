def pow_iterative(x, n):
    """
    Алгоритм возведения в степень
    2a. Итеративный (n умножений)
    """
    result = 1
    for _ in range(n):
        result *= x

    return result


def pow_2nm(x, k):
    """
    Алгоритм возведения в степень
    2b. Через степень двойки с домножением
    """
    result = 1
    n = 0
    while result * 2 <= k:
        n += 1
        result *= 2

    rounds = k - result

    base = 1
    for _ in range(n):
        base *= x

    remainder_rounds = k - (rounds * n + n)

    if remainder_rounds < 0:
        result = 1
        remainder_rounds += n
    else:
        result = base

    for _ in range(rounds):
        result *= base

    # remaining multiplications

    for _ in range(remainder_rounds):
        result *= x

    return result


def pow_bin_decomposition(x, n):
    """
    Алгоритм возведения в степень
    2c. Через двоичное разложение показателя степени.
    """
    res = 1
    power = n
    base = x
    while power > 1:
        if power % 2:
            res *= base
        base *= base
        power //= 2
    if power:
        res *= base

    return res
