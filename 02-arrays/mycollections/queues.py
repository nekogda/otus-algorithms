from abc import ABC, abstractmethod
from . import arrays


class AbstractQueue(ABC):

    @abstractmethod
    def enqueue(self, priority, item):
        pass

    @abstractmethod
    def dequeue(self):
        pass

    @abstractmethod
    def __len__(self):
        pass

    @abstractmethod
    def __repr__(self):
        pass


class PriorityQueue(AbstractQueue):

    def __init__(self, init_list=[]):
        self._array = arrays.VectorArray(10)
        self._size = 0
        for item in init_list:
            pass

    def enqueue(self, priority, item):
        pq = self._get_pq(priority)
        pq.append(item)
        self._set_pq(pq, priority)
        self._size += 1

    def dequeue(self):
        for i in range(len(self._array)):
            if len(self._array[i]):
                self._size -= 1
                return self._array[i].remove(0)
        raise IndexError('dequeue from empty queue')

    def _get_pq(self, priority):
        if priority >= len(self._array):
            return arrays.VectorArray()
        else:
            return self._array[priority]

    def _set_pq(self, pq, priority):
        if priority >= len(self._array):
            for i in range(len(self._array), priority):
                self._array.append(arrays.VectorArray())
            self._array.append(pq)

    def __len__(self):
        return self._size

    def __repr__(self):
        s = f'{self.__class__.__name__}'
        ss = ''
        for i in range(len(self._array)):
            if len(self._array[i]):
                for j in range(len(self._array[i])):
                    ss += f'({i}, {self._array[i][j]}), '

        return f'{s}({ss})'
