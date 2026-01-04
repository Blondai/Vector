use std::{
    ops::{Index, Range, RangeInclusive},
    slice::Iter,
};

use crate::{VectorError, Vectorable};

#[allow(unused_imports)]
use crate::{BorrowedVector, OwnedVector};

/// A trait to combine the usage [`OwnedVector`] and [`BorrowedVector`].
pub trait Vector<V: Vectorable>:
    Index<usize, Output = V>
    + Index<RangeInclusive<usize>, Output = [V]>
    + Index<Range<usize>, Output = [V]>
{
    /// Returns the `start` index of the [`Vector`].
    ///
    /// This is the first index where an element is located.
    #[must_use]
    fn start(&self) -> usize;

    /// Returns the `end` index of the [`Vector`].
    ///
    /// This is the last index where an element is located.
    #[must_use]
    fn end(&self) -> usize;

    /// Extracts a slice containing the entire vector.
    #[must_use]
    fn as_slice(&self) -> &[V];

    /// Slices into a [`Vector`] and returns a corresponding [`BorrowedVector`].
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - If `start` or `end` are out of bounds of the current vector.
    /// `start` < `self.start` or `end` > `self.end`.
    /// * [`VectorError::Order`] - The order of the arguments is wrong.
    /// `start` > `end`.
    fn slice(&'_ self, start: usize, end: usize) -> Result<BorrowedVector<'_, V>, VectorError>;

    /// Returns an iterator over the vector.
    ///
    /// The iterator yields all items from `start` to `end`.
    fn iter(&self) -> Iter<'_, V> {
        self.as_slice().iter()
    }

    /// Returns the length of the underlying vector.
    ///
    /// Per construction this is equal to `end` - `start` + 1.
    #[must_use]
    fn len(&self) -> usize;

    /// Returns the value at the `index`th position using the offset indexing system.
    ///
    /// This automatically uses the offest.
    /// In the underlying [`Vec`] this is the element at position `index` - `start`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The underlying vector does not have enough elements.
    /// `index` < `start` or `index` > `end`.
    fn get(&self, index: usize) -> Result<V, VectorError>;

    /// Returns the value at the `index`th position using the original indexing system.
    ///
    /// This ignores the offest.
    /// In the underlying [`Vec`] this is the element at position `index`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The underlying vector does not have enough elements.
    /// `vec.len()` - 1 < `index`.
    fn get_absolute(&self, index: usize) -> Result<V, VectorError>;

    /// Returns a slice inside the underlying vector based on the offset range from `start` to `end`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The `start` or `end` is outside the supported range.
    /// `start` < `self.start` or `end` > `self.end` + 1.
    fn get_range(&self, start: usize, end: usize) -> Result<&[V], VectorError>;

    /// Returns a slice inside the underlying vector based on the offset range from `start` to `end`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The `start` or `end` is outside the supported range.
    /// `start` < `self.start` or `end` > `self.end` + 1.
    fn get_range_inclusive(&self, start: usize, end: usize) -> Result<&[V], VectorError>;

    /// Checks whether two [`Vector`]s are compatible.
    ///
    /// This can be used before a [`Iter::zip`] to assert that no values are lost.
    /// It accepts any type implementing `Vector`, allowing comparison between Owned and Borrowed variants.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Compatibility`] - `start` or `end` indices do not match.
    #[inline]
    fn compatible(&self, other: &impl Vector<V>) -> Result<(), VectorError> {
        if self.start() == other.start() && self.end() == other.end() {
            Ok(())
        } else {
            Err(VectorError::Compatibility {
                start_1: self.start(),
                start_2: other.start(),
                end_1: self.end(),
                end_2: other.end(),
            })
        }
    }
}
