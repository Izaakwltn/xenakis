mod points;
mod rays;
mod shapes;
mod vectors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
/*#[test]
    fn point_test() {
        let test_point = default_point();
        assert_eq!(test_point.x, 0.0);
        assert_eq!(test_point.y, 1.0);
        assert_eq!(test_point.z, 0.0);

        let test_copy = test_point.copy();
        assert_eq!(test_copy.x, 0.0);
        assert_eq!(test_copy.y, 1.0);
        assert_eq!(test_copy.z, 0.0);

        let test_v = test_copy.point_to_vector();
        assert_eq!(test_v.x, 0.0);
        assert_eq!(test_v.y, 1.0);
        assert_eq!(test_v.z, 0.0);
    }
}
*/
