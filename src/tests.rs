use std::{
    collections::{
        hash_map::{DefaultHasher, RandomState},
        HashMap,
    },
    hash::BuildHasher,
    cell::Cell,
};

use crate::{Borrowed, NoneStorer};

use super::How;

#[test]
fn test_eq() {
    let mut a = How::new_default("hi");
    let b = How::new_default("hi");
    assert_eq!(a, b);
    assert!(! How::is_hashed(&a));
    assert!(! How::is_hashed(&b));

    How::make_hash(&a);
    assert!(How::is_hashed(&a));
    assert_eq!(a, b);

    How::make_mut(&mut a);
    assert!(! How::is_hashed(&a));
    assert_eq!(a, b);
}

#[test]
fn test_ne() {
    let mut a = How::new_default("hi");
    let b = How::new_default("hello");

    assert_ne!(a, b);

    How::make_hash(&a);
    assert_ne!(a, b);

    How::make_mut(&mut a);
    assert_ne!(a, b);
}

#[test]
fn test_hash() {
    let a = How::new_default("hi");
    let b = How::new_default("hi");
    let state = RandomState::new();

    let a_code = state.hash_one(&a);

    assert_eq!(a_code, state.hash_one(&b));
    assert!(How::is_hashed(&a));
    assert!(How::is_hashed(&b));

    assert_eq!(a_code, state.hash_one(&b));
    assert!(How::is_hashed(&a));
    assert!(How::is_hashed(&b));
}

#[test]
fn test_hash_map() {
    #[allow(clippy::mutable_key_type)]
    let mut map: HashMap<How<&'static str>, i32> = HashMap::new();

    let a: How<&str> = "a".into();

    assert!(map.insert(a.clone(), 1).is_none());
    assert!(map.insert("b".into(), 2).is_none());
    assert!(map.insert("c".into(), 3).is_none());

    assert_eq!(map.insert(a.clone(), -1), Some(1));
    assert_eq!(map.insert("b".into(), -2), Some(2));
    assert_eq!(map.insert("c".into(), -3), Some(3));

    assert_eq!(map.get(&a), Some(&-1));
    assert_eq!(map.get(&Borrowed::new("b")), Some(&-2));
    assert_eq!(map.get(&Borrowed::new("c")), Some(&-3));

    let x: HashMap<How<String>, ()> = HashMap::new();
    assert!(x.get(Borrowed::make_ref("a")).is_none());
}

#[test]
fn test_none_store() {
    type IHow<T> = How<T, DefaultHasher, Cell<u64>>;
    type NHow<T> = How<T, DefaultHasher, NoneStorer>;
    type IBorrowed<T> = Borrowed<T, DefaultHasher, Cell<u64>>;
    type NBorrowed<T> = Borrowed<T, DefaultHasher, NoneStorer>;

    let datas = [
        "foo",
        "",
        "test",
        "bar",
        "a",
        "egg",
        "examples",
        "BIG",
    ];

    for _ in 0..5000 {
        let bh = RandomState::new();

        for data in datas {
            let a = IHow::new(data);
            let b = NHow::new(data);

            assert_eq!(bh.hash_one(&a), bh.hash_one(&b));
            assert_eq!(bh.hash_one(&a), bh.hash_one(NBorrowed::new(data)));
            assert_eq!(bh.hash_one(&a), bh.hash_one(NBorrowed::new(data)));
            assert_eq!(bh.hash_one(&a), bh.hash_one(IBorrowed::new(data)));
            assert_eq!(bh.hash_one(&a), bh.hash_one(IBorrowed::new(data)));
            assert_eq!(bh.hash_one(&a), bh.hash_one(NBorrowed::make_ref(data)));
            assert_eq!(bh.hash_one(&a), bh.hash_one(NBorrowed::make_ref(data)));
            assert_eq!(bh.hash_one(&a), bh.hash_one(IBorrowed::make_ref(data)));
            assert_eq!(bh.hash_one(&a), bh.hash_one(IBorrowed::make_ref(data)));
        }
    }
}
