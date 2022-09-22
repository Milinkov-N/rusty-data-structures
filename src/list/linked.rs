use crate::list::persistent::PersistentList;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    pub fn to_persistent(self) -> PersistentList<T> {
        let mut list = PersistentList::new();
        let iter = self.into_iter();

        iter.for_each(|v| {
            list = list.prepend(v);
        });

        list
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();

        while let Some(mut node) = next {
            next = node.next.take();
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
    fn linked_list_new_test() {
        let list: LinkedList<i32> = LinkedList::new();

        assert_eq!(LinkedList { head: None }, list);
    }

    #[test]
    fn linked_list_push_test() {
        let mut list = LinkedList::new();

        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(
            LinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 2,
                        next: Some(Box::new(Node {
                            data: 3,
                            next: None,
                        }))
                    }))
                }))
            },
            list
        );
    }

    #[test]
    fn linked_list_pop_test() {
        let mut list = LinkedList::new();

        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(Some(1), list.pop());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(3), list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn linked_list_peeking_test() {
        let mut list = LinkedList::new();

        assert_eq!(None, list.peek());
        assert_eq!(None, list.peek_mut());

        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(Some(&1), list.peek());
        assert_eq!(Some(&mut 1), list.peek_mut());

        list.peek_mut().map(|v| *v = 0);

        assert_eq!(Some(&0), list.peek());
        assert_eq!(Some(&mut 0), list.peek_mut());
    }

    #[test]
    fn linked_list_into_iter_test() {
        let mut list = LinkedList::new();

        list.push(3);
        list.push(2);
        list.push(1);

        let mut iter = list.into_iter();

        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn linked_list_iter_test() {
        let mut list = LinkedList::new();

        list.push(3);
        list.push(2);
        list.push(1);

        let mut iter = list.iter();

        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn linked_list_iter_mut_test() {
        let mut list = LinkedList::new();

        list.push(3);
        list.push(2);
        list.push(1);

        let mut iter = list.iter_mut();

        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(Some(&mut 2), iter.next());
        assert_eq!(Some(&mut 3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn linked_list_to_persistent_test() {
        let mut list = LinkedList::new();

        list.push(3);
        list.push(2);
        list.push(1);

        let list = list.to_persistent();
        assert_eq!(Some(&3), list.head());

        let list = list.tail();
        assert_eq!(Some(&2), list.head());

        let list = list.tail();
        assert_eq!(Some(&1), list.head());

        let list = list.tail();
        assert_eq!(None, list.head());
    }
}
