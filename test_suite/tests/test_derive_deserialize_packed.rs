use serde::de::value::{Error, MapDeserializer, SeqDeserializer};
use serde::de::Deserialize;
use serde_derive::Deserialize;

#[test]
fn test_packed_struct_deserialize_in_place() {
    #[derive(Deserialize)]
    #[repr(C, packed)]
    struct Packed {
        tag: u8,
        value: u64,
    }

    let deserialized = Packed::deserialize(MapDeserializer::<_, Error>::new(
        [("tag", 42u64), ("value", 1234567890u64)].into_iter(),
    ))
    .unwrap();
    let tag = deserialized.tag;
    let value = deserialized.value;
    assert_eq!(tag, 42);
    assert_eq!(value, 1234567890);

    let mut in_place = Packed { tag: 0, value: 0 };
    Packed::deserialize_in_place(
        MapDeserializer::<_, Error>::new([("tag", 42u64), ("value", 1234567890u64)].into_iter()),
        &mut in_place,
    )
    .unwrap();
    let tag = in_place.tag;
    let value = in_place.value;
    assert_eq!(tag, 42);
    assert_eq!(value, 1234567890);
}

#[test]
fn test_packed_tuple_struct() {
    #[derive(Deserialize)]
    #[repr(packed)]
    struct PackedTuple(u8, u64, u16);

    let deserialized =
        PackedTuple::deserialize(SeqDeserializer::<_, Error>::new([1u64, 999, 42].into_iter()))
            .unwrap();
    let a = deserialized.0;
    let b = deserialized.1;
    let c = deserialized.2;
    assert_eq!(a, 1);
    assert_eq!(b, 999);
    assert_eq!(c, 42);

    let mut in_place = PackedTuple(0, 0, 0);
    PackedTuple::deserialize_in_place(
        SeqDeserializer::<_, Error>::new([1u64, 999, 42].into_iter()),
        &mut in_place,
    )
    .unwrap();
    let a = in_place.0;
    let b = in_place.1;
    let c = in_place.2;
    assert_eq!(a, 1);
    assert_eq!(b, 999);
    assert_eq!(c, 42);
}
