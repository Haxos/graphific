extern crate graphific;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use graphific::Vertex;

#[test]
fn test_new_vertex() {
    let v1: Vertex<i32, i32> = Vertex::new(0);
    let v2: Vertex<i32, i32> = Vertex::new(1);
    let v3: Vertex<i32, f32> = Vertex::new(2);

    assert_eq!(0, *v1.key());
    assert_eq!(0, *v1.value());
    assert_eq!(i32::default(), *v1.value());
    assert_eq!(1, *v2.key());
    assert_eq!(0, *v2.value());
    assert_eq!(i32::default(), *v2.value());
    assert_eq!(2, *v3.key());
    assert_eq!(0.0, *v3.value());
    assert_eq!(f32::default(), *v3.value());
}

#[test]
fn test_fn_with_value() {
    let v1: Vertex<i32, i32> = Vertex::with_value(0, 10);
    let v2: Vertex<i32, i32> = Vertex::with_value(1, 11);
    let v3: Vertex<i32, f32> = Vertex::with_value(2, 12.0);

    assert_eq!(0, *v1.key());
    assert_eq!(10, *v1.value());
    assert_ne!(i32::default(), *v1.value());
    assert_eq!(1, *v2.key());
    assert_eq!(11, *v2.value());
    assert_ne!(i32::default(), *v2.value());
    assert_eq!(2, *v3.key());
    assert_eq!(12.0, *v3.value());
    assert_ne!(f32::default(), *v3.value());
}

#[test]
fn test_fn_set_value() {
    let mut v1: Vertex<i32, i32> = Vertex::new(0);
    let mut v2: Vertex<i32, f32> = Vertex::new(1);

    assert_eq!(0, *v1.key());
    assert_eq!(0, *v1.value());
    assert_eq!(i32::default(), *v1.value());
    assert_eq!(1, *v2.key());
    assert_eq!(0.0, *v2.value());
    assert_eq!(f32::default(), *v2.value());

    v1.set_value(10);
    v2.set_value(11.0);

    assert_eq!(10, *v1.value());
    assert_ne!(i32::default(), *v1.value());
    assert_eq!(11.0, *v2.value());
    assert_ne!(f32::default(), *v2.value());
}
