# HW22 otus-algorithms

Homework contains implementations of BM, BMH, KMP pattern searching algorithms.

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/22-pattern-searching/pattern-searching-py
$ pip3 install pipenv
$ pipenv install
```

#### run quick tests
```
$ pipenv run pytest -v --benchmark-skip
```
#### coverage report
```
$ pipenv run pytest --cov=psearching --cov-report=html --benchmark-skip && firefox htmlcov/index.html
```
#### benchmark
```
$ pipenv run pytest -v --benchmark-columns=min,max,mean --benchmark-group-by=param:name
```
##### benchmark results
```
----------------------------- benchmark 'name=end': 3 tests -----------------------------
Name (time in ms)                   Min                 Max                Mean
-----------------------------------------------------------------------------------------
test_bench_bm[end-ds8-KMP]      49.1790 (1.0)       58.1181 (1.0)       50.5839 (1.0)
test_bench_bm[end-ds7-BMH]      78.8410 (1.60)      83.9288 (1.44)      79.7499 (1.58)
test_bench_bm[end-ds6-BM]      122.9775 (2.50)     125.7093 (2.16)     124.0128 (2.45)
-----------------------------------------------------------------------------------------

----------------------------- benchmark 'name=full_suff': 3 tests ------------------------------
Name (time in ms)                          Min                 Max                Mean
------------------------------------------------------------------------------------------------
test_bench_bm[full_suff-ds10-BMH]      77.0546 (1.0)       83.7330 (1.0)       78.6974 (1.0)
test_bench_bm[full_suff-ds9-BM]       102.6130 (1.33)     105.4529 (1.26)     103.4220 (1.31)
test_bench_bm[full_suff-ds11-KMP]     416.0138 (5.40)     433.0950 (5.17)     422.1809 (5.36)
------------------------------------------------------------------------------------------------

----------------------------- benchmark 'name=middle': 3 tests -----------------------------
Name (time in ms)                      Min                 Max                Mean
--------------------------------------------------------------------------------------------
test_bench_bm[middle-ds3-BM]       40.8689 (1.0)       44.7268 (1.0)       41.6124 (1.0)
test_bench_bm[middle-ds5-KMP]      48.2105 (1.18)      55.1865 (1.23)      49.0642 (1.18)
test_bench_bm[middle-ds4-BMH]     193.5516 (4.74)     212.7434 (4.76)     199.5002 (4.79)
--------------------------------------------------------------------------------------------

----------------------------- benchmark 'name=part_suff': 3 tests ------------------------------
Name (time in ms)                          Min                 Max                Mean
------------------------------------------------------------------------------------------------
test_bench_bm[part_suff-ds12-BM]      234.5336 (1.0)      242.2775 (1.0)      237.0026 (1.0)
test_bench_bm[part_suff-ds13-BMH]     325.0096 (1.39)     340.1604 (1.40)     333.4696 (1.41)
test_bench_bm[part_suff-ds14-KMP]     666.4604 (2.84)     726.2705 (3.00)     694.3519 (2.93)
------------------------------------------------------------------------------------------------

----------------------------- benchmark 'name=start': 3 tests -----------------------------
Name (time in ms)                     Min                 Max                Mean
-------------------------------------------------------------------------------------------
test_bench_bm[start-ds0-BM]       33.6869 (1.0)       39.2447 (1.0)       34.5431 (1.0)
test_bench_bm[start-ds2-KMP]      46.1037 (1.37)      50.2555 (1.28)      46.8791 (1.36)
test_bench_bm[start-ds1-BMH]     283.3641 (8.41)     287.3433 (7.32)     285.4657 (8.26)
-------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean

```
