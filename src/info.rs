//! This module contains the implementation of the [`Info`] struct and all related things.

use std::fmt::{self, Display, Formatter};

/// A struct for handling the metadata of a [`Vector`].
#[derive(Debug, Copy, Clone)]
pub struct Info<T: Copy> {
    /// The start of the vector.
    start: usize,

    /// The end of the vector.
    end: usize,

    /// The fallback value when the index is smaller than start.
    fallback_start: T,

    /// The fallback value when the index is larger than end.
    fallback_end: T,
}

impl<T: Copy> Info<T> {
    /// Creates a new [`Info`] instance.
    ///
    /// This will return an [`InfoError`] if the `end` is smaller than the `start`.
    #[inline]
    pub fn new(
        start: usize,
        end: usize,
        fallback_start: T,
        fallback_end: T,
    ) -> Result<Self, InfoError> {
        InfoError::check_interval(start, end)?;

        Ok(Self {
            start,
            end,
            fallback_start,
            fallback_end,
        })
    }

    /// Creates a new [`Info`] instance, where `fallback_start = fallback_end = fallback`.
    ///
    /// This will return an [`InfoError`] if the `end` is smaller than the `start`.
    #[inline]
    pub fn new_single_fallback(start: usize, end: usize, fallback: T) -> Result<Self, InfoError> {
        InfoError::check_interval(start, end)?;

        Ok(Self {
            start,
            end,
            fallback_start: fallback,
            fallback_end: fallback,
        })
    }

    /// Returns the length.
    ///
    /// This is `end - start + 1`.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }

    /// Returns the `start` index.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the `end` index.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the `fallback_start` value.
    ///
    /// This is the value that should be returned when indexing below `start`.
    #[inline]
    pub fn fallback_start(&self) -> T {
        self.fallback_start
    }

    /// Returns a reference to the `fallback_start` value.
    ///
    /// This is the value that should be returned when indexing below `start`.
    #[inline]
    pub(crate) fn fallback_start_ref(&self) -> &T {
        &self.fallback_start
    }

    /// Returns the `fallback_end` value.
    ///
    /// This is the value that should be returned when indexing beyond `end`.
    #[inline]
    pub fn fallback_end(&self) -> T {
        self.fallback_end
    }

    /// Returns a reference to the `fallback_end` value.
    ///
    /// This is the value that should be returned when indexing beyond `end`.
    #[inline]
    pub(crate) fn fallback_end_ref(&self) -> &T {
        &self.fallback_end
    }
}

impl<T: Copy + Display> Display for Info<T> {
    /// Displays the [`Info`] instance.
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(
            format,
            "start: {}, end: {}, fallback start: {}, fallback end: {}",
            self.start, self.end, self.fallback_start, self.fallback_end
        )
    }
}

/// An enum for handling error involving the [`Info`] struct.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InfoError {
    /// The `end` index is smaller than the `start` index.
    InvalidInterval { start: usize, end: usize },
}

impl InfoError {
    /// Checks if the `start` is smaller or equal the `end`.
    #[inline]
    pub fn check_interval(start: usize, end: usize) -> Result<(), InfoError> {
        if start <= end {
            Ok(())
        } else {
            Err(InfoError::InvalidInterval { start, end })
        }
    }
}

impl Display for InfoError {
    /// Displays the [`InfoError`] instance.
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        match self {
            InfoError::InvalidInterval { start, end } => {
                write!(format, "Invalid interval: [{}, {}]", start, end)
            }
        }
    }
}

impl std::error::Error for InfoError {}
