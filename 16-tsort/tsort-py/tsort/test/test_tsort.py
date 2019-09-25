from tsort.tsort import Graph
import pytest

dataset = [
    {
        'ordered': [
            set([7, 4]), set([1, 8, 9]), set([13, 0, 6]),
            set([5]), set([3, 10, 11, 12]), set([2]),
        ],
        'graph': [
            [12, 2], [12], [], [2], [2, 8, 9], [3, 10, 11, 12],
            [10], [1, 3, 5, 6], [13], [0, 6, 11], [2], [], [2], [5],
        ],
    },
    {
        'ordered': [set([1, 7, 2]), set([5, 0, 8]), set([6, 3]), set([4])],
        'graph': [[6], [4, 5], [8, 6], [4], [], [6], [], [0, 3], [3]],
    },
    {
        'ordered': [
            set([3]), set([0, 5]), set([6, 10, 2]), set([7, 1]),
            set([4, 8]), set([9])],
        'graph': [
            [2, 6, 9, 10], [8], [7, 1],
            [0, 4, 5], [9], [2], [], [4], [9], [], [7]],
    },
]


@pytest.mark.parametrize(
    'variant, ds',
    (
        (variant, ds)
        for variant in [Graph]
        for ds in dataset
    )
)
def test_base(variant, ds):
    g = variant(ds['graph'])
    result = [set(lvl) for lvl in g.tsort()]
    assert result == ds['ordered']


def test_loop():
    graph = [[2, 6, 9, 10], [8], [7, 1],
             [0, 4, 5], [9], [2], [], [4, 0], [9], [], [7]]
    g = Graph(graph)
    with pytest.raises(ValueError):
        g.tsort()
