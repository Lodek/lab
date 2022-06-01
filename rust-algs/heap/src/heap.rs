struct Iter<'a, P>(&'a mut P) where P: Heap;

trait Heap {

    fn heapify(&mut self);



    fn sort_one(&mut self) -> i32;

    fn iter(&mut self) -> Iter {

    }

    fn sort(&mut self) -> &[i32] {
        //blanket impl
    }
}


struct MaxHeap<'a> {
    array: &'a mut [i32],
    heap_len: usize
}


struct MinHeap{
    array: &'a mut [i32],
    heap_len: usize
}
