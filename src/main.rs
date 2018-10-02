#![feature(test)]

#[macro_use]
extern crate serde_derive;

extern crate rmp_serde;
extern crate ron;
extern crate serde;
extern crate serde_json;
extern crate serde_pickle;
extern crate serde_yaml;
extern crate toml;
extern crate test;

mod val;
mod pf;

fn main() {
    pf::run();
}
