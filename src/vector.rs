//! This module contains the implementation of the [`Vector`] struct and all related things.

use std::fmt::{self, Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::Info;

/// A struct for simplifying and hardening the use of [`Vec`]tors.
#[derive(Debug, Clone)]
pub struct Vector<T: Copy> {
    data: Vec<T>,
    info: Info<T>,
}

impl<T: Copy> Vector<T> {
    /// Creates an empty [`Vector`] instance based on the [`Info`] given.
    ///
    /// This will always use [`Vec::with_capacity`].
    #[inline]
    pub fn with_capacity(info: Info<T>) -> Self {
        let data: Vec<T> = Vec::with_capacity(info.len());
        Self { data, info }
    }

    /// Creates a new [`Vector`] instance with all entries set to a given `value`.
    ///
    /// This uses the capacity from [`Info`].
    #[inline]
    pub fn with_value(value: T, info: Info<T>) -> Self {
        let data: Vec<T> = vec![value; info.len()];
        Self { data, info }
    }

    /// Creates a new [`Vector`] based on a given [`Vec`].
    ///
    /// The lengths of the `vector` and the provided [`Info`] instance must be the same.
    #[inline]
    pub fn from_data(data: Vec<T>, info: Info<T>) -> Result<Self, VectorError> {
        VectorError::check_length(&data, &info)?;

        Ok(Self { data, info })
    }

    /// Returns the length of the [`Info`]
    #[inline]
    pub fn len(&self) -> usize {
        self.info.len()
    }

    /// Returns the start index.
    #[inline]
    pub fn start(&self) -> usize {
        self.info.start()
    }

    /// Returns the end index.
    #[inline]
    pub fn end(&self) -> usize {
        self.info.end()
    }

    /// Returns a reference to the [`Info`].
    #[inline]
    pub fn info(&self) -> &Info<T> {
        &self.info
    }

    /// Returns a mutable reference to a value at a specific `index` if this value is present.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let start = self.info.start();
        let end = self.info.end();

        if index >= start && index <= end {
            let internal_index = index - start;
            Some(&mut self.data[internal_index])
        } else {
            None
        }
    }

    /// Tries to [`AddAssign`].
    ///
    /// This will return an error when the info is not compatible.
    pub fn try_add_assign(&mut self, other: &Vector<T>) -> Result<(), VectorError>
    where
        T: AddAssign + PartialEq,
    {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a += *b);

        Ok(())
    }

    /// Tries to [`SubAssign`].
    ///
    /// This will return an error when the info is not compatible.
    pub fn try_sub_assign(&mut self, other: &Vector<T>) -> Result<(), VectorError>
    where
        T: SubAssign + PartialEq,
    {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a -= *b);

        Ok(())
    }

    /// Tries to [`MulAssign`].
    ///
    /// This will return an error when the info is not compatible.
    pub fn try_mul_assign(&mut self, other: &Vector<T>) -> Result<(), VectorError>
    where
        T: MulAssign + PartialEq,
    {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a *= *b);

        Ok(())
    }

    /// Tries to [`DivAssign`].
    ///
    /// This will return an error when the info is not compatible.
    pub fn try_div_assign(&mut self, other: &Vector<T>) -> Result<(), VectorError>
    where
        T: DivAssign + PartialEq,
    {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a /= *b);

        Ok(())
    }

    /// Turns a [`Vector`] into a [`Iterator`].
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl<T: Copy> Index<usize> for Vector<T> {
    type Output = T;

    /// Returns a reference to the data at a specific index based on the indexing from [`Info`].
    ///
    /// For indices outside the [`Info`]-supported range this will return the fallback values.
    ///
    /// This will automatically do the shifting.
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let start: usize = self.info.start();
        let end: usize = self.info.end();

        if index < start {
            self.info.fallback_start_ref()
        } else if index > end {
            self.info.fallback_end_ref()
        } else {
            let internal_index: usize = index - start;
            &self.data[internal_index]
        }
    }
}

impl<T: Copy> IndexMut<usize> for Vector<T> {
    /// Returns a mutable reference to the data at a specific index based on the indexing from [`Info`].
    ///
    /// For indices outside the [`Info`]-supported range this will panic.
    ///
    /// This will automatically do the shifting.
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = self.info.start();
        let end = self.info.end();

