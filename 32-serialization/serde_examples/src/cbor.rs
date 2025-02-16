use serde::{Deserialize, Serialize};
use serde_cbor::Value;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Mascot {
    name: String,
    species: String,
    year_of_birth: u32,
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let ferris = Mascot {
        name: "Ferris".to_owned(),
        species: "crab".to_owned(),
        year_of_birth: 2015,
    };

    let ferris_file = File::create("ferris.cbor").unwrap();
    serde_cbor::to_writer(ferris_file, &ferris).unwrap();

    let tux_file = File::open("ferris.cbor").unwrap();
    let tux: Mascot = serde_cbor::from_reader(tux_file).unwrap();
    println!("Deserialized mascot: {:?}", tux);

    let bytes1 = serde_cbor::to_vec(&ferris).unwrap();
    let value1: Value = serde_cbor::from_slice(&bytes1).unwrap();
    println!("Deserialized value: {:?}", value1);

    let bytes2 = serde_cbor::to_vec(&value1).unwrap();
    let value2: Value = serde_cbor::from_slice(&bytes2).unwrap();
    assert_eq!(value1, value2);
    println!("Deserialized value: {:?}", value2);

    let result: Mascot = serde_cbor::from_slice(&bytes2).unwrap();
    assert_eq!(ferris, result);
    println!("Deserialized mascot: {:?}", result);
}
