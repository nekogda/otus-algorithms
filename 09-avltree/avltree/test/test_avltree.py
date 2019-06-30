import avltree.avltree as avl
import pytest
from typing import List, TypeVar, Set


T = TypeVar('T')


def test_small_left():
    root = avl.Node(10)
    root.right = avl.Node(20)
    root.right.right = avl.Node(30)
    avl.small_left_rotation(root)
    assert root.value == 20
    assert root.left.value == 10
    assert root.right.value == 30


def test_small_right():
    root = avl.Node(10)
    root.left = avl.Node(5)
    root.left.left = avl.Node(1)
    avl.small_right_rotation(root)
    assert root.value == 5
    assert root.left.value == 1
    assert root.right.value == 10


def test_big_left():
    root = avl.Node(10)
    root.right = avl.Node(20)
    root.right.left = avl.Node(15)
    root.right.right = avl.Node(30)
    avl.big_left_rotation(root)
    assert root.value == 15
    assert root.left.value == 10
    assert root.right.value == 20
    assert root.right.right.value == 30


def test_big_right():
    root = avl.Node(30)
    root.left = avl.Node(20)
    root.left.right = avl.Node(25)
    root.left.left = avl.Node(10)
    avl.big_right_rotation(root)
    assert root.value == 25
    assert root.right.value == 30
    assert root.left.value == 20
    assert root.left.left.value == 10


def test_tree_init():
    tree = avl.AVLtree(None)
    assert tree.root is None


def test_remove_raises():
    tree = avl.AVLtree([3, 2, 1])
    with pytest.raises(ValueError):
        tree.remove(4)
    result = _compare_nodes(tree, [
        "v1h1-[v2h2]-v3h1",
        "None-[v1h1]-None",
        "None-[v3h1]-None",
        ])
    assert result


dataset_remove = [
    ([1], [], 1),
    ([1, 2, 3], [
        "v1h1-[v3h2]-None",
        "None-[v1h1]-None",
    ], 2),
    ([1, 2, 3], [
        "None-[v2h2]-v3h1",
        "None-[v3h1]-None",
    ], 1),
    ([1, 2, 3], [
        "v1h1-[v2h2]-None",
        "None-[v1h1]-None",
    ], 3),
    ([2, 1], [
        "None-[v1h1]-None",
    ], 2),
    ([12, 20, 6, 4, 10], [
        "v10h2-[v12h3]-v20h1",
        "v4h1-[v10h2]-None",
        "None-[v4h1]-None",
        "None-[v20h1]-None",
    ], 6),
    ([12, 20, 6, 4], [
        "v4h1-[v12h2]-v20h1",
        "None-[v4h1]-None",
        "None-[v20h1]-None",
    ], 6),
    ([12, 20, 6, 10, 25, 11, 4], [
        "v10h2-[v12h3]-v20h2",
        "v4h1-[v10h2]-v11h1",
        "None-[v4h1]-None",
        "None-[v11h1]-None",
        "None-[v20h2]-v25h1",
        "None-[v25h1]-None",
    ], 6),
    ([12, 6, 20, 25, 4, 10, 11, 8], [
        "v8h3-[v12h4]-v20h2",
        "v4h1-[v8h3]-v10h2",
        "None-[v4h1]-None",
        "None-[v10h2]-v11h1",
        "None-[v11h1]-None",
        "None-[v20h2]-v25h1",
        "None-[v25h1]-None",
    ], 6),
    ([12, 6, 20, 25, 15], [
        "v6h1-[v12h3]-v25h2",
        "None-[v6h1]-None",
        "v15h1-[v25h2]-None",
        "None-[v15h1]-None",
    ], 20),
    ([12, 6, 20, 15], [
        "v6h1-[v12h2]-v15h1",
        "None-[v6h1]-None",
        "None-[v15h1]-None",
    ], 20),
    ([12, 6, 20, 4, 25, 15, 17, 13], [
        "v6h2-[v12h4]-v15h3",
        "v4h1-[v6h2]-None",
        "None-[v4h1]-None",
        "v13h1-[v15h3]-v20h2",
        "None-[v13h1]-None",
        "v17h1-[v20h2]-None",
        "None-[v17h1]-None",
    ], 25),
]


