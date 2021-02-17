pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[cfg(test)]
mod tests {
    use crate::type_of;

    #[test]
    fn it_works() {
        assert_eq!("i32", type_of(10));
        assert_eq!("i32", type_of(12));
        assert_eq!("i32", type_of(15));
        assert_eq!("i32", type_of(13));
        assert_eq!("i32", type_of(333));
        assert_eq!("f64", type_of(15.2));
        assert_eq!("f64", type_of(12.5));
        assert_eq!("f64", type_of(3.5));
        assert_eq!("f64", type_of(3.6));
        assert_eq!("f64", type_of(7.5));
        assert_eq!("&str", type_of("Draw"));
        assert_eq!("&str", type_of("cross"));
        assert_eq!("&str", type_of("pattern"));
        assert_eq!("&str", type_of("hot"));
    }
}
