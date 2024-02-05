use std::{cmp::Ord, marker::Copy};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Clone + Ord + Copy> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut data = Vec::from(_input);
        data.dedup();
        data.sort_unstable();

        CustomSet { data }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.data.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if self.contains(&_element) {
            return;
        }

        self.data.push(_element);
        self.data.sort_unstable();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.data.is_empty() {
            return true;
        }

        if _other.data.len() < self.data.len() {
            return false;
        }

        self.data.iter().all(|i| _other.contains(i))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        !self.data.iter().any(|i| _other.contains(i))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let mut data: Vec<T> =
            Vec::from_iter(self.data.iter().filter(|i| _other.contains(i)).copied());

        data.sort_unstable();

        CustomSet { data }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut data: Vec<T> = Vec::from_iter(
            self.data
                .iter()
                .filter(|i| !_other.data.contains(&i))
                .copied(),
        );

        CustomSet { data }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let data: Vec<T> = Vec::from_iter(
            self.data
                .iter()
                .copied()
                .filter(|i| !_other.contains(i))
                .chain(_other.data.iter().copied()),
        );

        CustomSet { data }
    }
}
