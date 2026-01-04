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
    #[inline]
    fn start(&self) -> usize {
        self.start
    }

    #[inline]
    fn end(&self) -> usize {
        self.end
    }

    #[inline]
    fn as_slice(&self) -> &[V] {
        self.slice
    }

    fn slice(&self, start: usize, end: usize) -> Result<BorrowedVector<'_, V>, VectorError> {
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

    #[inline]
    fn len(&self) -> usize {
        self.slice.len()
    }

    #[inline]
    fn get(&self, index: usize) -> Result<V, VectorError> {
        self.slice
            // Underflow will wrap around and return a `None` variant
            .get(index.wrapping_sub(self.start))
            .copied()
            .ok_or(VectorError::Indexing { index })
    }

    #[inline]
    fn get_absolute(&self, index: usize) -> Result<V, VectorError> {
        self.slice
            .get(index)
            .copied()
            .ok_or(VectorError::Indexing { index })
    }

    fn get_range(&self, start: usize, end: usize) -> Result<&[V], VectorError> {
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

    fn get_range_inclusive(&self, start: usize, end: usize) -> Result<&[V], VectorError> {
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

impl<'a, V: Vectorable> IntoIterator for BorrowedVector<'a, V> {
    type Item = &'a V;
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.iter()
    }
}
