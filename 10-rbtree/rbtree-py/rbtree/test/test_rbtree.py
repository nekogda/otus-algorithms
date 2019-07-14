import rbtree.rbtree as rbt
import avltree.avltree as avl
from itertools import chain, product
import pytest
from typing import TypeVar


T = TypeVar('T')


def test_it_works():
    t = rbt.RBTree([3, 2, 1])
    assert t.root.value == 2


def test_find_pos():
    t = rbt.RBTree([1, 2, 3])
    assert t.find(3).value == 3


def test_find_neg_1():
    t = rbt.RBTree([1, 2, 3])
    assert t.find(4) is None


def test_find_neg_2():
    t = rbt.RBTree()
    assert t.find(4) is None


def test_insert_parent_1():
    t = rbt.RBTree([1])
    assert t.root.parent is None


def test_insert_parent_2():
    t = rbt.RBTree([1, 2])
    assert t.root.right.parent.value == 1


def test_insert_pos():
    t = rbt.RBTree([1, 2, 3])
    assert t.find(3).value == 3


def test_insert_neg():
    with pytest.raises(ValueError):
        rbt.RBTree([1, 2, 2, 3])


def test_grandparent_pos_1():
    t = rbt.RBTree([10, 6, 2, 15])
    assert t.find(15).grandparent.value == 6


def test_grandparent_pos_2():
    t = rbt.RBTree([10, 6, 15, 20])
    assert t.find(20).grandparent.value == 10


def test_grandparent_neg():
    t = rbt.RBTree([2, 1, 3])
    assert t.find(1).grandparent is None


def test_uncle_pos_1():
    t = rbt.RBTree([10, 6, 2, 15])
    assert t.find(15).uncle.value == 2


def test_uncle_pos_2():
    t = rbt.RBTree([10, 6, 15, 20])
    assert t.find(20).uncle.value == 6


def test_uncle_neg():
    t = rbt.RBTree([2, 1, 3])
    assert t.find(1).uncle is None


dataset = [
    ([10, 3, 15, 5, 7], {
        "Lv5cBpv10-[v10cBpvNone]-Rv15cBpv10",
        "None-[v15cBpv10]-None",
        "Lv3cRpv5-[v5cBpv10]-Rv7cRpv5",
        "None-[v7cRpv5]-None",
        "None-[v3cRpv5]-None",
    }),
    ([10, 3, 15, 7, 5], {
        "Lv5cBpv10-[v10cBpvNone]-Rv15cBpv10",
        "None-[v15cBpv10]-None",
        "Lv3cRpv5-[v5cBpv10]-Rv7cRpv5",
        "None-[v7cRpv5]-None",
        "None-[v3cRpv5]-None",
    }),
    ([10, 3, 15, 13, 14], {
        "Lv3cBpv10-[v10cBpvNone]-Rv14cBpv10",
        "Lv13cRpv14-[v14cBpv10]-Rv15cRpv14",
        "None-[v3cBpv10]-None",
        "None-[v15cRpv14]-None",
        "None-[v13cRpv14]-None",
    }),
    ([10, 4, 15, 20, 25, 30, 35, 40], {
        "Lv10cRpv20-[v20cBpvNone]-Rv30cRpv20",
        "Lv4cBpv10-[v10cRpv20]-Rv15cBpv10",
        "Lv25cBpv30-[v30cRpv20]-Rv35cBpv30",
        "None-[v35cBpv30]-Rv40cRpv35",
        "None-[v40cRpv35]-None",
        "None-[v25cBpv30]-None",
        "None-[v4cBpv10]-None",
        "None-[v15cBpv10]-None",
    }),
    ([30, 35, 25, 28, 26, 20, 15, 10], {
        "Lv20cRpv26-[v26cBpvNone]-Rv30cRpv26",
        "Lv28cBpv30-[v30cRpv26]-Rv35cBpv30",
        "Lv15cBpv20-[v20cRpv26]-Rv25cBpv20",
        "Lv10cRpv15-[v15cBpv20]-None",
        "None-[v10cRpv15]-None",
        "None-[v25cBpv20]-None",
        "None-[v28cBpv30]-None",
        "None-[v35cBpv30]-None",
    }),
]


@pytest.mark.parametrize(
    'test_data, expected',
    (
        (test_data, expected)
        for test_data, expected in dataset
    )
)
def test_full(test_data, expected):
    t = rbt.RBTree(test_data)
    assert t.compare(expected)


@pytest.mark.parametrize(
    'tree, num',
    (
        (tree, num) for tree, num in product(
            [rbt.RBTree, avl.AVLtree], [10_000, 100_000, 500_000, 1_000_000]
        )
    )
)
def test_bench_insert(benchmark, tree, num):
    benchmark(tree, range(num))
    assert True


def _find_test(tree: T):  # pragma: no cover
    for i in chain(range(10_000), range(990_000, 1_000_000)):
        tree.find(i)


@pytest.mark.parametrize(
    'tree, num',
    (
        (tree, num) for tree, num in product(
            [rbt.RBTree, avl.AVLtree], [800_000]
        )
    )
)
def test_bench_find(benchmark, tree, num):
    data = tree(range(num))
    benchmark(_find_test, data)
    assert True
