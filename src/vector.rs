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
    #[must_use]
    fn start(&self) -> usize;

    #[must_use]
    fn end(&self) -> usize;

    fn as_slice(&self) -> &[V];

    fn iter(&self) -> Iter<'_, V> {
        self.as_slice().iter()
    }

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
