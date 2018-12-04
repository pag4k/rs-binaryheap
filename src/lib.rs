use std::cmp::Ordering;

//: Ord+PartialOrd

#[derive(Debug)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap {
            data: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        BinaryHeap {
            data: Vec::new(capacity),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.data.is_empty() {
            false => {
                let element = self.data.swap_remove(0);
                self.bubble_down(0);
                Some(element)
            },
            true  => None,
        }
    }
    //Should that return something?
    pub fn push(&mut self, element: T) {
        self.data.push(element);
        let last_index = self.len() - 1;
        self.bubble_up(last_index);
    }
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    fn get_parent(&self, index: usize) -> Option<usize> {
        match index {
            0 => None,
            _ => Some(((index as f32) / (2 as f32)).ceil() as usize - 1)
        }
    }
    fn get_left_child(&self, index: usize) -> Option<usize> {
        let child = 2 * index + 1;
        match child < self.len() {
            true  => Some(child),
            false => None,
        }
    }
    fn get_right_child(&self, index: usize) -> Option<usize> {
        let child = 2 * index + 2;
        match child < self.len() {
            true  => Some(child),
            false => None,
        }
    }
    fn bubble_up(&mut self, index: usize) {
        let parent = self.get_parent(index);
        if parent.is_none() { return; }
        let parent = parent.unwrap();
        if self.data.get(index).unwrap().cmp(self.data.get(parent).unwrap()) == Ordering::Greater {
            self.data.swap(index, parent);
            self.bubble_up(parent);
        }
    }
    fn bubble_down(&mut self, index: usize) {
        let left_child = self.get_left_child(index);
        let right_child = self.get_right_child(index);
        let swap_candidate = match (left_child, right_child) {
            (Some(left), Some(right)) =>
                if self.data.get(left).unwrap().cmp(self.data.get(right).unwrap()) == Ordering::Greater { left } else { right },
            (Some(left), None       ) => left,
            (None      , Some(right)) => right,
            (None      , None       ) => { return; },
        };
        if self.data.get(index).unwrap().cmp(self.data.get(swap_candidate).unwrap()) == Ordering::Less {
            self.data.swap(index, swap_candidate);
            self.bubble_down(swap_candidate);
        }
    }
}

pub struct BinaryHeapIterator<'a,T: 'a> {
    binary_heap: &'a BinaryHeap<T>,
    index: usize,
}

impl<'a,T> Iterator for BinaryHeapIterator<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.index += 1;
        self.binary_heap.data.get(self.index-1)
    }
}

impl<'a,T> IntoIterator for &'a BinaryHeap<T> {
    type Item = &'a T;
    type IntoIter = BinaryHeapIterator<'a,T>;
    fn into_iter(self) -> Self::IntoIter {
        BinaryHeapIterator { binary_heap: &self, index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc_examples() {
        // Type inference lets us omit an explicit type signature (which
        // would be `BinaryHeap<i32>` in this example).
        let mut heap = BinaryHeap::new();

        // We can use peek to look at the next item in the heap. In this case,
        // there's no items in there yet so we get None.
        assert_eq!(heap.peek(), None);

        // Let's add some scores...
        heap.push(1);
        heap.push(5);
        heap.push(2);

        // Now peek shows the most important item in the heap.
        assert_eq!(heap.peek(), Some(&5));

        // We can check the length of a heap.
        assert_eq!(heap.len(), 3);

        // We can iterate over the items in the heap, although they are returned in
        // a random order.
        for x in &heap {
            println!("{}", x);
        }

        // If we instead pop these scores, they should come back in order.
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);

        // We can clear the heap of any remaining items.
        heap.clear();

        // The heap should now be empty.
        assert!(heap.is_empty())
    }
}
