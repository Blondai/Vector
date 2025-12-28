use std::fmt::Debug;

#[allow(unused_imports)]
use crate::Vector;

/// Helper trait to only allow numerics in [`Vector`].
///
/// No special requirements except [`Copy`] and [`Debug`].
///
/// This trait is automatically implemented for all basic numeric types.
pub trait Vectorable: Copy + Debug {}

impl Vectorable for f64 {}
impl Vectorable for f32 {}
impl Vectorable for i128 {}
impl Vectorable for i64 {}
impl Vectorable for i32 {}
impl Vectorable for i16 {}
impl Vectorable for i8 {}
impl Vectorable for isize {}
impl Vectorable for u128 {}
impl Vectorable for u64 {}
impl Vectorable for u32 {}
impl Vectorable for u16 {}
impl Vectorable for u8 {}
impl Vectorable for usize {}
