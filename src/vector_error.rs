use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[allow(unused_imports)]
use crate::vector::Vector;

/// An enum for handling the errors involved in the creation and access of [`Vector`] instances.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VectorError {
    /// Start and end are not ordered correctly.
    ///
    /// `start` > `end`.
    Order { start: usize, end: usize },

    /// The length of the given vector does not equal the expect value.
    ///
    /// `len` != `end` - `start` + 1.
    ///
    /// Note that the `end` index is included.
    Length {
        len: usize,
        start: usize,
        end: usize,
    },

    /// There is no element at this index.
    ///
    /// This error will be thrown if there is no element at the true index.
    /// When using the offset indexing this will mean, that there is no element at `index` - `start`.
    Indexing { index: usize },

    /// The `start` and `end` arguments are not compatible.
    ///
    /// Either `start_1` != `start_2` or `end_1` != `end_2`.
    Compatibility {
        start_1: usize,
        start_2: usize,
        end_1: usize,
        end_2: usize,
    },
}

impl VectorError {
    /// Helper to validate the order of the `start` and `end` parameters.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Order`] - `start` > `end`.
    #[inline]
    pub(crate) const fn check_order(start: usize, end: usize) -> Result<(), VectorError> {
        if start <= end {
            Ok(())
        } else {
            Err(Self::Order { start, end })
        }
    }

    /// Helper to validate the length of a vector with the `start` and `end` arguments.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Length`] - `len` != `end` - `start` + 1.
    #[inline]
    pub(crate) const fn check_len(len: usize, start: usize, end: usize) -> Result<(), VectorError> {
        // Equivalent to len == end - start + 1
        // Protects from underflow
        if len + start == end + 1 {
            Ok(())
        } else {
            Err(VectorError::Length { len, start, end })
        }
    }
}

impl Display for VectorError {
    fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Order { start, end } => write!(
                format,
                "The start ({}) is bigger than the end ({})",
                start, end
            ),
            Self::Length { len, start, end } => write!(
                format,
                "The length of the vector ({}) does not match with the distance between start ({}) and end ({})",
                len, start, end
            ),
            Self::Indexing { index } => write!(format, "No element at position {} exists", index),
            Self::Compatibility {
                start_1,
                start_2,
                end_1,
                end_2,
            } => write!(
                format,
                "Either the starts ({} vs. {}) do not match or the ends ({} vs {})",
                start_1, start_2, end_1, end_2
            ),
        }
    }
}

impl Error for VectorError {}
