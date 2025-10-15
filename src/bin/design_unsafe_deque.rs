use std::marker::PhantomData;
use std::ptr;

pub struct List<T> {
    head: Link<T>, // why link
    tail: Link<T>,
    len: usize,
    _marker: PhantomData<T>,
}

pub type Link<T> = Option<ptr::NonNull<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    // push to tail
    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = ptr::NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                next: None,
                prev: None,
            })));
            if let Some(old) = self.tail {
                (*old.as_ptr()).next = Some(new_tail);
                (*new_tail.as_ptr()).prev = Some(old);
            } else {
                self.head = Some(new_tail);
            }

            self.tail = Some(new_tail);
            self.len += 1;
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new_head = ptr::NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem,
                next: None,
                prev: None,
            })));

            if let Some(old) = self.head {
                (*old.as_ptr()).prev = Some(new_head);
                (*new_head.as_ptr()).next = Some(old);
            } else {
                self.tail = Some(new_head);
            }
            self.head = Some(new_head);
            self.len += 1;
        }
    }

    // pop from head
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            // if self.head.is_none() {
            //     return None;
            // }
            // // release the head by box
            // let head = Box::from_raw(self.head.unwrap().as_ptr());
            // // 原head修改next为空，新head修改prev为空，并修改头指针
            // self.head = head.next;
            // if self.head.is_none() {
            //     self.tail = None;
            // } else {
            //     (*self.head.unwrap().as_ptr()).prev = None;
            // }
            // self.len -= 1;
            // Some(head.elem)

            // ========== other way ==========
            self.head.map(|node| {
                let head = Box::from_raw(node.as_ptr());
                let result = head.elem;

                self.head = head.next;
                if let Some(new_head) = self.head {
                    (*new_head.as_ptr()).prev = None;
                } else {
                    self.tail = None;
                }
                self.len -= 1;
                result
            })
        }
    }

    // pop from tail
    pub fn pop_tail(&mut self) -> Option<T> {
        unsafe {
            if self.tail.is_none() {
                return None;
            }
            let tail = Box::from_raw(self.tail.unwrap().as_ptr());

            self.tail = tail.prev;
            if let Some(new_tail) = self.tail {
                (*new_tail.as_ptr()).next = None;
            } else {
                self.head = None;
            }

            self.len -= 1;
            Some(tail.elem)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        // unsafe { self.head.map(|node| &(*node.as_ptr()).elem) }
        // using ? operator

        // keep in mind, all operations should at the start of our methods or at the end of our methods
        unsafe { Some(&(*self.head?.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.map(|node| &(*node.as_ptr()).elem) }
    }
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.map(|node| &mut (*node.as_ptr()).elem) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

// iterator for List
pub struct Iter<'a, T> {
    prev: Link<T>,
    next: Link<T>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    prev: Link<T>,
    next: Link<T>,
    len: usize,
    _marker: PhantomData<&'a mut T>,
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            prev: self.head,
            next: self.tail,
            len: self.len,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            prev: self.head,
            next: self.tail,
            len: self.len,
            _marker: PhantomData,
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

// noting why this work
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.prev.map(|node| unsafe {
                let result = &(*node.as_ptr()).elem;
                self.prev = (*node.as_ptr()).next;
                self.len -= 1;
                result
            })
        } else {
            None
        }
    }

    // size for precheck
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.next.map(|node| unsafe {
                let result = &(*node.as_ptr()).elem;
                self.next = (*node.as_ptr()).prev;
                self.len -= 1;
                result
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.prev.map(|node| unsafe {
                let result = &mut (*node.as_ptr()).elem;
                self.prev = (*node.as_ptr()).next;
                self.len -= 1;
                result
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.next.map(|node| unsafe {
                let result = &mut (*node.as_ptr()).elem;
                self.next = (*node.as_ptr()).prev;
                self.len -= 1;
                result
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len()))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_tail()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len()
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
