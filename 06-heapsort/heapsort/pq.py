from . import heapsort as hs
from abc import ABC, abstractmethod


class AbstractQueue(ABC):

    @abstractmethod
    def enqueue(self, priority, item):
        pass

    @abstractmethod
    def dequeue(self):
        pass


class PriorityQueue(AbstractQueue):

    def __init__(self, init_list=[]):
        hs.heapify(init_list)
        self._heap = init_list

    def enqueue(self, item):
        self._heap.append(item)
        hs.heapify(self._heap)

    def dequeue(self):
        if len(self._heap) == 0:
            raise IndexError('dequeue from empty queue')
        else:
            r = hs.remove(self._heap, 0)
            return r

    def __len__(self):
        return len(self._heap)

    def __repr__(self):
        return repr(self._heap)
