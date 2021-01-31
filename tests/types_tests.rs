extern crate graphific;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod vertex_tests {
    use graphific::Vertex;

    #[test]
    fn new_vertex_test() {
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
    fn with_value_test() {
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
    fn set_value_test() {
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
}

#[cfg(test)]
mod edges_tests {
    use graphific::{Edge, Vertex};

    #[test]
    fn new_edge_test() {
        let v1: Vertex<i32, i32> = Vertex::with_value(0, 10);
        let v2: Vertex<i32, i32> = Vertex::with_value(1, 11);
        let v3: Vertex<i32, f32> = Vertex::with_value(2, 12.0);

        let e1: Edge<&i32> = Edge::new(v1.key(), v2.key());
        let e2: Edge<&i32> = Edge::new(v2.key(), v1.key());
        let e3: Edge<i32> = Edge::new(*v1.key(), *v3.key());
        let e4: Edge<i32> = Edge::new(*v3.key(), *v2.key());

        // test with borrow
        assert_eq!(0, **e1.from());
        assert_eq!(v1.key(), *e1.from());
        assert_eq!(1, **e1.to());
        assert_eq!(v2.key(), *e1.to());
        assert_eq!(1, **e2.from());
        assert_eq!(v2.key(), *e2.from());
        assert_eq!(0, **e2.to());
        assert_eq!(v1.key(), *e2.to());

        // test with direct value
        assert_eq!(0, *e3.from());
        assert_eq!(v1.key(), e3.from());
        assert_eq!(2, *e3.to());
        assert_eq!(v3.key(), e3.to());
        assert_eq!(2, *e4.from());
        assert_eq!(v3.key(), e4.from());
        assert_eq!(1, *e4.to());
        assert_eq!(v2.key(), e4.to());
    }
}
