pub struct BinarySearchTree<T> {
    root: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BinarySearchTree<T> where T: Ord
{
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, item: T) {
        match self.root {
            None => self.root = Some(Box::new(Node::new(item))),
            Some(ref mut link) => link.insert(item),
        };
    }

    pub fn find(&self, item: T) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref link) => link.find(item),
        }
    }
}

impl<T> Node<T> where T: Ord
{
    pub fn new(item: T) -> Self {
        Node {
            item: item,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, item: T) {
        if self.item == item {
            self.item = item;
            return
        }

        let next = if item < self.item {
            &mut self.left
        } else {
            &mut self.right
        };

        match next {
            &mut None => *next = Some(Box::new(Node::new(item))),
            &mut Some(ref mut link) => link.insert(item),
        }
    }

    pub fn find(&self, item: T) -> Option<&T> {
        if item == self.item {
            return Some(&self.item)
        }

        let next = if item < self.item {
            &self.left
        } else {
            &self.right
        };

        match next {
            &None => None,
            &Some(ref link) => link.find(item),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    #[test]
    fn test_basics() {
        let mut tree = BinarySearchTree::<i32>::new();

        assert_eq!(tree.find(1), None);

        tree.insert(5);
        tree.insert(1);
        tree.insert(10);
        tree.insert(2);

        assert_eq!(tree.find(1), Some(&1));
        assert_eq!(tree.find(2), Some(&2));
        assert_eq!(tree.find(5), Some(&5));
        assert_eq!(tree.find(10), Some(&10));
        assert_eq!(tree.find(3), None);
    }
}