dataset = [
    ([1, 2, 3], [
        "v1h1-[v2h2]-v3h1",
        "None-[v1h1]-None",
        "None-[v3h1]-None",
    ]),
    ([3, 2, 1], [
        "v1h1-[v2h2]-v3h1",
        "None-[v1h1]-None",
        "None-[v3h1]-None",
    ]),
    ([12, 6, 10, 20, 30, 15, 4], [
        "v6h2-[v12h3]-v20h2",
        "v4h1-[v6h2]-v10h1",
        "None-[v4h1]-None",
        "None-[v10h1]-None",
        "v15h1-[v20h2]-v30h1",
        "None-[v15h1]-None",
        "None-[v30h1]-None",
    ]),
    ([12, 6, 10, 4], [
        "v6h2-[v10h3]-v12h1",
        "v4h1-[v6h2]-None",
        "None-[v4h1]-None",
        "None-[v12h1]-None",
    ]),
    ([12, 20, 15, 30], [
        "v12h1-[v15h3]-v20h2",
        "None-[v12h1]-None",
        "None-[v20h2]-v30h1",
        "None-[v30h1]-None",
    ]),
    ([12, 6, 20, 4, 10], [
        "v6h2-[v12h3]-v20h1",
        "v4h1-[v6h2]-v10h1",
        "None-[v4h1]-None",
        "None-[v10h1]-None",
        "None-[v20h1]-None",
    ]),
]


@pytest.mark.parametrize(
    'test_data, expected',
    (
        (test_data, expected)
        for test_data, expected in dataset
    )
)
def test_insert(test_data, expected):
    tree = avl.AVLtree(test_data)
    assert _compare_nodes(tree, expected)
    assert len(tree) == len(expected)


def test_insert_raises():
    tree = avl.AVLtree([1])
    with pytest.raises(ValueError):
        tree.insert(1)


@pytest.mark.parametrize(
    'test_data, expected, removed',
    (
        (test_data, expected, removed)
        for test_data, expected, removed in dataset_remove
    )
)
def test_remove(test_data, expected, removed):
    tree = avl.AVLtree(test_data)
    tree.remove(removed)
    result = _compare_nodes(tree, expected)
    assert result
    assert len(tree) == len(expected)


@pytest.mark.parametrize(
    'test_data, query',
    (
        (test_data, query)
        for test_data, _, query in dataset_remove
    )
)
def test_find(test_data, query):
    tree = avl.AVLtree(test_data)
    result = tree.find(query)
    assert result.value == query


def _compare_nodes(t: avl.AVLtree, s: List[str]) -> bool:  # pragma: no cover
    dump = _dump_nodes(t)
    node_set = set()
    for n in dump:
        node_set.add(str(n))
    if len(node_set) != len(s):
        return False
    for n in s:
        if n not in node_set:
            return False
    return True


def _dump_nodes(t: avl.AVLtree) -> Set[avl.Node]:  # pragma: no cover
    result = set()
    if t.root is None:
        return set()
    node_stack = []
    node_stack.append(t.root)
    while len(node_stack) > 0:
        node = node_stack[-1]
        if node.left is not None and node.left not in result:
            if _in_node_stack(node_stack, node.left):
                raise Exception("loop detected")
            node_stack.append(node.left)
            continue
        if node.right is not None and node.right not in result:
            if _in_node_stack(node_stack, node.right):
                raise Exception("loop detected")
            node_stack.append(node.right)
            continue
        result.add(node)
        node_stack.pop()
    return result


def _in_node_stack(s: List[avl.Node], n: avl.Node) -> bool:  # pragma: no cover
    for node in s:
        if node is n:
            return True
    return False
