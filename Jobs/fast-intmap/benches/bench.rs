#![feature(test)]

extern crate test;

use std::collections::HashMap;

use fast_intmap::IntMapU64;
use test::Bencher;

#[bench]
fn bench_insert(b: &mut Bencher) {
    let mut map = IntMapU64::default();
    b.iter(|| {
        for i in 0..1000 {
            map.insert(i, i);
        }
    });
}

#[bench]
fn bench_insert_std(b: &mut Bencher) {
    let mut map: HashMap<u64, u64> = HashMap::default();
    b.iter(|| {
        for i in 0..1000 {
            map.insert(i, i);
        }
    });
}

#[bench]
fn bench_get(b: &mut Bencher) {
    let mut map = IntMapU64::default();
    for i in 0..1000 {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..1000 {
            assert!(map.get(i).is_some());
        }
    });
}

#[bench]
fn bench_get_std(b: &mut Bencher) {
    let mut map: HashMap<u64, u64> = HashMap::default();
    for i in 0..1000 {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..1000 {
            assert!(map.get(&i).is_some());
        }
    });
}
