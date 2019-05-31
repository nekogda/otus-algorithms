from heapsort import pq
import pytest


def test_pq_base():
    priq = pq.PriorityQueue()
    assert len(priq) == 0
    priq = pq.PriorityQueue([3, 1, 2])
    assert len(priq) == 3


def test_pq_enqueue():
    priq = pq.PriorityQueue()
    priq.enqueue(3)
    assert len(priq) == 1
    assert priq.dequeue() == 3
    assert len(priq) == 0


def test_pq_dequeue():
    priq = pq.PriorityQueue([2, 3, 1])
    v = priq.dequeue()
    assert len(priq) == 2
    assert v == 3
    v = priq.dequeue()
    assert len(priq) == 1
    assert v == 2
    v = priq.dequeue()
    assert len(priq) == 0
    assert v == 1
    with pytest.raises(IndexError):
        priq.dequeue()
