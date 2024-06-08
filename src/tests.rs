use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::BuildHasher,
};

use super::How;

#[test]
fn test_eq() {
    let mut a = How::new_default("hi");
    let b = How::new_default("hi");
    assert_eq!(a, b);
    assert!(! How::is_hashed(&a));
    assert!(! How::is_hashed(&b));

    How::make_hash(&mut a);
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

    How::make_hash(&mut a);
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
    let mut map: HashMap<How<&'static str>, i32> = HashMap::new();

    let a: How<&str> = "a".into();

    assert!(map.insert(a.clone(), 1).is_none());
    assert!(map.insert("b".into(), 2).is_none());
    assert!(map.insert("c".into(), 3).is_none());

    assert_eq!(map.insert(a.clone(), -1), Some(1));
    assert_eq!(map.insert("b".into(), -2), Some(2));
    assert_eq!(map.insert("c".into(), -3), Some(3));

    assert_eq!(map.get(&a), Some(&-1));
    assert_eq!(map.get(&"b".into()), Some(&-2));
    assert_eq!(map.get(&"c".into()), Some(&-3));
}
