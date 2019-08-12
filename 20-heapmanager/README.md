# HW20 otus-algorithms

Homework contains implementation of the SLAB-allocator/cache.

### Install and run tests

#### Install
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/20-heapmanager
```

#### run quick tests/benchmarks
```
# Linux 4.18.0-25-generic

$ clang++ --version
clang version 7.0.0-3 (tags/RELEASE_700/final)
Target: x86_64-pc-linux-gnu

$ clang++-7 -Wall -O2 slab_allocator.cpp && ./a.out
allocator: malloc, blk_size:      8, iterations: 10000000, elapsed (s): 0.118020, ns/iter: 11.802007
allocator:   slab, blk_size:      8, iterations: 10000000, elapsed (s): 0.186939, ns/iter: 18.693881
allocator: malloc, blk_size:     16, iterations: 10000000, elapsed (s): 0.111361, ns/iter: 11.136072
allocator:   slab, blk_size:     16, iterations: 10000000, elapsed (s): 0.186304, ns/iter: 18.630373
allocator: malloc, blk_size:     32, iterations: 10000000, elapsed (s): 0.112578, ns/iter: 11.257801
allocator:   slab, blk_size:     32, iterations: 10000000, elapsed (s): 0.187632, ns/iter: 18.763229
allocator: malloc, blk_size:     64, iterations: 10000000, elapsed (s): 0.120214, ns/iter: 12.021361
allocator:   slab, blk_size:     64, iterations: 10000000, elapsed (s): 0.194200, ns/iter: 19.419974
allocator: malloc, blk_size:    128, iterations: 10000000, elapsed (s): 0.329630, ns/iter: 32.962971
allocator:   slab, blk_size:    128, iterations: 10000000, elapsed (s): 0.199376, ns/iter: 19.937582
allocator: malloc, blk_size:    256, iterations: 10000000, elapsed (s): 0.606366, ns/iter: 60.636618
allocator:   slab, blk_size:    256, iterations: 10000000, elapsed (s): 0.213266, ns/iter: 21.326638
allocator: malloc, blk_size:    512, iterations: 10000000, elapsed (s): 1.155619, ns/iter: 115.561935
allocator:   slab, blk_size:    512, iterations: 10000000, elapsed (s): 0.269587, ns/iter: 26.958689
allocator: malloc, blk_size:   1024, iterations: 10000000, elapsed (s): 2.351731, ns/iter: 235.173144
allocator:   slab, blk_size:   1024, iterations: 10000000, elapsed (s): 0.368089, ns/iter: 36.808909
allocator: malloc, blk_size:   2048, iterations: 10000000, elapsed (s): 4.663243, ns/iter: 466.324346
allocator:   slab, blk_size:   2048, iterations: 10000000, elapsed (s): 0.481380, ns/iter: 48.138020
allocator: malloc, blk_size:   4096, iterations: 10000000, elapsed (s): 8.891674, ns/iter: 889.167440
allocator:   slab, blk_size:   4096, iterations: 10000000, elapsed (s): 0.537200, ns/iter: 53.719991
allocator: malloc, blk_size:   8192, iterations: 10000000, elapsed (s): 9.077236, ns/iter: 907.723564
allocator:   slab, blk_size:   8192, iterations: 10000000, elapsed (s): 0.546425, ns/iter: 54.642460

```

#### valgrind
```
$ valgrind ./a.out > /dev/null

==15497== Memcheck, a memory error detector
==15497== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
==15497== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
==15497== Command: ./a.out
==15497== 
==15497== 
==15497== HEAP SUMMARY:
==15497==     in use at exit: 0 bytes in 0 blocks
==15497==   total heap usage: 550,087 allocs, 550,087 frees, 835,703,168 bytes allocated
==15497== 
==15497== All heap blocks were freed -- no leaks are possible
==15497== 
==15497== For counts of detected and suppressed errors, rerun with: -v
==15497== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

```
