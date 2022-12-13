#![feature(test)]

extern crate test;

// const TO: u64 = 1000;
const TO: u64 = 100000;
// const TO: u64 = 10000000;

#[bench]
fn std_insert(b: &mut test::Bencher) {
    let mut map: std::collections::HashMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn std_get(b: &mut test::Bencher) {
    let mut map: std::collections::HashMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..TO {
            assert!(map.get(&i).is_some());
        }
    });
}

#[bench]
fn hashbrown_insert(b: &mut test::Bencher) {
    let mut map: hashbrown::HashMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn hashbrown_get(b: &mut test::Bencher) {
    let mut map: hashbrown::HashMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..TO {
            assert!(map.get(&i).is_some());
        }
    });
}

#[bench]
fn ahash_insert(b: &mut test::Bencher) {
    let mut map: ahash::AHashMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn ahash_get(b: &mut test::Bencher) {
    let mut map: ahash::AHashMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..TO {
            assert!(map.get(&i).is_some());
        }
    });
}

#[bench]
fn nohash_insert(b: &mut test::Bencher) {
    let mut map: nohash_hasher::IntMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn nohash_get(b: &mut test::Bencher) {
    let mut map: nohash_hasher::IntMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..TO {
            assert!(map.get(&i).is_some());
        }
    });
}

#[bench]
fn rayon_std_get(b: &mut test::Bencher) {
    use rayon::prelude::*;
    let mut map: std::collections::HashMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        map.par_iter().for_each(|(k, _)| {
            assert!(map.get(k).is_some());
        });
    });
}

#[bench]
fn rayon_ahash_get(b: &mut test::Bencher) {
    use rayon::prelude::*;
    let mut map: ahash::AHashMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        map.par_iter().for_each(|(k, _)| {
            assert!(map.get(k).is_some());
        });
    });
}

#[bench]
fn rayon_nohash_get(b: &mut test::Bencher) {
    use rayon::prelude::*;
    let mut map: nohash_hasher::IntMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        map.par_iter().for_each(|(k, _)| {
            assert!(map.get(k).is_some());
        });
    });
}
