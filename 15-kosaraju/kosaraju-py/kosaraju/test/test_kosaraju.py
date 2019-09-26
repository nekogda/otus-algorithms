from kosaraju.kosaraju import Graph
from . import dataset as lds
import pytest
import sys
sys.setrecursionlimit(2000)

test_datasets = [
    {
        'components': [
            set([5, 4, 3]), set([9, 8, 7, 6]), set([10]), set([2, 1, 0])
        ],
        'graph': [[1], [2, 3], [0], [4], [5], [3], [5, 7], [8], [9], [6], [9]],
    },
    {
        'components': [
            set([8, 7]), set([5, 6, 4, 3, 2, 0]),
            set([10, 12, 9, 11]), set([1])
        ],
        'graph': [
            [2], [0], [3, 4], [2, 4], [5, 6], [0, 3],
            [0, 7], [8], [7], [6, 8, 12], [9], [4, 9], [10, 11]
        ],
    },
    {
        'components': [set([4]), set([3]), set([1, 2, 0])],
        'graph':  [[2, 3], [0], [1], [4], []],
    },
    {
        'components': lds.result1,
        'graph': lds.dataset1,
    },
    {
        'components': lds.result2,
        'graph': lds.dataset2,
    },

]


@pytest.mark.parametrize(
    'variant, ds',
    (
        (variant, ds)
        for variant in [Graph]
        for ds in test_datasets
    )
)
def test_base(variant, ds):
    g = variant(ds['graph'])
    for scc in g.kosaraju():
        assert scc in ds['components']
