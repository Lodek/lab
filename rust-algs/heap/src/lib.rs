
// NOTE these indexes gave me trouble.
// They were a lot easier to think over for a
// 1-based array.
// Figure out a better way of thinking about them

// Maybe turn these into macros?
fn left_child(node: usize) -> usize {
    (2*node) + 1
}

fn right_child(node: usize) -> usize {
    (2*node) + 2
}

fn parent(node: usize) -> usize {
    (node-1) / 2
}


pub struct Heap<'a> {
    len: usize,
    array: &'a mut [i32]
}


impl<'a> Heap<'a> {

    pub fn new(array: &'a mut [i32]) -> Self {
        Heap {
            heap_len: array.len(),
            array: array
        }
    }

    pub fn slice(&self) -> &[i32] {
        &self.array
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    /// `max_heapify` takes a subtree starting at `node` and transforms it
    /// into a max heap.
    /// `max_heapify` assumes as an invariant that the left and right sub-trees
    /// of `node` are max heaps.
    /// 
    /// Panics if `node` is greater than the heap size.
    ///
    /// This is a recursive function that essentially pushes the value of `node`
    /// down the heap tree and the biggest value in the heap becomes the
    /// root.
    ///
    /// O(h) runtime, where h is the height given by the subtree starting at `node`.
    /// This runtime is because for each tree level, there are a constant number of 
    /// operations to be performed.
    fn max_heapify(&mut self, node: usize) {
        if node >= self.heap_len {
            panic!("Out of bound node! {} is greater than heap size", node);
        }

        let current = self.array[node];
        // Love this. An elegant way to avoid complicating the ifs.
        let right = self.getter(right_child(node)).unwrap_or(i32::MIN);
        let left = self.getter(left_child(node)).unwrap_or(i32::MIN);

        if right > current && right > left{
            let right_index = right_child(node);
            self.array[node] = right;
            self.array[right_index] = current;
            self.max_heapify(right_index);
        }
        // TODO I am unsure how the heap would behave with equal 
        // if left and right were the same.
        // Left would bubble up but the then max heap property is violated, no?
        else if left > current {
            let left_index = left_child(node);
            self.array[node] = left;
            self.array[left_index] = current;
            self.max_heapify(left_index);
        }
    }

    /// Transforms an unordered array into a max heap.
    ///
    /// Apparently this has complexity O(lg n) as opposed to O(n)
    /// but I don't fully understand why
    fn build_max_heap(&mut self) {
        for i in (1..=(self.array.len() / 2)).rev() {
            self.max_heapify(i);
        }
    }

    /// Recursively sorts the heap, bubbling
    /// the biggest value to the end of the array.
    ///
    /// Assumes the heap is properly formed.
    fn sort_recursion(&mut self) {
        if self.heap_len > 1 {
            let tail_index = self.heap_len - 1;

            let head = self.array[0];
            let tail = self.array[tail_index];
            self.array[0] = tail;
            self.array[tail_index] = head;
            self.heap_lenlen -= 1;
            self.max_heapify(0);
            self.sort();
        }
    }

    /// Inplace sorts the heap
    pub fn sort(&mut self) {
        self.build_max_heap();
        self.sort_recursion();
    }

    fn getter(&self, index: usize) -> Option<i32> {
        if index < self.len() {
            Some(self.array[index])
        }
        else {
            None
        }
    }
}


// TODO Implement a "min heap" and the `Iterator` trait for a lazily
// ascending sorted heap implementation.


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_heap() {
        let mut array = [9, 5, 2, 3, 1, 7];
        let mut heap = Heap::new(&mut array);

        heap.sort();

        assert_eq!(heap.slice(), &[1,2,3,5,7,9]);
    }

}
