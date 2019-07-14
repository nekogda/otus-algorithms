# HW10 otus-algorithms

Homework contains implementations of rbtree on rustlang (insert and find).

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/10-rbtree/rbtree-rs
$ rustup default nightly
```

#### run quick tests
```
$ cargo test -v
```
#### benchmark insert (1e6 elements)
```
$ cargo bench -v
```
```
== skip ==
...
test tests::bench_insert ... bench: 773,508,507 ns/iter (+/- 16,964,624)
```
