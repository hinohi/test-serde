# Test serde target

## env

```sh
$ rustc --version
rustc 1.31.0-nightly (de3d640f5 2018-10-01)
$ cargo --version
cargo 1.31.0-nightly (de314a8b2 2018-09-21)
```

+ macOS 10.13.6
+ Intel Core i7 3.3 GHz
+ 16GB

## データ型

## JSON

```text
json
serialized
{"x":1,"y":4294967295,"z":[0,1,2],"abc":0.12345678901234,"s":"abcあいう🍣🍺\n        \"エスケープ\"''\"\"","nu1":null,"nu2":false,"bo":false,"map":{"key2":-1,"key1":0}}
deserialized
Point { x: 1, y: 4294967295, z: [0, 1, 2], abc: 0.12345678901234, s: "abcあいう🍣🍺\n        \"エスケープ\"\'\'\"\"", nu1: None, nu2: Some(false), bo: false, map: {"key1": 0, "key2": -1} }
```

王道。 `u64` も問題なく扱えたので数値だから浮動小数とかいうわけがわからない実装ではなさそう。

## yaml

```text
yaml
serialized
---
x: 1
y: 4294967295
z:
  - 0
  - 1
  - 2
abc: 0.12345678901234
s: "abcあいう🍣🍺\n        \"エスケープ\"''\"\""
nu1: ~
nu2: false
bo: false
map:
  key2: -1
  key1: 0
deserialized
Point { x: 1, y: 4294967295, z: [0, 1, 2], abc: 0.12345678901234, s: "abcあいう🍣🍺\n        \"エスケープ\"\'\'\"\"", nu1: None, nu2: Some(false), bo: false, map: {"key1": 0, "key2": -1} }
```

`None` の表現が `~` なの初めて知った。

## toml

```text
toml
serialized
x = 1
y = 4294967295
z = [0, 1, 2]
abc = 0.12345678901234
s = "abcあいう🍣🍺\n        \"エスケープ\"''\"\""
nu2 = false
bo = false

[map]
key2 = -1
key1 = 0

deserialized
Point { x: 1, y: 4294967295, z: [0, 1, 2], abc: 0.12345678901234, s: "abcあいう🍣🍺\n        \"エスケープ\"\'\'\"\"", nu1: None, nu2: Some(false), bo: false, map: {"key2": -1, "key1": 0} }
```

## pickle

```text
pickle
serialized
[128, 3, 125, 40, 88, 1, 0, 0, 0, 120, 74, 1, 0, 0, 0, 88, 1, 0, 0, 0, 121, 138, 5, 255, 255, 255, 255, 0, 88, 1, 0, 0, 0, 122, 40, 74, 0, 0, 0, 0, 74, 1, 0, 0, 0, 74, 2, 0, 0, 0, 116, 88, 3, 0, 0, 0, 97, 98, 99, 71, 63, 191, 154, 221, 55, 70, 244, 198, 88, 1, 0, 0, 0, 115, 88, 50, 0, 0, 0, 97, 98, 99, 227, 129, 130, 227, 129, 132, 227, 129, 134, 240, 159, 141, 163, 240, 159, 141, 186, 10, 32, 32, 32, 32, 32, 32, 32, 32, 34, 227, 130, 168, 227, 130, 185, 227, 130, 177, 227, 131, 188, 227, 131, 151, 34, 39, 39, 34, 34, 88, 3, 0, 0, 0, 110, 117, 49, 78, 88, 3, 0, 0, 0, 110, 117, 50, 137, 88, 2, 0, 0, 0, 98, 111, 137, 88, 3, 0, 0, 0, 109, 97, 112, 125, 40, 88, 4, 0,
0, 0, 107, 101, 121, 50, 74, 255, 255, 255, 255, 88, 4, 0, 0, 0, 107, 101, 121, 49, 74, 0, 0, 0, 0, 117, 117, 46]
deserialized
Point { x: 1, y: 4294967295, z: [0, 1, 2], abc: 0.12345678901234, s: "abcあいう🍣🍺\n        \"エスケープ\"\'\'\"\"", nu1: None, nu2: Some(false), bo: false, map: {"key2": -1, "key1": 0} }
```

あることにビビる。
`u64` をシリアライズはできるがデシリアライズできないという謎仕様。

## ron

```text
ron
serialized
(x:1,y:4294967295,z:(0,1,2,),abc:0.12345678901234,s:"abcあいう🍣🍺\n        \"エスケープ\"\'\'\"\"",nu1:None,nu2:Some(false),bo:false,map:{"key2":-1,"key1":0,},)
Point { x: 1, y: 4294967295, z: [0, 1, 2], abc: 0.12345678901234, s: "abcあいう🍣🍺\n        \"エスケープ\"\'\'\"\"", nu1: None, nu2: Some(false), bo: false, map: {"key2": -1, "key1": 0} }
```

Rust like syntax の独自仕様フォーマット。
面白いと思う。

## msgpack

```text
rmp
serialized
[153, 1, 206, 255, 255, 255, 255, 147, 0, 1, 2, 203, 63, 191, 154, 221, 55, 70, 244, 198, 217, 50, 97, 98, 99, 227, 129, 130, 227, 129, 132, 227, 129, 134, 240, 159, 141, 163, 240, 159, 141, 186, 10, 32, 32, 32, 32, 32, 32,
32, 32, 34, 227, 130, 168, 227, 130, 185, 227, 130, 177, 227, 131, 188, 227, 131, 151, 34, 39, 39, 34, 34, 192, 194, 194, 130, 164, 107, 101, 121, 50, 255, 164, 107, 101, 121, 49, 0]
deserialized
Point { x: 1, y: 4294967295, z: [0, 1, 2], abc: 0.12345678901234, s: "abcあいう🍣🍺\n        \"エスケープ\"\'\'\"\"", nu1: None, nu2: Some(false), bo: false, map: {"key1": 0, "key2": -1} }  
```

実装してる人が強そう？

## サイズ

ron と msgpack に限定して `HashMap<[i32; 3], i32>` のシリアライズで計測。
1000要素。

```text
rmp=14352
ron=23211
```

msgpack 強い。

## 速度

最終的に Desk IO になるんだからサイズが小さいのが大正義だけど一応。
`HashMap<[i32; 6], i32>` のシリアライズで計測。
1000要素。

```text
    Finished release [optimized] target(s) in 0.87s
     Running target/release/deps/serde_test-744f648a72a312ed

running 2 tests
test pf::bench_rmp ... bench:     147,128 ns/iter (+/- 44,854)
test pf::bench_ron ... bench:     517,421 ns/iter (+/- 30,142)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
```

う〜ん、なるほど。