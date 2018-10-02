use std;
use std::collections::HashMap;

use rmp_serde;
use ron;
use serde_json;
use serde_pickle;
use serde_yaml;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: u32,
    z: [i32; 3],
    abc: f64,
    s: String,
    nu1: Option<bool>,
    nu2: Option<bool>,
    bo: bool,
    map: HashMap<String, i32>,
}

fn test_json(p: &Point) {
    println!("json");
    let s = serde_json::to_string(p);
    if s.is_err() {
        println!("serialized\n{}", s.err().unwrap());
        return;
    }
    let s = s.unwrap();
    println!("serialized\n{}", s);
    let d = serde_json::from_str::<Point>(&s);
    if d.is_ok() {
        println!("deserialized\n{:?}", d.unwrap());
    } else {
        println!("deserialized\n{}", d.err().unwrap());
    }
}

fn test_yaml(p: &Point) {
    println!("yaml");
    let s = serde_yaml::to_string(p);
    if s.is_err() {
        println!("serialized\n{}", s.err().unwrap());
        return;
    }
    let s = s.unwrap();
    println!("serialized\n{}", s);
    let d = serde_yaml::from_str::<Point>(&s);
    if d.is_ok() {
        println!("deserialized\n{:?}", d.unwrap());
    } else {
        println!("deserialized\n{}", d.err().unwrap());
    }
}

fn test_toml(p: &Point) {
    println!("toml");
    let s = toml::to_string(p);
    if s.is_err() {
        println!("serialized\n{}", s.err().unwrap());
        return;
    }
    let s = s.unwrap();
    println!("serialized\n{}", s);
    let d = toml::from_str::<Point>(&s);
    if d.is_ok() {
        println!("deserialized\n{:?}", d.unwrap());
    } else {
        println!("deserialized\n{}", d.err().unwrap());
    }
}

fn test_pickle(p: &Point) {
    println!("pickle");
    let s = serde_pickle::to_vec(p, true);
    if s.is_err() {
        println!("serialized\n{}", s.err().unwrap());
        return;
    }
    let s = s.unwrap();
    println!("serialized\n{:?}", s);
    let d = serde_pickle::from_slice::<Point>(&s);
    if d.is_ok() {
        println!("deserialized\n{:?}", d.unwrap());
    } else {
        println!("deserialized\n{}", d.err().unwrap());
    }
}

fn test_ron(p: &Point) {
    println!("ron");
    let s = ron::ser::to_string(p);
    if s.is_err() {
        println!("serialized\n{}", s.err().unwrap());
        return;
    }
    let s = s.unwrap();
    println!("serialized\n{}", s);
    let d = ron::de::from_str::<Point>(&s);
    if d.is_ok() {
        println!("deserialized\n{:?}", d.unwrap());
    } else {
        println!("deserialized\n{}", d.err().unwrap());
    }
}

fn test_rmp(p: &Point) {
    println!("rmp");
    let s = rmp_serde::to_vec(p);
    if s.is_err() {
        println!("serialized\n{}", s.err().unwrap());
        return;
    }
    let s = s.unwrap();
    println!("serialized\n{:?}", s);
    let d = rmp_serde::from_slice::<Point>(&s);
    if d.is_ok() {
        println!("deserialized\n{:?}", d.unwrap());
    } else {
        println!("deserialized\n{}", d.err().unwrap());
    }
}

pub fn run() {
    let mut point = Point {
        x: 1,
        y: std::u32::MAX,
        z: [0, 1, 2],
        abc: 0.12345678901234,
        s: r#"abcあいう🍣🍺
        "エスケープ"''"""#
            .to_string(),
        nu1: None,
        nu2: Some(false),
        bo: false,
        map: HashMap::new(),
    };
    point.map.insert("key1".to_string(), 0);
    point.map.insert("key2".to_string(), -1);
    test_json(&point);
    test_yaml(&point);
    test_toml(&point);
    test_pickle(&point);
    test_ron(&point);
    test_rmp(&point);
}
