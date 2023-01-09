//! Enumerate value structures definition.

use std::fmt::Debug;

/// # Enumerate value structure `ReferVal`
///
/// Used to fill those params which can be used as a reference value to another value, or a simple value itself.
///
/// ReferVal<T, U> => Refer(T) or Value(U)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReferVal<T, U>
where
    T: Copy + PartialEq + Clone + Debug,
    U: Copy + PartialEq + Clone + Debug,
{
    Refer(T),
    Value(U),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Strict<T>
where
    T: Copy + PartialEq + Clone + Debug,
{
    Strict(T),
    Loose(T),
    None,
}
