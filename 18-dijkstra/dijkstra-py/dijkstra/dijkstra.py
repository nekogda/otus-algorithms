import sys
from typing import List, Tuple, Generator

VertexId = int
HeapIndex = int
Distance = int
AdjVertex = Tuple[VertexId, Distance]
GraphItem = List[AdjVertex]
Graph = List[GraphItem]
Path = List['Edge']


class Graph:
    def __init__(self, data: 'Graph'):
        self._heap = Heap()
        self._graph = []
        self._parents = []

        for adj_list in data:
            self.add_vertex(adj_list)

    def add_vertex(self, adj_list: GraphItem):
        self._graph.append(adj_list)
        self._parents.append(None)
        self._heap.push(len(self._graph) - 1, sys.maxsize)

    def dijkstra(self, src: VertexId, dst: VertexId) -> Tuple[Distance, Path]:
        if src == dst:
            raise ValueError("src must be non equal to dst")
        if len(self._graph) == 0:
            raise IndexError("can't call search on empty graph")

        self._heap.set_dist(src, 0)
        while True:
            min_vtx_id, dist = self._heap.pop()
            for adj_vtx_id, cost in self._graph[min_vtx_id]:
                if self._heap.visited(adj_vtx_id):
                    continue
                new_dist = dist + cost
                if new_dist < self._heap.get_dist(adj_vtx_id):
                    self._heap.set_dist(adj_vtx_id, new_dist)
                    self._parents[adj_vtx_id] = min_vtx_id

            if self._heap.visited(dst):
                return self.get_path(src, dst)

    def get_path(self, src: VertexId, dst: VertexId) -> Tuple[Distance, Path]:
        path = []
        next_step = self._parents[dst]
        step = dst
        while next_step is not None:
            path.append(Edge(next_step, step))
            step = next_step
            next_step = self._parents[step]

        path.reverse()
        return self._heap.get_dist(dst), path


class Edge:
    def __init__(self, v1: VertexId, v2: VertexId):
        self.v1 = v1
        self.v2 = v2

    def __repr__(self) -> str:
        return f"({self.v1})->({self.v2})"


class HeapItem:
    def __init__(self, v: VertexId, d: Distance):
        self.vtx_id = v
        self.distance = d

    def __lt__(self, other: 'HeapItem') -> bool:
        return self.distance < other.distance

    def __le__(self, other: 'HeapItem') -> bool:
        return self.distance <= other.distance

    def __ge__(self, other: 'HeapItem') -> bool:
        return self.distance >= other.distance

    def __repr__(self) -> str:
        return f'v:{self.vtx_id}, d:{self.distance}'


class Heap:
    def __init__(self):
        self._heap = []
        self._len = 0
        self._vhmap = []

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return len(self) == 0

    def _drown(self, node_idx: HeapIndex):
        for child_idx in self._childs(node_idx):
            if self._heap[node_idx] <= self._heap[child_idx]:
                break
            self._swap(node_idx, child_idx)
            node_idx = child_idx

    def _raise(self, node_idx: HeapIndex):
        for parent_idx in self._parents(node_idx):
            if self._heap[node_idx] >= self._heap[parent_idx]:
                break
            self._swap(node_idx, parent_idx)
            node_idx = parent_idx

    def pop(self) -> Tuple[VertexId, Distance]:
        self._swap(0, len(self) - 1)
        self._len -= 1
        self._drown(0)
        item = self._heap[len(self)]
        return item.vtx_id, item.distance

    def _childs(self, node_idx: HeapIndex) -> Generator[HeapIndex, None, None]:
        while True:
            left = 2 * node_idx + 1
            right = 2 * node_idx + 2
            childs = [child for child in (left, right) if child < len(self)]
            next_child = min(
                childs, default=None, key=lambda x: self._heap[x].distance
            )
            if next_child is None:
                break
            yield next_child
            node_idx = next_child

    def _parents(self, idx: HeapIndex) -> Generator[HeapIndex, None, None]:
        while True:
            parent = (idx - 1) // 2
            if parent >= 0 \
               and self._heap[idx] < self._heap[parent]:
                yield parent
                idx = parent
            else:
                break

    def visited(self, v: VertexId) -> bool:
        return self._vhmap[v] >= len(self)

    def _get_item(self, v: VertexId) -> HeapItem:
        return self._heap[self._vhmap[v]]

    def get_dist(self, v: VertexId) -> Distance:
        return self._get_item(v).distance

    def set_dist(self, v: VertexId, value: Distance):
        heap_idx = self._vhmap[v]
        assert(value < self._heap[heap_idx].distance)
        self._heap[heap_idx].distance = value
        self._raise(heap_idx)

    def push(self, v: VertexId, d: Distance):
        if (len(self) != len(self._heap)):
            raise ValueError("pop was used earlier, can't push new items")
        self._heap.append(HeapItem(v, d))
        self._vhmap.append(v)
        self._len += 1
        self._raise(self._len - 1)

    def _swap(self, h1: HeapIndex, h2: HeapIndex):
        v1 = self._heap[h1].vtx_id
        v2 = self._heap[h2].vtx_id
        self._heap[h1], self._heap[h2] = self._heap[h2], self._heap[h1]
        self._vhmap[v1], self._vhmap[v2] = self._vhmap[v2], self._vhmap[v1]
