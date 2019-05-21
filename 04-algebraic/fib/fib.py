def fib_recursive(n: int) -> int:
    """
    4. Алгоритм поиска чисел Фибоначчи
    4a. Через рекурсию
    """
    if n < 0:
        raise ValueError("n must be >= 0.")
    if 0 <= n < 3:
        return 1 if n else 0

    return fib_recursive(n - 2) + fib_recursive(n - 1)


def fib_recursive_acc(n: int, d={}) -> int:
    """
    4. Алгоритм поиска чисел Фибоначчи
    4a. Через рекурсию с аккумулятором
    """
    if n < 0:
        raise ValueError("n must be >= 0.")
    if 0 <= n < 3:
        return 1 if n else 0

    if n in d:
        return d[n]
    else:
        d[n] = fib_recursive_acc(n - 2) + fib_recursive_acc(n - 1)
        return d[n]


def fib_iterative(n: int) -> int:
    """
    4. Алгоритм поиска чисел Фибоначчи
    4b. Через итерацию
    """
    if n < 0:
        raise ValueError("n must be >= 0.")
    if 0 <= n < 3:
        return 1 if n else 0

    a = 1
    b = 1

    for i in range(3, n + 1):
        f = a + b
        a = b
        b = f

    return b


def fib_golden_ration(n: int) -> int:
    """
    4. Алгоритм поиска чисел Фибоначчи
    4c. По формуле золотого сечения
    """
    if n < 0:
        raise ValueError("n must be >= 0.")
    if 0 <= n < 3:
        return 1 if n else 0

    f = (1 + 5 ** 0.5) / 2
    return int(f ** n / 5 ** 0.5 + 0.5)


def fib_matrix(n: int) -> int:
    """
    4. Алгоритм поиска чисел Фибоначчи
    4d. Через умножение матриц
    """
    if n < 0:
        raise ValueError("n must be >= 0.")
    if 0 <= n < 3:
        return 1 if n else 0

    F = [[1, 1],
         [1, 0]]

    M = [[1, 1],
         [1, 0]]

    for i in range(2, n):
        x = (F[0][0] * M[0][0] +
             F[0][1] * M[1][0])
        y = (F[0][0] * M[0][1] +
             F[0][1] * M[1][1])
        z = (F[1][0] * M[0][0] +
             F[1][1] * M[1][0])
        w = (F[1][0] * M[0][1] +
             F[1][1] * M[1][1])

        F[0][0] = x
        F[0][1] = y
        F[1][0] = z
        F[1][1] = w

    return F[0][0]
