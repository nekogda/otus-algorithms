# HW06 otus-algorithms

Homework contains implementations of heap sort algorithm (with different grown algorithms) and heap-based PriorityQueue.

heapsort.remove_min - for removing min element of heap

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/06-heapsort
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