        if index < start || index > end {
            panic!(
                "Index {} is out of the mutable range [{}, {}]",
                index, start, end
            );
        }

        let internal_index = index - start;
        &mut self.data[internal_index]
    }
}

impl<'a, 'b, T> Add<&'b Vector<T>> for &'a Vector<T>
where
    T: Copy + Add<Output = T> + PartialEq,
{
    type Output = Result<Vector<T>, VectorError>;

    /// Elementwise [`Add`]ition.
    ///
    /// This will return an error if the [`Info`] instances are incompatible.
    fn add(self, other: &'b Vector<T>) -> Self::Output {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        let new_data: Vec<T> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Ok(Vector {
            data: new_data,
            info: self.info,
        })
    }
}

impl<'a, 'b, T> Sub<&'b Vector<T>> for &'a Vector<T>
where
    T: Copy + Sub<Output = T> + PartialEq,
{
    type Output = Result<Vector<T>, VectorError>;

    /// Elementwise [`Sub`]traction.
    ///
    /// This will return an error if the [`Info`] instances are incompatible.
    fn sub(self, other: &'b Vector<T>) -> Self::Output {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        let new_data: Vec<T> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a - *b)
            .collect();

        Ok(Vector {
            data: new_data,
            info: self.info,
        })
    }
}

impl<'a, T> Neg for &'a Vector<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Vector<T>;

    /// Elementwise [`Neg`]ation.
    fn neg(self) -> Self::Output {
        let new_data: Vec<T> = self.data.iter().map(|a| -*a).collect();

        Vector {
            data: new_data,
            info: self.info,
        }
    }
}

impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a Vector<T>
where
    T: Copy + Mul<Output = T> + PartialEq,
{
    type Output = Result<Vector<T>, VectorError>;

    /// Elementwise [`Mul`]tiplication.
    ///
    /// This will return an error if the [`Info`] instances are incompatible.
    fn mul(self, other: &'b Vector<T>) -> Self::Output {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        let new_data: Vec<T> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a * *b)
            .collect();

        Ok(Vector {
            data: new_data,
            info: self.info,
        })
    }
}

impl<'a, 'b, T> Div<&'b Vector<T>> for &'a Vector<T>
where
    T: Copy + Div<Output = T> + PartialEq,
{
    type Output = Result<Vector<T>, VectorError>;

    /// Elementwise [`Div`]ision.
    ///
    /// This will return an error if the [`Info`] instances are incompatible.
    fn div(self, other: &'b Vector<T>) -> Self::Output {
        VectorError::check_interval(&self.info, &other.info)?;
        VectorError::check_fallback(&self.info, &other.info)?;

        let new_data: Vec<T> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a / *b)
            .collect();

        Ok(Vector {
            data: new_data,
            info: self.info,
        })
    }
}

impl<'a, T> Mul<T> for &'a Vector<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vector<T>;

    /// Scalar [`Mul`]tiplication.
    fn mul(self, other: T) -> Self::Output {
        let new_data: Vec<T> = self.data.iter().map(|a| *a * other).collect();

        Vector {
            data: new_data,
            info: self.info,
        }
    }
}

impl<'a, T> Div<T> for &'a Vector<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Vector<T>;

    /// Scalar [`Div`]ision.
    fn div(self, other: T) -> Self::Output {
        let new_data: Vec<T> = self.data.iter().map(|a| *a / other).collect();

        Vector {
            data: new_data,
            info: self.info,
        }
    }
}

impl<'a, T> AddAssign<&'a Vector<T>> for Vector<T>
where
    T: Copy + AddAssign + PartialEq,
{
    /// Elementwise [`AddAssign`].
    ///
    /// This will panic if the [`Info`] instances are incompatible.
    ///
    /// This uses the [`Vector::try_add_assign`] method and [`Result::unwrap`]s it.
    #[inline]
    fn add_assign(&mut self, other: &'a Vector<T>) -> () {
        self.try_add_assign(other).unwrap();
    }
}

