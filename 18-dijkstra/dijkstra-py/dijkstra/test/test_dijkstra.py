from dijkstra.dijkstra import Graph
import pytest


dataset = [
    {
        'searches': [
            {'q': (0, 4),
             'expected': '(20, [(0)->(2), (2)->(5), (5)->(4)])'},
            {'q': (5, 3),
             'expected': '(13, [(5)->(2), (2)->(3)])'},
            {'q': (1, 4),
             'expected': '(21, [(1)->(2), (2)->(5), (5)->(4)])'},
        ],
        'graph': [
            [(1, 7), (2, 9), (5, 14)],
            [(0, 7), (2, 10), (3, 15)],
            [(0, 9), (1, 10), (3, 11), (5, 2)],
            [(1, 15), (2, 11), (4, 6)],
            [(3, 6), (5, 9)],
            [(0, 14), (2, 2), (4, 9)],
        ],
    },
    {
        'searches': [
            {'q': (0, 8),
             'expected': '(14, [(0)->(1), (1)->(2), (2)->(8)])'},
            {'q': (3, 7),
             'expected': '(14, [(3)->(2), (2)->(5), (5)->(6), (6)->(7)])'},
            {'q': (5, 0),
             'expected': '(11, [(5)->(6), (6)->(7), (7)->(0)])'},
        ],
        'graph': [
            [(1, 4), (7, 8)],
            [(0, 4), (2, 8), (7, 11)],
            [(1, 8), (3, 7), (8, 2), (5, 4)],
            [(2, 7), (4, 9), (5, 14)],
            [(3, 9), (5, 10)],
            [(4, 10), (3, 14), (2, 4), (6, 2)],
            [(5, 2), (8, 6), (7, 1)],
            [(6, 1), (0, 8), (8, 7)],
            [(7, 7), (6, 6), (2, 2)],
        ],
    },
]


@pytest.mark.parametrize(
    'variant, graph, search',
    (
        (variant, ds['graph'], search)
        for variant in [Graph]
        for ds in dataset
        for search in ds['searches']
    )
)
def test_simple(variant, graph, search):
    src, dst = search['q']
    g = variant(graph)
    result = repr(g.dijkstra(src, dst))
    assert result == search['expected']
