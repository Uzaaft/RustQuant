// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// See LICENSE or <https://www.gnu.org/licenses/>.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::autodiff::{variables::variable::Variable, vertex::Arity};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: MIN
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Overload the `Min` trait.
pub trait Min<T> {
    /// Return type of `Min`
    type Output;

    /// Overloaded `min` function.
    fn min(&self, other: T) -> Self::Output;
}

// min{ Variable<'v> , Variable<'v> }
impl<'v> Min<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn min(&self, rhs: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, rhs.graph));

        Self::Output {
            graph: self.graph,
            value: self.value.min(rhs.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, rhs.index],
                &[
                    if self.value < rhs.value { 1.0 } else { 0.0 },
                    if self.value > rhs.value { 1.0 } else { 0.0 },
                ],
            ),
        }
    }
}

// min{ Variable<'v> , f64 }
impl<'v> Min<f64> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn min(&self, rhs: f64) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: self.value.min(rhs),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[if self.value < rhs { 1.0 } else { 0.0 }, 0.0],
            ),
        }
    }
}

// min{ f64 , Variable<'v> }
impl<'v> Min<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    #[inline]
    fn min(&self, rhs: Variable<'v>) -> Self::Output {
        Self::Output {
            graph: rhs.graph,
            value: f64::min(*self, rhs.value),
            index: rhs.graph.push(
                Arity::Binary,
                &[rhs.index, rhs.index],
                &[0.0, if self < &rhs.value { 1.0 } else { 0.0 }],
            ),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// OVERLOADING: MAX
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Overload the `Max` trait.
pub trait Max<T> {
    /// Return type of `Max`
    type Output;

    /// Overloaded `max` function.
    fn max(&self, other: T) -> Self::Output;
}

// max{ Variable<'v> , Variable<'v> }
impl<'v> Max<Variable<'v>> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn max(&self, rhs: Variable<'v>) -> Self::Output {
        assert!(std::ptr::eq(self.graph, rhs.graph));

        Self::Output {
            graph: self.graph,
            value: self.value.max(rhs.value),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, rhs.index],
                &[
                    if self.value > rhs.value { 1.0 } else { 0.0 },
                    if self.value < rhs.value { 1.0 } else { 0.0 },
                ],
            ),
        }
    }
}

// max{ Variable<'v> , f64 }
impl<'v> Max<f64> for Variable<'v> {
    type Output = Variable<'v>;

    #[inline]
    fn max(&self, rhs: f64) -> Self::Output {
        Self::Output {
            graph: self.graph,
            value: self.value.max(rhs),
            index: self.graph.push(
                Arity::Binary,
                &[self.index, self.index],
                &[if self.value > rhs { 1.0 } else { 0.0 }, 0.0],
            ),
        }
    }
}

// max{ f64 , Variable<'v> }
impl<'v> Max<Variable<'v>> for f64 {
    type Output = Variable<'v>;

    #[inline]
    fn max(&self, rhs: Variable<'v>) -> Self::Output {
        Self::Output {
            graph: rhs.graph,
            value: f64::max(*self, rhs.value),
            index: rhs.graph.push(
                Arity::Binary,
                &[rhs.index, rhs.index],
                &[0.0, if self > &rhs.value { 1.0 } else { 0.0 }],
            ),
        }
    }
}
