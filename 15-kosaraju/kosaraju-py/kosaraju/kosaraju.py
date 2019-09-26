from typing import List, Set

VertexId = int
Graph = List[List[VertexId]]
SCCs = List[Set[VertexId]]


class Graph:
    def __init__(self, g: Graph):
        self._graph = g
        self._rgraph = [[] for _ in range(len(g))]
        for v, adj_lst in enumerate(self._graph):
            for av in adj_lst:
                self._rgraph[av].append(v)

    def kosaraju(self) -> SCCs:
        visited = [False] * len(self._graph)
        stack = []
        result = []
        for vtx in range(len(self._graph)):
            if visited[vtx] is True:
                continue
            self._dfs(self._rgraph, vtx, visited, stack)
        visited = [False] * len(self._graph)
        while stack:
            vtx = stack.pop()
            if visited[vtx] is True:
                continue
            scc = []
            self._dfs(self._graph, vtx, visited, scc)
            result.append(set(scc))
        return result

    @staticmethod
    def _dfs(g: Graph, v: VertexId, visited: List[bool],
             stack: List[VertexId]):
        visited[v] = True
        for av in g[v]:
            if visited[av] is True:
                continue
            Graph._dfs(g, av, visited, stack)
        stack.append(v)
