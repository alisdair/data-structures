#[derive(Debug)]
pub struct BinaryHeap<T> {
    vec: Vec<T>
}

const DEFAULT_HEAP_CAPACITY: usize = 256;

impl<T> BinaryHeap<T> where T: Ord {
    pub fn new() -> BinaryHeap<T> {
        BinaryHeap::with_capacity(DEFAULT_HEAP_CAPACITY)
    }

    pub fn from_vec(vec: Vec<T>) -> BinaryHeap<T> {
        BinaryHeap { vec: vec }
    }

    pub fn with_capacity(capacity: usize) -> BinaryHeap<T> {
        BinaryHeap { vec: Vec::with_capacity(capacity) }
    }

    pub fn peek(&self) -> Option<&T> {
        self.vec.get(0)
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            return None;
        }

        let last = self.vec.len() - 1;
        self.vec.swap(0, last);

        let item = self.vec.pop();
        self.sink(0);

        item
    }

    fn sink(&mut self, index: usize) {
        let len = self.vec.len();
        let mut parent = index;
        while parent * 2 + 1 < len {
            let left = parent * 2 + 1;
            let right = left + 1;

            let child = if right < len && self.vec[left] < self.vec[right] {
                right
            } else {
                left
            };

            if self.vec[child] <= self.vec[parent] {
                break;
            }

            self.vec.swap(parent, child);
            parent = child;
        }
    }

    pub fn push(&mut self, item: T) {
        self.vec.push(item);
        let index = self.vec.len() - 1;
        self.swim(index);
    }

    fn swim(&mut self, index: usize) {
        let mut child = index;
        while child > 0 {
            let parent = (child - 1) / 2;

            if self.vec[parent] >= self.vec[child] {
                return;
            }

            self.vec.swap(child, parent);
            child = parent;
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[cfg(test)]

mod tests {
    use super::BinaryHeap;

    #[test]
    fn test_new() {
        let heap = BinaryHeap::<i32>::new();
        assert_eq!(256, heap.capacity());
    }

    #[test]
    fn test_with_capacity() {
        let heap = BinaryHeap::<i32>::with_capacity(512);
        assert_eq!(512, heap.capacity());
        assert_eq!(0, heap.len());
    }

    #[test]
    fn test_push_peek() {
        let mut heap = BinaryHeap::<i32>::new();

        assert_eq!(None, heap.peek());

        heap.push(53);
        assert_eq!(&53, heap.peek().unwrap());
    }

    #[test]
    fn test_swimming() {
        let mut heap = BinaryHeap::<i32>::new();

        heap.push(53);
        heap.push(127);
        assert_eq!(&127, heap.peek().unwrap());
        heap.push(23);
        assert_eq!(&127, heap.peek().unwrap());
        heap.push(12345);
        assert_eq!(&12345, heap.peek().unwrap());
    }

    #[test]
    fn test_pop() {
        let xs = vec![97, 84, 58, 73, 80, 30, 40, 49, 61, 66];
        let mut heap = BinaryHeap::<i32>::from_vec(xs);

        assert_eq!(97, heap.pop().unwrap());
        assert_eq!(84, heap.pop().unwrap());
        assert_eq!(80, heap.pop().unwrap());
        assert_eq!(73, heap.pop().unwrap());
        assert_eq!(66, heap.pop().unwrap());
        assert_eq!(61, heap.pop().unwrap());
        assert_eq!(58, heap.pop().unwrap());
        assert_eq!(49, heap.pop().unwrap());
        assert_eq!(40, heap.pop().unwrap());
        assert_eq!(30, heap.pop().unwrap());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_len() {
        let mut heap = BinaryHeap::<i32>::new();

        assert_eq!(0, heap.len());

        heap.push(53);
        assert_eq!(1, heap.len());

        heap.push(127);
        heap.push(23);
        heap.push(12345);
        assert_eq!(4, heap.len());
    }
}