impl<'a, T> SubAssign<&'a Vector<T>> for Vector<T>
where
    T: Copy + SubAssign + PartialEq,
{
    /// Elementwise [`SubAssign`].
    ///
    /// This will panic if the [`Info`] instances are incompatible.
    ///
    /// This uses the [`Vector::try_sub_assign`] method and [`Result::unwrap`]s it.
    #[inline]
    fn sub_assign(&mut self, other: &'a Vector<T>) -> () {
        self.try_sub_assign(other).unwrap();
    }
}

impl<'a, T> MulAssign<&'a Vector<T>> for Vector<T>
where
    T: Copy + MulAssign + PartialEq,
{
    /// Elementwise [`MulAssign`].
    ///
    /// This will panic if the [`Info`] instances are incompatible.
    ///
    /// This uses the [`Vector::try_mul_assign`] method and [`Result::unwrap`]s it.
    #[inline]
    fn mul_assign(&mut self, other: &'a Vector<T>) -> () {
        self.try_mul_assign(other).unwrap();
    }
}

impl<'a, T> DivAssign<&'a Vector<T>> for Vector<T>
where
    T: Copy + DivAssign + PartialEq,
{
    /// Elementwise [`DivAssign`].
    ///
    /// This will panic if the [`Info`] instances are incompatible.
    ///
    /// This uses the [`Vector::try_div_assign`] method and [`Result::unwrap`]s it.
    #[inline]
    fn div_assign(&mut self, other: &'a Vector<T>) -> () {
        self.try_div_assign(other).unwrap();
    }
}

/// An enum for handling error involving the [`Vector`] struct.
#[derive(Debug, PartialEq)]
pub enum VectorError {
    /// The `start`s or `end`s of the [`Info`]s is not the same.
    IncompatibleInterval {
        start_1: usize,
        end_1: usize,
        start_2: usize,
        end_2: usize,
    },
    /// The length of the `data` is not equal to the `len` of the [`Info`].
    InvalidLength {
        vector_length: usize,
        info_length: usize,
    },
    /// The `fallback_stars`s or `fallsback_end`s of the [`Info`]s is not the same.
    IncompatibleFallback,
}

impl VectorError {
    /// Checks if two [`Info`]s have the same `start` and `end` parameters.
    #[inline]
    fn check_interval<T: Copy>(info_1: &Info<T>, info_2: &Info<T>) -> Result<(), Self> {
        if info_1.start() == info_2.start() && info_1.end() == info_2.end() {
            Ok(())
        } else {
            Err(Self::IncompatibleInterval {
                start_1: info_1.start(),
                end_1: info_1.end(),
                start_2: info_2.start(),
                end_2: info_2.end(),
            })
        }
    }

    /// Checks if a [`Vec`] and a [`Info`] have the same `len`.
    #[inline]
    fn check_length<T: Copy>(vector: &Vec<T>, info: &Info<T>) -> Result<(), Self> {
        if vector.len() == info.len() {
            Ok(())
        } else {
            Err(Self::InvalidLength {
                vector_length: vector.len(),
                info_length: info.len(),
            })
        }
    }

    /// Checks if two [`Info`]s have the same `fallback_start` and `fallback_end` parameters.
    #[inline]
    fn check_fallback<T: Copy + PartialEq>(info_1: &Info<T>, info_2: &Info<T>) -> Result<(), Self> {
        if info_1.fallback_start() == info_2.fallback_start()
            && info_1.fallback_end() == info_2.fallback_end()
        {
            Ok(())
        } else {
            Err(Self::IncompatibleFallback)
        }
    }
}

impl Display for VectorError {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        match self {
            Self::IncompatibleInterval {
                start_1,
                end_1,
                start_2,
                end_2,
            } => write!(
                format,
                "Incompatible Info: start_1 = {}, start_2 = {}, end_1 = {}, end_2 = {}",
                start_1, start_2, end_1, end_2
            ),
            Self::InvalidLength {
                vector_length,
                info_length,
            } => write!(
                format,
                "Invalid Length: vector length = {}, info length = {}",
                vector_length, info_length
            ),
            Self::IncompatibleFallback => write!(format, "Incompatible Fallback values"),
        }
    }
}

impl std::error::Error for VectorError {}
