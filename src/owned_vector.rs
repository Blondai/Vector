use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Range, RangeInclusive},
    slice::{Iter, IterMut},
};

use crate::{BorrowedVector, Vector, VectorError, Vectorable};

/// A wrapper struct around a generic [`Vec`] allowing the automatic calculation of indexing offsets.
///
/// The generic value needs to implement the [`Vectorable`] trait.
#[derive(Debug, Clone)]
pub struct OwnedVector<V: Vectorable> {
    /// The [`Vec`]tor containing the values.
    vector: Vec<V>,

    /// The start to allow the correct index offsetting.
    start: usize,

    /// The end to allow to assertion of the correct length.
    ///
    /// Note that the end is included.
    end: usize,
}

impl<V: Vectorable> OwnedVector<V> {
    /// Creates a new [`OwnedVector`] instance based on a given [`Vec`].
    ///
    /// # Errors
    ///
    /// * [`VectorError::Order`] - The order of the arguments is wrong.
    /// `start` > `end`.
    /// * [`VectorError::Length`] - The expected length does not match the provided one.
    /// `vec.len() != end - start + 1`.
    #[inline]
    pub fn from_vec(vec: Vec<V>, start: usize, end: usize) -> Result<Self, VectorError> {
        // Not possible as const fn (Vec deconstruction)
        VectorError::check_order(start, end)?;
        VectorError::check_len(vec.len(), start, end)?;

        Ok(Self {
            vector: vec,
            start,
            end,
        })
    }

    /// Creates a new [`OwnedVector`] with a given `value` at all positions.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Order`] - `start` > `end`.
    pub fn from_num(value: V, start: usize, end: usize) -> Result<Self, VectorError> {
        VectorError::check_order(start, end)?;

        // `end` - `start` si safe, because `check_order` passed
        let vector: Vec<V> = vec![value; end - start + 1];

        Ok(Self { vector, start, end })
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
        self.vector
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
        self.vector
            .get(index)
            .copied()
            .ok_or(VectorError::Indexing { index })
    }

    /// Returns an [`IterMut`] of the underlying [`Vec`].
    ///
    /// This is simply a getter of the `iter_mut` and will not consider the offest indexing.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        self.vector.iter_mut()
    }

    /// Slices into a [`OwnedVector`] and returns a corresponding [`BorrowedVector`].
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

            let slice: &[V] = &self.vector[internal_start..=internal_end];

            BorrowedVector::try_new(slice, start, end)
        } else {
            let index: usize = if start < self.start { start } else { end };
            Err(VectorError::Indexing { index })
        }
    }

    /// Returns the length of the underlying vector.
    pub fn len(&self) -> usize {
        self.vector.len()
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

            Ok(&self.vector[start_offest..end_offset])
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

            Ok(&self.vector[start_offest..=end_offset])
        }
    }

    /// Turns the [`OwnedVector`] into a [`BorrowedVector`] using the [`OwnedVector::as_slice`] method.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Order`] - The order of the arguments is wrong.
    /// `start` > `end`.
    /// * [`VectorError::Length`] - The expected length does not match the provided one.
    /// `vec.len() != end - start + 1`.a
    pub fn to_borrowed(&'_ self) -> Result<BorrowedVector<'_, V>, VectorError> {
        BorrowedVector::try_new(self.as_slice(), self.start, self.end)
    }
}

impl<V: Default + Vectorable> OwnedVector<V> {
    /// Creates a new [`OwnedVector`] instance based on a given `start` and `end`.
    /// This will be filled with the [`Default`] value of the generic.
    ///
    /// # Errors
    ///
    /// * [`VectorError::Order`] - The order of the arguments is wrong.
    /// `start` > `end`.
    pub fn new(start: usize, end: usize) -> Result<Self, VectorError> {
        VectorError::check_order(start, end)?;

        let vector: Vec<V> = vec![V::default(); end - start + 1];

        Ok(Self { vector, start, end })
    }
}

impl<V: Vectorable> Index<usize> for OwnedVector<V> {
    type Output = V;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        // Underflow will wrap around and panic
        &self.vector[index.wrapping_sub(self.start)]
    }
}

impl<V: Vectorable> IndexMut<usize> for OwnedVector<V> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // Underflow will wrap around and panic
        &mut self.vector[index.wrapping_sub(self.start)]
    }
}

impl<V: Vectorable> Index<RangeInclusive<usize>> for OwnedVector<V> {
    type Output = [V];

    #[inline]
    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        let start: usize = range.start() - self.start;
        let end: usize = range.end() - self.start;

        &self.vector[start..=end]
    }
}

impl<V: Vectorable> Index<Range<usize>> for OwnedVector<V> {
    type Output = [V];

    #[inline]
    fn index(&self, range: Range<usize>) -> &Self::Output {
        let start: usize = range.start - self.start;
        let end: usize = range.end - self.start;

        &self.vector[start..end]
    }
}

impl<V: Vectorable> Vector<V> for OwnedVector<V> {
    /// Returns the `start` index of the [`OwnedVector`].
    ///
    /// This is the first index where an element is located.
    #[inline]
    fn start(&self) -> usize {
        self.start
    }

    /// Returns the `end` index of the [`OwnedVector`].
    ///
    /// This is the last index where an element is located.
    #[inline]
    fn end(&self) -> usize {
        self.end
    }

    #[inline]
    fn as_slice(&self) -> &[V] {
        &self.vector
    }
}

impl<V: Vectorable> IntoIterator for OwnedVector<V> {
    type Item = V;
    type IntoIter = std::vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.vector.into_iter()
    }
}

impl<'a, V: Vectorable> IntoIterator for &'a OwnedVector<V> {
    type Item = &'a V;
    type IntoIter = Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V: Vectorable> IntoIterator for &'a mut OwnedVector<V> {
    type Item = &'a mut V;
    type IntoIter = IterMut<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
