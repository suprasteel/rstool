use std::any::type_name;

/// Returns the name of the given type
///
/// # Usage example
///
/// ```
/// use rstool::kind;
/// print!("{}", kind(4));
/// // "i32"
/// ```
///
/// # Argument
///
/// - _ the value for which we want to display the type
///
/// # Returns
///
/// - &' static str
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
