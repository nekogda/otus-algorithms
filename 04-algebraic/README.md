# HW04 otus-algorithms

Homework contains implementations of: GCD, Pow, Prime, Fibonacci algorithms

### Install and run tests

#### Install and activate pipenv
```
$ pip3 install pipenv
$ pipenv install
$ pipenv shell
```

#### run quick tests
```
$ pytest --benchmark-skip
```
#### run benchmarks
```
$ pytest
# or benchmark per package
$ pytest { pkg_name }
```

### Results

#### GCD: a=123456789, b=12
```
platform linux -- Python 3.6.8, pytest-4.5.0, py-1.8.0, pluggy-0.11.0
benchmark: 3.2.2 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
plugins: benchmark-3.2.2

--------------------------------------------- benchmark: 3 tests ---------------------------------------------
Name (time in ns)                        Min                         Max                        Mean          
--------------------------------------------------------------------------------------------------------------
test_gcd_bench[gcd_rec]             469.6514 (1.0)            4,377.2015 (1.0)              498.9678 (1.0)    
test_gcd_bench[gcd_mod]             643.6700 (1.37)          16,521.6625 (3.77)             693.6586 (1.39)   
test_gcd_bench[gcd_min]     879,474,190.9951 (>1000.0)  934,541,235.0376 (>1000.0)  901,372,351.1947 (>1000.0)
--------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean

```

#### Pow: n=1, m=100_000_000
```
platform linux -- Python 3.6.8, pytest-4.5.0, py-1.8.0, pluggy-0.11.0
benchmark: 3.2.2 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
plugins: benchmark-3.2.2

------------------------------------------------- benchmark: 3 tests -------------------------------------------------
Name (time in us)                                    Min                       Max                      Mean          
----------------------------------------------------------------------------------------------------------------------
test_pow_bench[pow_bin_decomposition]             3.5570 (1.0)             91.6660 (1.0)              4.0272 (1.0)    
test_pow_bench[pow_2nm]                   1,417,140.0050 (>1000.0)  1,430,479.3440 (>1000.0)  1,421,407.0846 (>1000.0)
test_pow_bench[pow_iterative]             4,688,160.2940 (>1000.0)  4,718,893.8690 (>1000.0)  4,700,616.6382 (>1000.0)
----------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean

```

#### Prime numbers: n=50_000
```
platform linux -- Python 3.6.8, pytest-4.5.0, py-1.8.0, pluggy-0.11.0
benchmark: 3.2.2 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
plugins: benchmark-3.2.2

--------------------------------------------- benchmark: 4 tests ---------------------------------------------
Name (time in ms)                                    Min                   Max                  Mean          
--------------------------------------------------------------------------------------------------------------
test_prime_bench[prime_sieve]                     8.9353 (1.0)          9.9091 (1.0)          9.2712 (1.0)    
test_prime_bench[prime_sieve_bin]                27.4411 (3.07)        28.7247 (2.90)        27.7913 (3.00)   
test_prime_bench[prime_divider_optimized]        42.3784 (4.74)        44.4886 (4.49)        42.9747 (4.64)   
test_prime_bench[prime_divider]               8,271.8155 (925.75)   8,473.3588 (855.11)   8,320.1607 (897.42) 
--------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean

```

#### Fibonacci numbers: n=30
```
platform linux -- Python 3.6.8, pytest-4.5.0, py-1.8.0, pluggy-0.11.0
benchmark: 3.2.2 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
plugins: benchmark-3.2.2

-------------------------------------------------- benchmark: 5 tests --------------------------------------------------
Name (time in ns)                                  Min                         Max                        Mean
------------------------------------------------------------------------------------------------------------------------
test_fib_bench[fib_recursive_acc]             385.9750 (1.0)            8,307.9794 (1.0)              447.9498 (1.0)
test_fib_bench[fib_golden_ration]             851.9855 (2.21)          14,101.0387 (1.70)             960.0064 (2.14)
test_fib_bench[fib_iterative]               2,101.5003 (5.44)           9,440.4968 (1.14)           2,238.4349 (5.00)
test_fib_bench[fib_matrix]                 14,059.0128 (36.42)         39,762.9919 (4.79)          14,736.2702 (32.90)
test_fib_bench[fib_recursive]         285,937,067.0049 (>1000.0)  290,454,688.0163 (>1000.0)  288,430,649.0072 (>1000.0)
------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```
