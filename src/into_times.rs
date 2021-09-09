use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// How many times is an expectation expected to occur.
/// Implemented for usize and any range of usize values.
pub trait IntoTimes: Sealed {
    /// Provide the lower and upper bounds.
    fn into_times(self) -> (Bound<usize>, Bound<usize>);
}

/// Sealed is not exposed publicly so IntoTimes can be referenced, but not
/// implemented outside this crate.
pub trait Sealed {}

impl IntoTimes for usize {
    fn into_times(self) -> (Bound<usize>, Bound<usize>) {
        (Bound::Included(self), Bound::Included(self))
    }
}

impl Sealed for usize {}

macro_rules! impl_into_times {
    ($range_ty:ty) => {
        impl IntoTimes for $range_ty {
            fn into_times(self) -> (Bound<usize>, Bound<usize>) {
                fn cloned_range(b: Bound<&usize>) -> Bound<usize> {
                    match b {
                        Bound::Included(b) => Bound::Included(*b),
                        Bound::Excluded(b) => Bound::Excluded(*b),
                        Bound::Unbounded => Bound::Unbounded,
                    }
                }
                (
                    cloned_range(self.start_bound()),
                    cloned_range(self.end_bound()),
                )
            }
        }
        impl Sealed for $range_ty {}
    };
}

impl_into_times!(Range<usize>);
impl_into_times!(Range<&usize>);
impl_into_times!(RangeFrom<usize>);
impl_into_times!(RangeFrom<&usize>);
impl_into_times!(RangeInclusive<usize>);
impl_into_times!(RangeInclusive<&usize>);
impl_into_times!(RangeTo<usize>);
impl_into_times!(RangeTo<&usize>);
impl_into_times!(RangeToInclusive<usize>);
impl_into_times!(RangeToInclusive<&usize>);
impl_into_times!(RangeFull);
impl_into_times!((Bound<usize>, Bound<usize>));
impl_into_times!((Bound<&usize>, Bound<&usize>));
