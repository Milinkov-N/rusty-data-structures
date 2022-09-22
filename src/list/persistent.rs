use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersistentList<T> {
    head: Link<T>,
}

impl<T> PersistentList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn prepend(&self, data: T) -> PersistentList<T> {
        PersistentList {
            head: Some(Rc::new(Node {
                data,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> PersistentList<T> {
        PersistentList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

impl<T> Drop for PersistentList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(node) = head {
            match Rc::try_unwrap(node) {
                Err(_) => break,
                Ok(mut node) => head = node.next.take(),
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persistent_list_general_test() {
        let list = PersistentList::new();
        assert_eq!(None, list.head());

        let list = list.prepend(3).prepend(2).prepend(1);
        assert_eq!(Some(&1), list.head());

        let list = list.tail();
        assert_eq!(Some(&2), list.head());

        let list = list.tail();
        assert_eq!(Some(&3), list.head());

        let list = list.tail();
        assert_eq!(None, list.head());

        let list = list.tail();
        assert_eq!(None, list.head());
    }

    #[test]
    fn persistent_list_iter_test() {
        let list = PersistentList::new().prepend(3).prepend(2).prepend(1);

        let mut iter = list.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
    }
}
