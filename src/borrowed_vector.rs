use std::{
    ops::{Index, Range, RangeInclusive},
    slice::Iter,
};

use crate::{Vector, VectorError, Vectorable, question_mark};

/// A wrapper struct around a generic slice of a [`Vec`] allowing the automatic calculation of indexing offsets.
///
/// The generic value needs to implement the [`Vectorable`] trait.
#[derive(Debug, Copy, Clone)]
pub struct BorrowedVector<'a, V: Vectorable> {
    /// The slice containing the values.
    slice: &'a [V],

    /// The start to allow the correct index offsetting.
    start: usize,

    /// The end to allow to assertion of the correct length.
    ///
    /// Note that the end is included.
    end: usize,
}

impl<'a, V: Vectorable> BorrowedVector<'a, V> {
    /// Creates a new [`BorrowedVector`] based on a given slice, `start` and `end` arguments.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Order`] - The order of the arguments is wrong.
    /// `start` > `end`.
    /// * [`VectorError::Length`] - The expected length does not match the provided one.
    /// `vec.len() != end - start + 1`.
    pub const fn try_new(slice: &'a [V], start: usize, end: usize) -> Result<Self, VectorError> {
        question_mark!(VectorError::check_order(start, end));
        question_mark!(VectorError::check_len(slice.len(), start, end));

        Ok(Self { slice, start, end })
    }

    /// Creates a new [`BorrowedVector`] based on a given slice, `start` and `end` arguments.
    ///
    /// # Panics
    ///
    /// * The order of the arguments is wrong.
    /// `start` > `end`.
    /// *  The expected length does not match the provided one.
    /// `vec.len() != end - start + 1`.
    pub const fn new(slice: &'a [V], start: usize, end: usize) -> Self {
        assert!(VectorError::check_order(start, end).is_ok());
        assert!(VectorError::check_len(slice.len(), start, end).is_ok());

        Self { slice, start, end }
    }

    /// Returns the value at the `index`th position using the offset indexing system.
    ///
    /// This automatically uses the offest.
    /// In the underlying [`Vec`] this is the element at position `index - start`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The underlying vector does not have enough elements.
    /// `index` < `start` or `index` > `end`.
    #[inline]
    pub fn get(&self, index: usize) -> Result<V, VectorError> {
        self.slice
            // Underflow will wrap around and return a `None` variant
            .get(index.wrapping_sub(self.start))
            .copied()
            .ok_or(VectorError::Indexing { index })
    }

    /// Returns the value at the `index`th position using the original indexing system.
    ///
    /// This ignores the offest.
    /// In the underlying [`Vec`] this is the element at position `index`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The underlying vector does not have enough elements.
    /// `vec.len() - 1 < index`.
    #[inline]
    pub fn get_absolute(&self, index: usize) -> Result<V, VectorError> {
        self.slice
            .get(index)
            .copied()
            .ok_or(VectorError::Indexing { index })
    }

    /// Slices into a [`BorrowedVector`] and returns another [`BorrowedVector`].
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - If `start` or `end` are out of bounds of the current vector.
    /// `start` < `self.start` or `end` > `self.end`.
    /// * [`VectorError::Order`] - The order of the arguments is wrong.
    /// `start` > `end`.
    pub fn slice(&self, start: usize, end: usize) -> Result<BorrowedVector<'_, V>, VectorError> {
        if start >= self.start && end <= self.end {
            let internal_start: usize = start - self.start;
            let internal_end: usize = end - self.start;

            let slice: &[V] = &self.slice[internal_start..=internal_end];

            BorrowedVector::try_new(slice, start, end)
        } else {
            let index: usize = if start < self.start { start } else { end };
            Err(VectorError::Indexing { index })
        }
    }

    /// Returns the length of the underlying slice.
    pub fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns a slice inside the underlying vector based on the offset range from `start` to `end`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The `start` or `end` is outside the supported range.
    /// `start` < `self.start` or `end` > `self.end` + 1.
    pub fn get_range(&self, start: usize, end: usize) -> Result<&[V], VectorError> {
        VectorError::check_order(start, end)?;

        if start < self.start {
            Err(VectorError::Indexing { index: self.start })
        } else if end > self.end + 1 {
            Err(VectorError::Indexing { index: self.end })
        } else {
            let start_offest: usize = start - self.start;
            let end_offset: usize = end - self.start;

            Ok(&self.slice[start_offest..end_offset])
        }
    }

    /// Returns an inclusive slice inside the underlying vector based on the offset range from `start` to `end`.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Indexing`] - The `start` or `end` is outside the supported range.
    /// `start` < `self.start` or `end` > `self.end`.
    pub fn get_range_inclusive(&self, start: usize, end: usize) -> Result<&[V], VectorError> {
        VectorError::check_order(start, end)?;

        if start < self.start {
            Err(VectorError::Indexing { index: self.start })
        } else if end > self.end {
            Err(VectorError::Indexing { index: self.end })
        } else {
            let start_offest: usize = start - self.start;
            let end_offset: usize = end - self.start;

            Ok(&self.slice[start_offest..=end_offset])
        }
    }
}

impl<V: Vectorable> Index<usize> for BorrowedVector<'_, V> {
    type Output = V;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        // Underflow will wrap around and panic
        &self.slice[index.wrapping_sub(self.start)]
    }
}

impl<V: Vectorable> Index<RangeInclusive<usize>> for BorrowedVector<'_, V> {
    type Output = [V];

    #[inline]
    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        let start: usize = range.start() - self.start;
        let end: usize = range.end() - self.start;

        &self.slice[start..=end]
    }
}

impl<V: Vectorable> Index<Range<usize>> for BorrowedVector<'_, V> {
    type Output = [V];

    #[inline]
    fn index(&self, range: Range<usize>) -> &Self::Output {
        let start: usize = range.start - self.start;
        let end: usize = range.end - self.start;

        &self.slice[start..end]
    }
}

impl<'a, V: Vectorable> Vector<V> for BorrowedVector<'a, V> {
    /// Returns the `start` index of the [`BorrowedVector`].
    ///
    /// This is the first index where an element is located.
    #[inline]
    fn start(&self) -> usize {
        self.start
    }

    /// Returns the `end` index of the [`BorrowedVector`].
    ///
    /// This is the last index where an element is located.
    #[inline]
    fn end(&self) -> usize {
        self.end
    }

    #[inline]
    fn as_slice(&self) -> &[V] {
        self.slice
    }
}

impl<'a, V: Vectorable> IntoIterator for BorrowedVector<'a, V> {
    type Item = &'a V;
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.iter()
    }
}
