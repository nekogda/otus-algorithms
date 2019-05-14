from abc import ABC, abstractmethod
import ctypes
import math


class AbstractList:

    @property
    @abstractmethod
    def cap(self):
        pass

    @abstractmethod
    def insert(self, idx, item):
        pass

    @abstractmethod
    def append(self, item):
        pass

    @abstractmethod
    def remove(self, idx) -> int:
        pass


def _copy(*, src, start_src, stop_src, dst, start_dst):

    if src._array is dst._array:
        if start_src == start_dst:
            return
        else:
            if start_src < start_dst:
                for i in range(stop_src, start_src, -1):
                    dst[i], dst[i-1] = dst[i-1], dst[i]
            else:
                for i in range(start_src, stop_src, 1):
                    dst[i-1], dst[i] = dst[i], dst[i-1]

    else:
        if start_src == start_dst:
            for i in range(start_src, stop_src):
                dst[i] = src[i]
        else:
            for i in range(start_src, stop_src):
                dst[start_dst] = src[i]
                start_dst += 1


class Array:

    def __init__(self, size=0, init_list=[]):
        if size < 0:
            raise ValueError('size must be greater of equal to zero.')
        if size < len(init_list):
            raise ValueError('init_list must be less or equal to size.')
        self._array = (ctypes.py_object * size)(
            *init_list,
            *[None for _ in range(size - len(init_list))],
        )

    def __getitem__(self, idx):
        self._check_idx(idx)
        return self._array[idx]

    def __setitem__(self, idx, item):
        self._check_idx(idx)
        self._array[idx] = item

    def __delitem__(self, idx):
        self._check_idx(idx)
        self._array[idx] = None

    def __len__(self):
        return len(self._array)

    def _check_idx(self, idx):
        if idx < 0 or idx > len(self):
            raise IndexError("index out of range")

    def __repr__(self):
        return (
            f'{self.__class__.__name__}({len(self)}, '
            f'{[x for x in self._array]})'
        )


class SingleArray(AbstractList):

    def __init__(self, size=0, init_list=[]):
        self._array = Array(size, init_list)
        self._size = size
        self._resize_step = 1

    @property
    def cap(self):
        return len(self._array)

    def __len__(self):
        return self._size

    def __getitem__(self, idx):
        self._check_idx(idx)
        return self._array[idx]

    def __setitem__(self, idx, item):
        self._check_idx(idx)
        self._array[idx] = item

    def __delitem__(self, idx):
        removed = self[idx]
        old = self._resize(-1)
        _copy(
            src=old,
            start_src=0,
            stop_src=idx,
            dst=self,
            start_dst=0,
        )
        _copy(
            src=old,
            start_src=idx+1,
            stop_src=old._size,
            dst=self,
            start_dst=idx,
        )
        return removed

    def __repr__(self):
        return (
            f'{self.__class__.__name__}({len(self)}, '
            f'{[self[i] for i in range(len(self))]})'
        )

    def _check_idx(self, idx):
        if idx < 0 or idx > len(self):
            raise IndexError("index out of range")

    def append(self, item):
        old = self._resize(1)
        _copy(
            src=old,
            start_src=0,
            stop_src=old._size,
            dst=self,
            start_dst=0,
        )
        self[len(self) - 1] = item

    def insert(self, idx, item):
        if idx >= len(self):
            self.append(item)
        else:
            old = self._resize(1)
            _copy(
                src=old,
                start_src=0,
                stop_src=idx,
                dst=self,
                start_dst=0,
            )
            _copy(
                src=old,
                start_src=idx,
                stop_src=old._size,
                dst=self,
                start_dst=idx+1
            )
            self[idx] = item

    def _allocator(self, step):
        sign = 1 if step > 0 else -1
        self._array = Array(self.cap + sign * self._resize_step)

    def _resize(self, step):
        if not step:
            raise ValueError("step can't be equal to zero")

        old = self.__class__()
        old._size = self._size
        old._array = self._array

        new_size = len(self) + step
        size_delta = self.cap - new_size

        if new_size > self.cap or size_delta == self._resize_step:
            self._allocator(step)

        self._size = new_size
        return old

    def remove(self, idx):
        return self.__delitem__(idx)


class VectorArray(SingleArray):
    def __init__(self, resize_step=10, *args, **kwargs):
        if resize_step <= 0:
            raise ValueError('resize_step must be greater then zero')
        super().__init__(*args, **kwargs)
        self._resize_step = resize_step


class FactorArray(SingleArray):
    def __init__(self, resize_step=50, *args, **kwargs):
        if resize_step <= 0:
            raise ValueError('resize_step must be greater then zero')
        super().__init__(*args, **kwargs)
        self.__resize_step = resize_step

    @property
    def _resize_step(self):
        if not self.cap:
            return 1
        else:
            return math.ceil(self.cap * self.__resize_step / 100)

    @_resize_step.setter
    def _resize_step(self, value):
        self.__resize_step = value


class MatrixArray(SingleArray):
    def __init__(self, resize_step=5, init_list=[]):
        if resize_step <= 0:
            raise ValueError('capacity must be greater then zero')
        self._array = VectorArray(10)
        self._size = 0
        self._resize_step = resize_step
        for item in init_list:
            self.append(item)

    @property
    def cap(self):
        return self._resize_step * len(self._array)

    def _allocator(self, step):
        if step > 0:
            self._array.append(Array(self._resize_step))
        else:
            self._array.remove(len(self._array) - 1)

    def __getitem__(self, idx):
        return self._array[idx // self._resize_step][idx % self._resize_step]

    def __setitem__(self, idx, item):
        self._array[idx // self._resize_step][idx % self._resize_step] = item
