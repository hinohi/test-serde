use std::collections::HashMap;

use ron;
use serde_json;
use rmp_serde;
use test;

fn ser_non_string_key_map() {
    let mut map = HashMap::new();
    map.insert([0, 1, 2], 4);
    map.insert([3, 4, 5], 6);

    let s = serde_json::to_string(&map);
    println!("{:?}", s);

    let s = rmp_serde::to_vec(&map);
    println!("{:?}", s);

    let s = ron::ser::to_string(&map);
    println!("{:?}", s);
}

fn size_test() {
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert([i, i + 1, i + 2, i+ 3 + i + 4 + i + 5], i);
    }
    let s = rmp_serde::to_vec(&map).unwrap();
    println!("rmp={}", s.len());
    let s = ron::ser::to_string(&map).unwrap();
    println!("ron={}", s.len());
}

#[bench]
fn bench_rmp(b: &mut test::Bencher) {
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert([i, i + 1, i + 2, i+ 3, i + 4, i + 5], i);
    }
    b.iter(move || rmp_serde::to_vec(&map).unwrap());
}

#[bench]
fn bench_ron(b: &mut test::Bencher) {
    let mut map = HashMap::new();
    for i in 0..1000 {
        map.insert([i, i + 1, i + 2, i+ 3, i + 4, i + 5], i);
    }
    b.iter(move || ron::ser::to_string(&map).unwrap());
}

pub fn run() {
    ser_non_string_key_map();
    size_test();
}