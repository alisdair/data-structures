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

    pub fn walk(&self) -> Vec<&T> {
        let items = Vec::new();
        match self.root {
            None => items,
            Some(ref x) => x.walk(&items),
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
            x if item < *x => match self.left {
                None => self.left = Some(Box::new(Node::new(item))),
                Some(ref mut x) => x.insert(item),
            },
            x if item > *x => match self.right {
                None => self.right = Some(Box::new(Node::new(item))),
                Some(ref mut x) => x.insert(item),
            },
            _ => (),
        }
    }

    fn find(&self, item: T) -> Option<&T> {
        match &self.item {
            x if *x == item => Some(&x),
            x if item < *x => match self.left {
                None => None,
                Some(ref x) => x.find(item),
            },
            _ => match self.right {
                None => None,
                Some(ref x) => x.find(item),
            },
        }
    }

    fn walk(&self, items: &Vec<&T>) -> Vec<&T> {
        let children_left = match self.left {
            None => Vec::new(),
            Some(ref x) => x.walk(&items),
        };
        let children_right = match self.right {
            None => Vec::new(),
            Some(ref x) => x.walk(&items),
        };
        let mut new_items = Vec::new();
        new_items.extend(children_left.iter().cloned());
        new_items.push(&self.item);
        new_items.extend(children_right.iter().cloned());
        new_items
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

    #[test]
    fn test_walk() {
        let mut tree = BinarySearchTree::<i32>::new();
        
        let empty: Vec<&i32> = Vec::new();
        assert_eq!(tree.walk(), empty);

        tree.insert(6);
        tree.insert(4);
        tree.insert(10);
        tree.insert(2);
        tree.insert(5);
        tree.insert(1);
        tree.insert(3);
        tree.insert(9);
        tree.insert(11);

        assert_eq!(vec![&1,&2,&3,&4,&5,&6,&9,&10,&11], tree.walk());

        tree.insert(5);

        assert_eq!(vec![&1,&2,&3,&4,&5,&6,&9,&10,&11], tree.walk());
    }
}
