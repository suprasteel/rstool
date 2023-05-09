use std::any::type_name;

pub fn kind<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_number_is_i32() {
        assert_eq!("i32", kind(10));
    }
}
