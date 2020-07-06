#![feature(test)]
extern crate test;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
// cargo +nightly bench --bench deserialize_string_enum_u8_bool

#[derive(Serialize, Deserialize)]
struct BoolFieldForm {
    data: bool,
}

#[derive(Serialize, Deserialize)]
struct U8FieldForm {
    data: u8,
}

#[derive(Serialize, Deserialize)]
struct StringFieldForm {
    data: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
enum EnumU8 {
    Data,
}

#[derive(Serialize, Deserialize)]
struct EnumFieldForm {
    data: EnumU8,
}

/*
test deserialize_bool   ... bench:  144 ns/iter (+/- 6)
test deserialize_enum   ... bench:  259 ns/iter (+/- 60)
test deserialize_string ... bench:  386 ns/iter (+/- 139)
test deserialize_u8     ... bench:  138 ns/iter (+/- 37)
*/

#[derive(Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
enum State {
    Waiting = 100,
    Finished = 20,
}

#[bench]
fn deserialize_enum(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let var = EnumFieldForm { data: EnumU8::Data };
        let serialized = serde_json::to_string(&var).unwrap();
        let form: EnumFieldForm = serde_json::from_str(&serialized).unwrap();
        assert_eq!(serde_json::to_string(&form.data).unwrap(), "0");
    });
}

#[bench]
fn deserialize_bool(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let var = BoolFieldForm { data: false };
        let serialized = serde_json::to_string(&var).unwrap();
        let form: BoolFieldForm = serde_json::from_str(&serialized).unwrap();
        assert!(form.data.eq(&false))
    });
}

#[bench]
fn deserialize_string(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let var = StringFieldForm {
            data: "0".to_string(),
        };
        let serialized = serde_json::to_string(&var).unwrap();
        let form: StringFieldForm = serde_json::from_str(&serialized).unwrap();
        assert!(form.data.eq("0"))
    });
}
