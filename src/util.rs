use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

// You can set the length of number of stored price - LENGTH
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct PriceArray<T, const LENGTH: usize> {
    array: [T; LENGTH],
    position: usize,
}

// PriceArray must have at least 1 value 
// We don't need to save much numbers of price. Of course we can set up LENGTH = 5 directly, but for for flexibility
// we use this way
impl<T, const LENGTH: usize> Default for PriceArray<T, LENGTH>
where
    T: Default + Copy,
{
    fn default() -> Self {
        if LENGTH == 0 {
            near_sdk::env::panic(b"PriceArray must have at least one value");
        } else if LENGTH > usize::MAX {
            near_sdk::env::panic(b"Too large number of price. Save less numbers");
        }
        Self {
            array: [T::default(); LENGTH],
            position: 0,
        }
    }
}

impl<T, const LENGTH: usize> PriceArray<T, LENGTH> {
    /// Generate new array of price and its length is LENGTH.
    pub fn new() -> Self
    where
        T: Default + Copy,
    {
        Self::default()
    }

    /// Push new value to this array
    pub fn push(&mut self, item: T) {
        let insert_position = (self.position + LENGTH) % LENGTH;
        self.position = (self.position + 1) % LENGTH;
        self.array[insert_position] = item;
    }

    /// Returns an iterator over the queued items.
    pub fn iter(&self) -> PriceIterator<'_, T, LENGTH> {
        PriceIterator {
            array: &self.array,
            start_position: self.position,
            count: LENGTH,
        }
    }
}

impl<'a, T: 'a, const LENGTH: usize> IntoIterator for &'a PriceArray<T, LENGTH> {
    type Item = &'a T;

    type IntoIter = PriceIterator<'a, T, LENGTH>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the queued values.
// the iterator could have been implemented alternatively as a chained
// iterator over two consecutive slices.
pub struct PriceIterator<'a, T, const LENGTH: usize> {
    array: &'a [T; LENGTH],
    start_position: usize,
    count: usize,
}

impl<'a, T: 'a, const LENGTH: usize> Iterator for PriceIterator<'a, T, LENGTH> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count.checked_sub(1)?;
        let item = &self.array[self.start_position % LENGTH];
        self.start_position += 1;
        Some(item)
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    #[test]
    fn simple_test() {
        let mut queue = PriceArray::<_, 5>::new();
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0]
        );

        queue.push(100);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 100]
        );

        queue.push(200);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 0, 100, 200]
        );

        queue.push(300);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 100, 200, 300]
        );

        queue.push(400);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 100, 200, 300, 400]
        );

        queue.push(500);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![100, 200, 300, 400, 500]
        );
    }

    #[quickcheck]
    fn extensive_test(input: Vec<u8>) -> TestResult {
        if input.len() < 5 {
            return TestResult::discard();
        }

        let mut reference_queue = VecDeque::with_capacity(5);
        let mut testing_queue = PriceArray::<u8, 5>::new();
        for item in input {
            if reference_queue.len() >= 5 {
                reference_queue.pop_front();
            }
            reference_queue.push_back(item);
            testing_queue.push(item);
        }

        let reference: Vec<_> = reference_queue.into_iter().collect();
        let testing: Vec<_> = testing_queue.iter().copied().collect();

        assert_eq!(reference, testing);

        TestResult::passed()
    }
}
