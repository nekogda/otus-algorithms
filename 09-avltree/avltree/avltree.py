from __future__ import annotations
import enum
from typing import List, TypeVar, Tuple


T = TypeVar('T')


class Node:
    def __init__(self, value: T, height: int = 1,
                 left: Node = None, right: Node = None):
        self.value = value
        self.height = height
        self.left = left
        self.right = right

    def copy_from(self, other: Node):
        self.value = other.value
        self.height = other.height
        self.left = other.left
        self.right = other.right

    def __str__(self) -> str:
        left, right = "None", "None"
        if self.left is not None:
            left = f"v{self.left.value}h{self.left.height}"
        if self.right is not None:
            right = f"v{self.right.value}h{self.right.height}"
        return f"{left}-[v{self.value}h{self.height}]-{right}"

    def is_leaf(self) -> bool:
        if self.left is None and self.right is None:
            return True
        return False


class direction(enum.Enum):
    left = 1
    right = 2


class PathNode:
    def __init__(self, node: Node, prev: Node, d: direction):
        self.node = node
        self.prev = prev
        self.direction = d

    def update_prev(self, node: Node):
        if self.direction == direction.left:
            self.prev.left = node
        else:
            self.prev.right = node
        self.node = node


class AVLtree:
    def __init__(self, s: List[T] = None):
        if s is None:
            s = []
        self.root = None
        self.len_ = 0
        for v in s:
            self.insert(v)
            self.len_ += 1

    def __len__(self) -> int:
        return self.len_

    def insert(self, v: T):
        path, node = self._find_helper(v, trace=True)
        if node is not None:
            raise ValueError("Duplicate found")
        n = Node(v)
        if self.root is None:
            self.root = n
        else:
            path[-1].update_prev(n)
            rebalance(path)

    def _find_helper(self, v: T, *,
                     trace: bool) -> Tuple(List[PathNode], Node):
        pnode = PathNode(self.root, None, None)
        path = []
        while True:
            if trace:
                path.append(pnode)
            # stop searching if we found node or end of the tree
            if pnode.node is None or pnode.node.value == v:
                return (path, pnode.node)
            elif v > pnode.node.value:
                pnode = PathNode(pnode.node.right, pnode.node, direction.right)
            else:
                pnode = PathNode(pnode.node.left, pnode.node, direction.left)

    def find(self, v: T) -> Node:
        _, node = self._find_helper(v, trace=False)
        return node

    def remove(self, v: T) -> T:
        path, node = self._find_helper(v, trace=True)
        if node is None:
            raise ValueError("not found")
        self.len_ -= 1
        if len(path) == 1:
            if self.root.is_leaf():
                self.root = None
                return v
        if node.is_leaf():
            path[-1].update_prev(None)
            path.pop()
        else:
            # get min node
            min_node, d = _get_min(path)
            if d is direction.right:
                min_node.left = node.left
                min_node.right = node.right
            node.copy_from(min_node)
        rebalance(path)
        return v


def _get_min(path: List[PathNode]) -> Tuple(Node, direction):
    pnode = path[-1]
    assert pnode.node.is_leaf() is False, "leaf node"
    min_node = pnode.node.left
    if pnode.node.right is None:
        return min_node, direction.left

    pnode = PathNode(pnode.node.right, pnode.node, direction.right)
    path.append(pnode)
    while pnode.node.left is not None:
        pnode = PathNode(pnode.node.left, pnode.node, direction.left)
        path.append(pnode)

    min_node = pnode.node
    pnode.update_prev(pnode.node.right)
    path.pop()
    return min_node, direction.right


def get_height(node: Node) -> int:
    if node is None:
        return 0
    return node.height


def fix_height(node: Node):
    assert node is not None
    if get_height(node.left) > get_height(node.right):
        node.height = get_height(node.left) + 1
        return
    node.height = get_height(node.right) + 1


def get_balance(node: Node) -> int:
    assert node is not None
    return get_height(node.right) - get_height(node.left)


def rebalance(path: List[PathNode]):
    for pnode in path[::-1]:
        fix_height(pnode.node)
        if get_balance(pnode.node) == -2:
            if get_balance(pnode.node.left) > 0:
                big_right_rotation(pnode.node)
            else:
                small_right_rotation(pnode.node)
        elif get_balance(pnode.node) == 2:
            if get_balance(pnode.node.right) < 0:
                big_left_rotation(pnode.node)
            else:
                small_left_rotation(pnode.node)


def small_left_rotation(node: Node):
    root = node
    tmp = node.right
    node.right = tmp.left
    # copy old root to the left
    node_copy = Node(None)
    node_copy.copy_from(node)
    # copy new tree to the root
    tmp.left = node_copy
    root.copy_from(tmp)
    fix_height(root.left)
    fix_height(root)


def small_right_rotation(node: Node):
    root = node
    tmp = node.left
    node.left = tmp.right
    # copy old root to the left
    node_copy = Node(None)
    node_copy.copy_from(node)
    # copy new tree to the root
    tmp.right = node_copy
    root.copy_from(tmp)
    fix_height(root.right)
    fix_height(root)


def big_left_rotation(node: Node):
    small_right_rotation(node.right)
    small_left_rotation(node)


def big_right_rotation(node: Node):
    small_left_rotation(node.left)
    small_right_rotation(node)
