
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


pub struct MaxHeap<'a> {
    heap_len: usize,
    array: &'a mut [i32]
}


impl<'a> MaxHeap<'a> {

    pub fn new(array: &'a mut [i32]) -> Self {
        Self {
            heap_len: array.len(),
            array: array
        }
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
        assert!(node < self.heap_len, "Out of bound node! index {} should be less than {} (heap size)", node, self.heap_len);

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
        // NOTE: If left and right are the same, left would bubble up and 
        // array[parent(i)] == array[i].
        //
        // This scenario does not violate the heap property because
        // the heap property acconts for parent equality.
        // eg. for a max heap, parent(i) >= i. so it would beok
        else if left > current {
            let left_index = left_child(node);
            self.array[node] = left;
            self.array[left_index] = current;
            self.max_heapify(left_index);
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
            self.heap_len = tail_index;
            self.max_heapify(0);
            self.sort_recursion();
        }
    }

    /// Inplace sorts the heap and returns slice to sorted array
    pub fn sort(&mut self) -> &[i32]{
        // By starting at the last parent node in the heap
        // it skips `n/2` items in the array, (ie all leaves).
        // This is ok beucase subtrees composed by the leaves 
        // are max-heapified by definition.
        //
        // Apparently this step complexity O(lg n) as opposed to O(n)
        // but I don't fully understand why
        let last_leaf = self.array.len() - 1;
        let last_node = parent(last_leaf);
        for i in (1..=last_node).rev() {
            self.max_heapify(i);
        }
        self.sort_recursion();
        &self.array
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
        let mut heap = MaxHeap::new(&mut array);

        let slice = heap.sort();

        assert_eq!(slice, &[1,2,3,5,7,9]);
    }

}
