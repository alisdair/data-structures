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
            None => self.root = Some(box Node::new(item)),
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
    fn new(item: T) -> Self {
        Node {
            item: item,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, item: T) {
        match &self.item {
            x if item <= *x => match self.left {
                None => self.left = Some(box Node::new(item)),
                Some(ref mut x) => x.insert(item),
            },
            _ => match self.right {
                None => self.right = Some(box Node::new(item)),
                Some(ref mut x) => x.insert(item),
            },
        }
    }

    fn find(&self, item: T) -> Option<&T> {
        match &self.item {
            x if *x == item => Some(&x),
            x if item <= *x => match self.left {
                None => None,
                Some(ref x) => x.find(item),
            },
            _ => match self.right {
                None => None,
                Some(ref x) => x.find(item),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn test_basics() {
        let mut tree = BinarySearchTree::<i32>::new();

        assert_eq!(tree.find(1), None);

        tree.insert(5);
        tree.insert(1);
        tree.insert(10);
        tree.insert(2);

        assert_eq!(tree.find(1),  Some(&1));
        assert_eq!(tree.find(2),  Some(&2));
        assert_eq!(tree.find(5),  Some(&5));
        assert_eq!(tree.find(10), Some(&10));
        assert_eq!(tree.find(3),  None);
    }
}
