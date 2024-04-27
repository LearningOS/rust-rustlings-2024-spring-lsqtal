/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        if self.count >= self.items.len() {
            self.items.push(value);
        } else {
            self.items[self.count] = value;
        }
        self.percolate_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
    

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		//0
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
    if right_idx > self.count || (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
        left_idx
    } else {
        right_idx
    }
    }

    fn percolate_up(&mut self, idx: usize) {
        let mut idx = idx;
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            let xs=self.parent_idx(idx);
            self.items.swap(idx, xs);
            idx = self.parent_idx(idx);
        }
}
fn percolate_down(&mut self, idx: usize) {
    let mut idx = idx;
    loop {
        let smallest_child_idx = self.smallest_child_idx(idx);
        if smallest_child_idx > self.count || !(self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
            break;
        }
        self.items.swap(idx, smallest_child_idx);
        idx = smallest_child_idx;
    }
}
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		//None
        if self.count == 0 {
            return None;
        }
        let root_value = self.items[1].clone();
        let last_value = self.items[self.count].clone();
        self.count -= 1;
        self.items[1] = last_value;
        self.percolate_down(1);
        Some(root_value)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}