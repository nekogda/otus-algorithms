# HW22 otus-algorithms

Homework contains implementations of BM, BMH, KMP pattern searching algorithms.

### Install and run tests

#### Install and activate pipenv
```
$ git clone https://github.com/nekogda/otus-algorithms
$ cd otus-algorithms/22-pattern-searching/pattern-searching-rs
$ rustup default nightly
```

#### run quick tests
```
$ cargo test -v
```
#### benchmark
```
$ cargo bench -v
```
```
== skip ==
...
test tests::bench_bm_regular    ... bench:   5,563,160 ns/iter (+/- 561,249)
test tests::bench_bmh_regular   ... bench:   2,932,214 ns/iter (+/- 314,306)
test tests::bench_kmp_regular   ... bench: 135,168,124 ns/iter (+/- 10,850,576)
test tests::fib::bm             ... bench:   2,843,395 ns/iter (+/- 30,124)
test tests::fib::bmh            ... bench:   2,303,943 ns/iter (+/- 34,433)
test tests::fib::kmp            ... bench:   1,546,460 ns/iter (+/- 28,755)
test tests::full_suffix::bm     ... bench:      49,816 ns/iter (+/- 1,233)
test tests::full_suffix::bmh    ... bench:      43,311 ns/iter (+/- 1,579)
test tests::full_suffix::kmp    ... bench:     214,232 ns/iter (+/- 17,457)
test tests::partial_suffix::bm  ... bench:      67,430 ns/iter (+/- 2,719)
test tests::partial_suffix::bmh ... bench:     109,840 ns/iter (+/- 6,102)
test tests::partial_suffix::kmp ... bench:     210,149 ns/iter (+/- 6,411)
test tests::synth_end::bm       ... bench:     556,144 ns/iter (+/- 5,871)
test tests::synth_end::bmh      ... bench:     452,547 ns/iter (+/- 12,415)
test tests::synth_end::kmp      ... bench:     211,401 ns/iter (+/- 15,421)
test tests::synth_middle::bm    ... bench:     166,627 ns/iter (+/- 14,309)
test tests::synth_middle::bmh   ... bench:   1,059,291 ns/iter (+/- 175,109)
test tests::synth_middle::kmp   ... bench:     213,631 ns/iter (+/- 13,012)
test tests::synth_start::bm     ... bench:     159,592 ns/iter (+/- 11,775)
test tests::synth_start::bmh    ... bench:   1,627,028 ns/iter (+/- 119,218)
test tests::synth_start::kmp    ... bench:     201,315 ns/iter (+/- 5,527)

```
