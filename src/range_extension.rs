use std::ops::RangeInclusive;

pub trait RangeExtension {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<U: Sized + PartialOrd> RangeExtension for RangeInclusive<U> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || other.contains(self.start())
    }
}
