#[allow(unused_imports)]
use crate::OwnedVector;

/// A constant version of the ?-operator.
///
/// This macro will take a [`Result`]`<T, E>` and return the [`Err`] prematurely.
/// This is mostly equivalent to the ?-operation except for the [`From::from`] conversion.
#[macro_export]
macro_rules! question_mark {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => return Err(err),
        }
    };
}

/// Creates an [`OwnedVector`] containing the arguments.
///
/// `vector![start_index; elements...]`
///
/// Similar to the [`vec!`] macro.
///
/// # Example
///
/// ```rust
/// # use vector::{vector, Vector};
/// let vec = vector![5; 5, 6, 7];
/// assert_eq!(vec.start(), 5);
/// assert_eq!(vec.end(), 7);
/// assert_eq!(vec[5], 5);
/// assert_eq!(vec[6], 6);
/// assert_eq!(vec[7], 7);
/// ```
#[macro_export]
macro_rules! vector {
    ($start:expr; $($element:expr),* $(,)?) => {{
        let vec = vec![$($element),*]; // Vec

        let len: usize = vec.len();
        let end: usize = if len > 0 { $start + len - 1 } else { $start };

        $crate::OwnedVector::from_vec(vec, $start, end).unwrap() // Safe
    }};
}
