#![feature(test)]

extern crate test;

const TO: u64 = 1000;
// const TO: u64 = 100000;
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
fn indexmap_insert(b: &mut test::Bencher) {
    let mut map: indexmap::IndexMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn indexmap_get(b: &mut test::Bencher) {
    let mut map: indexmap::IndexMap<u64, u64> = Default::default();
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
fn dashmap_insert(b: &mut test::Bencher) {
    let map: dashmap::DashMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn dashmap_get(b: &mut test::Bencher) {
    let map: dashmap::DashMap<u64, u64> = Default::default();
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
fn linkedhashmap_insert(b: &mut test::Bencher) {
    let mut map: linked_hash_map::LinkedHashMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn linkedhashmap_get(b: &mut test::Bencher) {
    let mut map: linked_hash_map::LinkedHashMap<u64, u64> = Default::default();
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
fn chasmap_insert(b: &mut test::Bencher) {
    let map: chashmap::CHashMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn chasmap_get(b: &mut test::Bencher) {
    let map: chashmap::CHashMap<u64, u64> = Default::default();
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
fn intmap_insert(b: &mut test::Bencher) {
    let mut map: intmap::IntMap<u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn intmap_get(b: &mut test::Bencher) {
    let mut map: intmap::IntMap<u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..TO {
            assert!(map.get(i).is_some());
        }
    });
}

// #[bench]
// fn flurry_insert(b: &mut test::Bencher) {
//     let mut map: flurry::HashMap<u64, u64> = Default::default();
//     b.iter(|| {
//         for i in 0..TO {
//             map.insert(i, i);
//         }
//     });
// }

// #[bench]
// fn flurry_get(b: &mut test::Bencher) {
//     let mut map: flurry::HashMap<u64, u64> = Default::default();
//     for i in 0..TO {
//         map.insert(i, i);
//     }
//     b.iter(|| {
//         for i in 0..TO {
//             assert!(map.get(&i).is_some());
//         }
//     });
// }

#[bench]
fn metrohash_insert(b: &mut test::Bencher) {
    let mut map: metrohash::MetroHashMap<u64, u64> = Default::default();
    b.iter(|| {
        for i in 0..TO {
            map.insert(i, i);
        }
    });
}

#[bench]
fn metrohash_get(b: &mut test::Bencher) {
    let mut map: metrohash::MetroHashMap<u64, u64> = Default::default();
    for i in 0..TO {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..TO {
            assert!(map.get(&i).is_some());
        }
    });
}

// #[bench]
// fn blake_insert(b: &mut test::Bencher) {
//     let mut map: std::collections::HashMap<u64, u64, blake3::Hasher> = Default::default();
//     b.iter(|| {
//         for i in 0..TO {
//             map.insert(i, i);
//         }
//     });
// }

// #[bench]
// fn blake_get(b: &mut test::Bencher) {
//     let mut map: std::collections::HashMap<u64, u64, blake3::Hasher> = Default::default();
//     for i in 0..TO {
//         map.insert(i, i);
//     }
//     b.iter(|| {
//         for i in 0..TO {
//             assert!(map.get(&i).is_some());
//         }
//     });
// }

#[bench]
fn hppc_insert(b: &mut test::Bencher) {
    let mut map: collection_performance::Map<usize> = collection_performance::Map::new(4);
    b.iter(|| {
        for i in 0..TO as usize {
            map.insert(i, i);
        }
    });
}

#[bench]
fn hppc_get(b: &mut test::Bencher) {
    let mut map: collection_performance::Map<usize> = collection_performance::Map::new(4);
    for i in 0..TO as usize {
        map.insert(i, i);
    }
    b.iter(|| {
        for i in 0..TO as usize {
            assert!(map.get(&i).is_some());
        }
    });
}
