def gcd_min(a: int, b: int) -> int:
    """
    Алгоритм Евклида поиска НОД
    1a. Через вычитание
    """
    while a != b:
        if a > b:
            a = a - b
        else:
            b = b - a
    return a


def gcd_mod(a: int, b: int) -> int:
    """
    Алгоритм Евклида поиска НОД
    1b. Через остаток
    """
    while a != 0 and b != 0:
        if a > b:
            a = a % b
        else:
            b = b % a

    return max([a, b])


def gcd_rec(a: int, b: int) -> int:
    """
    Алгоритм Евклида поиска НОД
    Рекурсивная версия
    """
    if b == 0:
        return a
    else:
        return gcd_rec(b, a % b)
