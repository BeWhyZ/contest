use std::marker::PhantomData;
use std::ptr;

pub struct List<T> {
    head: Link<T>, // why link

    tail: *mut Node<T>,
}

pub type Link<T> = *mut Node<T>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    // push to tail
    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));

            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
                (*new_tail).prev = self.tail;
            }
            self.tail = new_tail;
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new_head = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));

            if self.head.is_null() {
                self.tail = new_head;
            } else {
                (*new_head).next = self.head;
                (*self.head).prev = new_head;
            }

            self.head = new_head;
        }
    }

    // pop from head
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                return None;
            }
            // release the head by box
            let head = Box::from_raw(self.head);
            // 原head修改next为空，新head修改prev为空，并修改头指针
            self.head = head.next;
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            } else {
                (*self.head).prev = ptr::null_mut();
            }
            Some(head.elem)
        }
    }

    // pop from tail
    pub fn pop_tail(&mut self) -> Option<T> {
        unsafe {
            if self.tail.is_null() {
                return None;
            }

            let tail = Box::from_raw(self.tail);
            self.tail = tail.prev;
            (*self.tail).next = ptr::null_mut();
            if self.tail.is_null() {
                self.head = ptr::null_mut();
            }
            Some(tail.elem)
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: *mut Node<T>,
}

pub struct IterMut<'a, T> {
    next: *mut Node<T>,
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe { self.0.pop() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a PhantomData<T>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.next.is_null() {
                return None;
            }

            let node = &*self.next;
            self.next = node.next;
            Some(&node.elem)
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.next.is_null() {
                return None;
            }

            let node = &mut *self.next;
            self.next = node.next;
            Some(&mut node.elem)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter().map(|x| *x + 1);
        assert_eq!(iter.next(), Some(2));

        let mut iter_mut = list.iter_mut().map(|x| (*x) * 2);
        assert_eq!(iter_mut.next(), Some(2));

        let mut into_iter = list.into_iter().map(|x| x * 3);
        assert_eq!(into_iter.next(), Some(3));
    }
}
