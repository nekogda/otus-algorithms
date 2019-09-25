from typing import List

VertexId = int
Level = List[VertexId]
Graph = List[List[VertexId]]


class Graph:
    def __init__(self, data: 'Graph'):
        self._graph = data

    def tsort(self) -> List[Level]:
        vdegrees = [0] * len(self._graph)
        for adjs in self._graph:
            for adj_id in adjs:
                vdegrees[adj_id] += 1

        level = [v_id for v_id, ind in enumerate(vdegrees) if ind == 0]
        result = [level]
        count = len(level)
        while count < len(self._graph):
            new_level = []
            for v_id in level:
                for adj in self._graph[v_id]:
                    vdegrees[adj] -= 1
                    if vdegrees[adj] == 0:
                        new_level.append(adj)
                        count += 1
            if len(new_level) == 0:
                raise ValueError("Loop detected in the graph")
            result.append(new_level)
            level = new_level
        return result
