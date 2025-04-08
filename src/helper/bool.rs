/*
 * Copyright (c) Portalnesia - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Putu Aditya <aditya@portalnesia.com>
 */

/// Trait untuk mengecek “truthiness” sesuai rules:
/// - integer: truthy jika == 1
/// - float:   truthy jika == 1.0
/// - bool:    truthy jika == true
/// - String/&str: truthy jika == "true"
pub trait Truthy {
    fn is_truthy(&self) -> bool;
}

impl Truthy for i32 {
    fn is_truthy(&self) -> bool {
        *self == 1
    }
}
impl Truthy for i64 {
    fn is_truthy(&self) -> bool {
        *self == 1
    }
}
impl Truthy for u32 {
    fn is_truthy(&self) -> bool {
        *self == 1
    }
}
impl Truthy for u64 {
    fn is_truthy(&self) -> bool {
        *self == 1
    }
}

impl Truthy for f32 {
    fn is_truthy(&self) -> bool {
        (*self - 1.0).abs() < f32::EPSILON
    }
}
impl Truthy for f64 {
    fn is_truthy(&self) -> bool {
        (*self - 1.0).abs() < f64::EPSILON
    }
}

impl Truthy for bool {
    fn is_truthy(&self) -> bool {
        *self
    }
}

impl Truthy for String {
    fn is_truthy(&self) -> bool {
        self == "true"
    }
}
impl Truthy for &str {
    fn is_truthy(&self) -> bool {
        *self == "true"
    }
}

/// Macro yang memanggil `.is_truthy()`
///
/// # Example
/// ```
/// assert!(utils::is_true!(1_i32));
/// assert!(!utils::is_true!(2_i32));
/// assert!(utils::is_true!(1.0_f64));
/// assert!(!utils::is_true!(0.5_f64));
/// assert!(utils::is_true!(String::from("true")));
/// assert!(!utils::is_true!(String::from("false")));
/// assert!(utils::is_true!("true"));
/// assert!(!utils::is_true!("false"));
/// assert!(utils::is_true!(true));
/// assert!(!utils::is_true!(false));
/// ```
#[macro_export]
macro_rules! is_true {
    ($x:expr) => {{
        // helper generic, memanggil method dari trait yang fully‐qualified
        fn __is_true_helper<T: $crate::helper::Truthy>(val: T) -> bool {
            val.is_truthy()
        }
        __is_true_helper($x)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_truthy_integers() {
        assert!(is_true!(1_i32));
        assert!(!is_true!(0_i32));
        assert!(!is_true!(2_i32));

        assert!(is_true!(1_i64));
        assert!(!is_true!(0_i64));
    }

    #[test]
    fn test_is_truthy_floats() {
        assert!(is_true!(1.0_f32));
        assert!(!is_true!(0.999_f32));

        assert!(is_true!(1.0_f64));
        assert!(!is_true!(1.0001_f64));
    }

    #[test]
    fn test_is_truthy_bool() {
        assert!(is_true!(true));
        assert!(!is_true!(false));
    }

    #[test]
    fn test_is_truthy_string_and_str() {
        assert!(is_true!(String::from("true")));
        assert!(!is_true!(String::from("false")));

        assert!(is_true!("true"));
        assert!(!is_true!("false"));
    }
}
