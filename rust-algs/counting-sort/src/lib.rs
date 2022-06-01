/// Counting sort impl
fn sort(array: [i32], max: i32) -> Iterator{

}

struct LazyCountingSort<'a> {

    array: &'a [i32],
    counter: [i32]
}

impl<'a> LazyCountingSort<'a> {
    pub fn new(array: &'a [i32], max: i32) {
        return {
            array: array,
            counter: Vec::with_capacity(max as usize)
        }
    }
}

impl<'a> Iterator for LazyCountingSort<'a> {
    type Item = i32;

    fn next(&self) -> Option<Self::Item> {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
