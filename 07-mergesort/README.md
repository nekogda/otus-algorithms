# HW07 otus-algorithms

Homework contains implementations of mergesort algorithm.
* merge_sort_base: base version without optimisations
* merge_sort_insertion: base merge sort with optimisation (using insertion sort on leafs)
* merge_sort_threaded: parallel version of mergesort. Uses python threading (doesn't work well because of GIL). Just for example.
* merge_sort_tim: merge sort with following optimisations
  * Find ascending and descending sequences, and uses their order
  * Merge run's
  * Galloping mode (using bisect module).

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/07-mergesort
$ pip3 install pipenv
$ pipenv install
```

#### run quick tests
```
$ pipenv run pytest -v --benchmark-skip
```
#### run benchmarks
```
$ pipenv run pytest -v --benchmark-columns=min,max,mean
```

#### Result (100_000 items)

```
---------------------------------------------- benchmark 'dataset_name=100000_rnd_100_pct': 4 tests ----------------------------------------------
Name (time in ms)                                                                        Min                   Max                  Mean
--------------------------------------------------------------------------------------------------------------------------------------------------
test_bench_mergesorts[merge_sort_insertion-100000_rnd_100_pct-dataset_value9]       516.7692 (1.0)        610.5912 (1.0)        539.6337 (1.0)
test_bench_mergesorts[merge_sort_base-100000_rnd_100_pct-dataset_value11]           648.5463 (1.26)       692.1687 (1.13)       676.2017 (1.25)
test_bench_mergesorts[merge_sort_threaded-100000_rnd_100_pct-dataset_value10]     1,036.5199 (2.01)     1,100.9383 (1.80)     1,074.6042 (1.99)
test_bench_mergesorts[merge_sort_tim-100000_rnd_100_pct-dataset_value8]           1,103.3701 (2.14)     1,364.8440 (2.24)     1,193.5694 (2.21)
--------------------------------------------------------------------------------------------------------------------------------------------------

------------------------------------------- benchmark 'dataset_name=100000_rnd_10_pct': 4 tests -------------------------------------------
Name (time in ms)                                                                     Min                 Max                Mean
-------------------------------------------------------------------------------------------------------------------------------------------
test_bench_mergesorts[merge_sort_tim-100000_rnd_10_pct-dataset_value4]           406.5891 (1.0)      439.9537 (1.0)      416.6013 (1.0)
test_bench_mergesorts[merge_sort_insertion-100000_rnd_10_pct-dataset_value5]     440.3671 (1.08)     859.3097 (1.95)     565.1150 (1.36)
test_bench_mergesorts[merge_sort_base-100000_rnd_10_pct-dataset_value7]          636.6838 (1.57)     757.2629 (1.72)     691.6943 (1.66)
test_bench_mergesorts[merge_sort_threaded-100000_rnd_10_pct-dataset_value6]      890.8310 (2.19)     976.2386 (2.22)     954.4401 (2.29)
-------------------------------------------------------------------------------------------------------------------------------------------

------------------------------------------- benchmark 'dataset_name=100000_rnd_5': 4 tests -------------------------------------------
Name (time in ms)                                                                Min                 Max                Mean
--------------------------------------------------------------------------------------------------------------------------------------
test_bench_mergesorts[merge_sort_tim-100000_rnd_5-dataset_value0]            61.9684 (1.0)       76.0323 (1.0)       66.4355 (1.0)
test_bench_mergesorts[merge_sort_insertion-100000_rnd_5-dataset_value1]     368.7405 (5.95)     386.8134 (5.09)     379.6956 (5.72)
test_bench_mergesorts[merge_sort_base-100000_rnd_5-dataset_value3]          541.7187 (8.74)     580.8459 (7.64)     568.6775 (8.56)
test_bench_mergesorts[merge_sort_threaded-100000_rnd_5-dataset_value2]      839.1476 (13.54)    897.2871 (11.80)    861.8680 (12.97)
--------------------------------------------------------------------------------------------------------------------------------------

-------------------------------------------- benchmark 'dataset_name=100000_rnd_R': 4 tests -------------------------------------------
Name (time in ms)                                                                 Min                 Max                Mean
---------------------------------------------------------------------------------------------------------------------------------------
test_bench_mergesorts[merge_sort_tim-100000_rnd_R-dataset_value12]            37.0587 (1.0)       40.8603 (1.0)       38.1225 (1.0)
test_bench_mergesorts[merge_sort_insertion-100000_rnd_R-dataset_value13]     477.0697 (12.87)    482.6536 (11.81)    480.0931 (12.59)
test_bench_mergesorts[merge_sort_base-100000_rnd_R-dataset_value15]          571.0478 (15.41)    589.2599 (14.42)    579.4324 (15.20)
test_bench_mergesorts[merge_sort_threaded-100000_rnd_R-dataset_value14]      825.2260 (22.27)    896.7169 (21.95)    870.4355 (22.83)
---------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean

```
