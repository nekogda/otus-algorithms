from __future__ import annotations
from typing import List, TypeVar, Set

T = TypeVar('T')


class Node:
    def __init__(self, value: T, *, left: Node = None, right: Node = None,
                 parent: Node = None, is_red: bool = True):
        self.value = value
        self.left = left
        self.right = right
        self.parent = parent
        self.is_red = is_red

    def __str__(self):
        left, right, parent = "None", "None", "None"
        if self.left:
            color = "R" if self.left.is_red else "B"
            left = f"Lv{self.left.value}c{color}pv{self.left.parent.value}"
        if self.right:
            color = "R" if self.right.is_red else "B"
            right = f"Rv{self.right.value}c{color}pv{self.right.parent.value}"
        if self.parent:
            parent = self.parent.value
        color = "R" if self.is_red else "B"
        return f"{left}-[v{self.value}c{color}pv{parent}]-{right}"

    @property
    def uncle(self) -> Node:
        if self.grandparent is None:
            return None
        if self.grandparent.left is None:
            return None
        if self.grandparent.left is self.parent:
            return self.grandparent.right
        else:
            return self.grandparent.left

    @property
    def grandparent(self) -> Node:
        return self.parent.parent if self.parent else None


class RBTree:
    def __init__(self, s: List[T] = None):
        if s is None:
            s = []
        self.root = None
        for v in s:
            self.insert(v)

    def insert(self, v: T):
        if self.root is None:
            self.root = Node(v)
            node = self.root
        else:
            node = self._insert_util(self.root, v)
        self.rebalance(node)

    def _insert_util(self, node: Node, v: T) -> Node:
        if v < node.value:
            if node.left is None:
                node.left = Node(v, parent=node)
                return node.left
            return self._insert_util(node.left, v)
        elif v > node.value:
            if node.right is None:
                node.right = Node(v, parent=node)
                return node.right
            return self._insert_util(node.right, v)
        else:
            raise ValueError("Duplicate found")

    def find(self, v: T) -> Node:
        return self._find_util(self.root, v)

    def _find_util(self, node: Node, v: T) -> Node:
        if node is None:
            return None
        if v < node.value:
            return self._find_util(node.left, v)
        if v > node.value:
            return self._find_util(node.right, v)
        return node

    def rebalance(self, node: Node):
        self._case_01(node)

    def _case_01(self, node: Node):
        if node.parent is None:
            node.is_red = False
        else:
            self._case_02(node)

    def _case_02(self, node: Node):
        if node.grandparent is None:
            return
        else:
            self._case_03(node)

    def _case_03(self, node: Node):
        if node.uncle and node.uncle.is_red:
            node.parent.is_red = False
            node.uncle.is_red = False
            node.grandparent.is_red = True
            self._case_01(node.grandparent)
        else:
            self._case_04(node)

    def _case_04(self, node: Node):
        if node.parent.right is node \
           and node.grandparent.left is node.parent:
            self.rotate_left(node.parent)
            self._case_05(node.left)
        elif node.parent.left is node \
                and node.grandparent.right is node.parent:
            self.rotate_right(node.parent)
            self._case_05(node.right)
        else:
            self._case_05(node)

    def _case_05(self, node: Node):
        node.parent.is_red = False
        node.grandparent.is_red = True
        if node.parent.left is node \
           and node.grandparent.left is node.parent:
            self.rotate_right(node.grandparent)
        elif node.parent.right is node \
                and node.grandparent.right is node.parent:
            self.rotate_left(node.grandparent)

    def rotate_left(self, pivot: Node):
        tmp = pivot.right
        pivot.right = tmp.left
        if tmp.left:
            tmp.left.parent = pivot
        tmp.parent = pivot.parent
        if pivot.parent:
            if pivot.parent.left and pivot is pivot.parent.left:
                pivot.parent.left = tmp
            else:
                pivot.parent.right = tmp
        else:
            self.root = tmp

        tmp.left = pivot
        pivot.parent = tmp

    def rotate_right(self, pivot: Node):
        tmp = pivot.left
        pivot.left = tmp.right
        if tmp.right:
            tmp.right.parent = pivot
        tmp.parent = pivot.parent
        if pivot.parent:
            if pivot.parent.right and pivot is pivot.parent.right:
                pivot.parent.right = tmp
            else:
                pivot.parent.left = tmp
        else:
            self.root = tmp

        tmp.right = pivot
        pivot.parent = tmp

    def compare(self, other: Set[str]) -> bool:  # pragma: no cover
        for i, v in enumerate(self.dump()):
            if str(v) not in other:
                return False
        if i + 1 != len(other):
            return False
        return True

    def dump(self) -> List[Node]:
        return self._dump_util(self.root)

    def _dump_util(self, node: Node):
        if node is None:
            return
        yield node
        yield from self._dump_util(node.left)
        yield from self._dump_util(node.right)
