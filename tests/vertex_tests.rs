extern crate graphific;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod vertex_tests {
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
}
