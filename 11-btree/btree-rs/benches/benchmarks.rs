extern crate btree_rs;
extern crate env_logger;
extern crate log;
extern crate rand;
use btree_rs::btree;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

fn log_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

fn setup(path: &std::path::Path) {
    let _ = std::fs::remove_file(path);
}

fn bench_base(block_size: u32, n: u32, cache_size: usize) {
    log_init();
    let mut rng = rand::thread_rng();
    let mut v: Vec<u32> = (0..n).collect();
    let mut r: Vec<bool> = vec![false; n as usize];
    v.shuffle(&mut rng);
    let mut remove_num = n / 100 * 20;
    while remove_num > 0 {
        let idx: usize = rng.gen_range(0, n as usize);
        if r[idx] == true {
            continue;
        }
        r[idx] = true;
        remove_num -= 1;
    }
    let path = std::path::Path::new("test.idx");
    {
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), block_size, 2, n, 100);
        for i in &v {
            let _ = bt.insert(*i, i * 10 + i);
        }
        bt.flush_cache();
    }

    let bt = btree::Btree::load(path, block_size, n, cache_size);

    let now = Instant::now();
    for i in 0..(n as usize) {
        if r[i] {
            let result = bt.remove(v[i]);
            result.unwrap();
        } else {
            let result = bt.find(v[i]);
            result.unwrap();
        }
    }
    println!(
        "N = {}, cache_size = {:5}, Elapsed time: {:6} ms",
        n,
        cache_size,
        now.elapsed().as_millis()
    );
    let _ = bt.compact();
}

fn main() {
    bench_base(512, 10_000, 0);
    bench_base(512, 10_000, 1);
    bench_base(512, 10_000, 10);
    bench_base(512, 10_000, 100);
    bench_base(512, 10_000, 1000);
}
