from __future__ import annotations
from typing import List


def find_empty(t: str, p: str, start: int) -> int:
    assert len(p) == 0
    if start <= len(t):
        return start
    raise StopIteration


def find_bf(t: str, p: str, start: int) -> int:
    """ brute force special case """
    assert len(p) != 0
    for i in range(start, len(t) - len(p) + 1):
        matches = 0
        for j in range(len(p)):
            if t[i+j] == p[j]:
                matches += 1
            else:
                matches = 0
                break
        if matches == len(p):
            return i
    raise StopIteration


def bct_prep(p: str) -> List[int]:
    table = [len(p)] * 256
    for i in range(len(p) - 1):
        table[
            int.from_bytes(bytes(p[i], "utf-8"), byteorder='little')
        ] = len(p) - 1 - i
    return table


class BMH_iterator:
    def __init__(self, bmh: BMH):
        self._bmh = bmh
        self.start = 0
        self.bct = bct_prep(self._bmh.p)

    def __next__(self) -> int:
        len_p = len(self._bmh.p)
        len_t = len(self._bmh.t)
        if len_t < len_p:
            raise StopIteration
        elif len_p == 0:
            result = find_empty(self._bmh.t, self._bmh.p, self.start)
            self.start += 1
        elif len_p == 1:
            result = find_bf(self._bmh.t, self._bmh.p, self.start)
            self.start = result + len_p
        else:
            result = self._find()
            self.start = result + len_p

        return result

    def _find(self) -> int:
        i = self.start
        len_p = len(self._bmh.p)
        len_t = len(self._bmh.t)

        while i <= len_t - len_p:
            j = len_p - 1
            while self._bmh.p[j] == self._bmh.t[i+j]:
                if j == 0:
                    return i
                j -= 1
            idx = int.from_bytes(
                bytes(self._bmh.t[i + len_p - 1], "utf-8"), byteorder='little'
            )
            i += self.bct[idx]
        raise StopIteration


class BMH:
    def __init__(self, text: str, pattern: str):
        if not text.isascii() or not pattern.isascii():
            raise ValueError("only ASCII symbols allowed")
        self.t = text
        self.p = pattern

    def __iter__(self) -> BMH_iterator:
        return BMH_iterator(self)

    def find(self) -> int:
        for match in self:
            return match
        return None


class BM_iterator:
    def __init__(self, bm: BM):
        self._bm = bm
        self.start = 0
        self.bct = bct_prep(self._bm.p)
        self.gst = gst_prep(self._bm.p)

    def __next__(self) -> int:
        len_p = len(self._bm.p)
        len_t = len(self._bm.t)
        if len_t < len_p:
            raise StopIteration
        elif len_p == 0:
            result = find_empty(self._bm.t, self._bm.p, self.start)
            self.start += 1
        elif len_p == 1:
            result = find_bf(self._bm.t, self._bm.p, self.start)
            self.start = result + len_p
        else:
            result = self._find()
            self.start = result + len_p
        return result

    def _find(self) -> int:
        i = self.start + len(self._bm.p) - 1
        while i < len(self._bm.t):
            j = len(self._bm.p) - 1
            while j > 0 and self._bm.t[i] == self._bm.p[j]:
                i -= 1
                j -= 1
            if j == 0 and self._bm.t[i] == self._bm.p[j]:
                return i
            idx = int.from_bytes(bytes(self._bm.t[i], 'utf-8'),
                                 byteorder='little')
            i += max(self.bct[idx], self.gst[j])
        raise StopIteration


class BM:
    def __init__(self, text: str, pattern: str):
        if not text.isascii() or not pattern.isascii():
            raise ValueError("only ASCII symbols allowed")
        self.t = text
        self.p = pattern

    def __iter__(self) -> BM_iterator:
        return BM_iterator(self)

    def find(self) -> int:
        for match in self:
            return match
        return None


def gst_prep(p: str) -> List[int]:
    table = [0] * len(p)
    last = len(p) - 1
    last_prefix = last
    for i in range(last, -1, -1):
        if p.startswith(p[i+1:]):
            last_prefix = i + 1
        table[i] = last_prefix + last - i
    for i in range(last):
        len_suffix = longest_common_suffix(p, p[1:i+1])
        if p[i - len_suffix] != p[last - len_suffix]:
            table[last - len_suffix] = len_suffix + last - i
    return table


def longest_common_suffix(a: str, b: str) -> int:
    i = 0
    while i < len(a) and i < len(b):
        if a[len(a) - 1 - i] != b[len(b) - 1 - i]:
            break
        i += 1
    return i


class KMP_iterator:
    def __init__(self, kmp: KMP):
        self._kmp = kmp
        self.start = 0
        self.dfa = dfa_prep(self._kmp.p)

    def __next__(self) -> int:
        len_p = len(self._kmp.p)
        len_t = len(self._kmp.t)
        if len_t < len_p:
            raise StopIteration
        elif len_p == 0:
            result = find_empty(self._kmp.t, self._kmp.p, self.start)
            self.start += 1
        elif len_p == 1:
            result = find_bf(self._kmp.t, self._kmp.p, self.start)
            self.start = result + len_p
        else:
            result = self._find()
            self.start = result + 1
        return result

    def _find(self) -> int:
        i = self.start
        j = 0
        len_p = len(self._kmp.p)
        len_t = len(self._kmp.t)
        while i < len_t and j < len_p:
            idx = int.from_bytes(bytes(self._kmp.t[i], 'utf-8'),
                                 byteorder='little')
            j = self.dfa[idx][j]
            i += 1
        if j == len_p:
            return i - len_p
        raise StopIteration


class KMP:
    def __init__(self, text: str, pattern: str):
        if not text.isascii() or not pattern.isascii():
            raise ValueError("only ASCII symbols allowed")
        self.t = text
        self.p = pattern

    def __iter__(self) -> KMP_iterator:
        return KMP_iterator(self)

    def find(self) -> int:
        for match in self:
            return match
        return None


def dfa_prep(p: str) -> List[List[int]]:
    if len(p) == 0:
        return [[]]
    table = [[0] * len(p) for _ in range(256)]
    table[int.from_bytes(bytes(p[0], 'utf-8'), byteorder='little')][0] = 1
    x = 0
    for j in range(1, len(p)):
        for c in range(256):
            table[c][j] = table[c][x]
        table[int.from_bytes(bytes(p[j], 'utf-8'),
                             byteorder='little')][j] = j + 1
        x = table[int.from_bytes(bytes(p[j], 'utf-8'), byteorder='little')][x]
    return table
