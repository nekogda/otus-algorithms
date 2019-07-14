# HW10 otus-algorithms

Homework contains implementations of rbtree (insert and find).

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/10-rbtree/rbtree-py
$ pip3 install pipenv
$ pipenv install
$ export PYTHONPATH=../../09-avltree/
```

#### run quick tests
```
$ pipenv run pytest -v --benchmark-skip
```
#### coverage report
```
$ pipenv run pytest --cov=rbtree --cov-report=html --benchmark-skip && firefox htmlcov/index.html
```
#### benchmark
```
$ pipenv run pytest -v --benchmark-columns=min,max,mean --benchmark-group-by=param:num
```
##### benchmark insert
```
-------------------------------- benchmark 'num=10000': 2 tests -------------------------------
Name (time in ms)                         Min                 Max                Mean
-----------------------------------------------------------------------------------------------
test_bench_insert[RBTree-10000]      141.7796 (1.0)      177.3409 (1.0)      148.7859 (1.0)
test_bench_insert[AVLtree-10000]     336.4786 (2.37)     342.9656 (1.93)     338.7638 (2.28)
-----------------------------------------------------------------------------------------------

---------------------------- benchmark 'num=100000': 2 tests -----------------------------
Name (time in s)                         Min               Max              Mean
------------------------------------------------------------------------------------------
test_bench_insert[RBTree-100000]      1.7669 (1.0)      1.8577 (1.0)      1.7983 (1.0)
test_bench_insert[AVLtree-100000]     4.1062 (2.32)     4.1761 (2.25)     4.1392 (2.30)
------------------------------------------------------------------------------------------

------------------------------ benchmark 'num=500000': 2 tests ------------------------------
Name (time in s)                          Min                Max               Mean
---------------------------------------------------------------------------------------------
test_bench_insert[RBTree-500000]       9.9654 (1.0)      10.2289 (1.0)      10.1037 (1.0)
test_bench_insert[AVLtree-500000]     23.4819 (2.36)     23.9250 (2.34)     23.7229 (2.35)
---------------------------------------------------------------------------------------------

------------------------------ benchmark 'num=1000000': 2 tests ------------------------------
Name (time in s)                           Min                Max               Mean
----------------------------------------------------------------------------------------------
test_bench_insert[RBTree-1000000]      20.7898 (1.0)      21.4274 (1.0)      21.1329 (1.0)
test_bench_insert[AVLtree-1000000]     49.1880 (2.37)     49.8430 (2.33)     49.4284 (2.34)
----------------------------------------------------------------------------------------------

```
##### benchmark find
```
------------------------------ benchmark 'num=800000': 2 tests -------------------------------
Name (time in ms)                        Min                 Max                Mean
----------------------------------------------------------------------------------------------
test_bench_find[RBTree-800000]      123.7526 (1.0)      139.1574 (1.0)      128.4009 (1.0)
test_bench_find[AVLtree-800000]     286.3387 (2.31)     315.7093 (2.27)     295.5626 (2.30)
----------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```
