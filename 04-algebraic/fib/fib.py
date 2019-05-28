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


class Matrix2D:

    def __init__(self, x11: int, x12: int, x21: int, x22: int):
        self.x11 = x11
        self.x12 = x12
        self.x21 = x21
        self.x22 = x22

    @staticmethod
    def INDENTITY():
        return Matrix2D(1, 0, 0, 1)

    @staticmethod
    def BASE():
        return Matrix2D(1, 1, 1, 0)

    def pow2(self) -> 'Matrix2D':
        return self.multiply(self)

    def multiply(self, m: 'Matrix2D') -> 'Matrix2D':
        x11 = self.x11 * m.x11 + self.x12 * m.x21
        x12 = self.x11 * m.x12 + self.x12 * m.x22
        x21 = self.x21 * m.x11 + self.x22 * m.x21
        x22 = self.x21 * m.x12 + self.x22 * m.x22
        return Matrix2D(x11, x12, x21, x22)


def fib_matrix(n: int) -> int:
    """
    4. Алгоритм поиска чисел Фибоначчи
    4d. Через умножение матриц
    """
    if n < 0:
        raise ValueError("n must be >= 0.")
    if 0 <= n < 3:
        return 1 if n else 0

    res = Matrix2D.INDENTITY()
    base = Matrix2D.BASE()

    while (n > 1):
        if n & 1:
            res = res.multiply(base)
        base = base.pow2()
        n >>= 1

    return res.multiply(base).x21
