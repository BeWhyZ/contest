use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
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

// common traits for linked list
impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        let mut list = Self::new();
        for item in self.iter() {
            list.push(item.clone());
        }
        list
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for List<T> {}

impl<T: PartialOrd> PartialOrd for List<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for List<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for List<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self.iter() {
            item.hash(state);
        }
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


    fn generate_test() -> List<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn list_from<T: Clone>(v: &[T]) -> List<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    #[test]
    fn test_basic_front() {
        let mut list = List::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_basic() {
        let mut m = List::new();
        assert_eq!(m.pop(), None);
        assert_eq!(m.pop_tail(), None);
        assert_eq!(m.pop(), None);
        m.push_front(1);
        assert_eq!(m.pop(), Some(1));
        m.push(2);
        m.push(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop(), Some(2));
        assert_eq!(m.pop(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop(), None);
        m.push(1);
        m.push(3);
        m.push(5);
        m.push(7);
        assert_eq!(m.pop(), Some(1));

        let mut n = List::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop(), Some(0));
        assert_eq!(n.pop(), Some(1));
    }

    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as i32, *elt);
        }
        let mut n = List::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = List::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            assert_eq!(6 - i as i32, *elt);
        }
        let mut n = List::new();
        assert_eq!(n.iter().rev().next(), None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = List::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = List::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: List<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        let nan = 0.0f64 / 0.0;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!(!(n < m));
        assert!(!(n > m));
        assert!(!(n <= m));
        assert!(!(n >= m));

        let n = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert!(!(n < one));
        assert!(!(n > one));
        assert!(!(n <= one));
        assert!(!(n >= one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert!(!(u < v));
        assert!(!(u > v));
        assert!(!(u <= v));
        assert!(!(u >= v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert!(!(s < t));
        assert!(s > one);
        assert!(!(s <= one));
        assert!(s >= one);
    }

    #[test]
    fn test_debug() {
        let list: List<i32> = (0..10).collect();
        assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: List<&str> = vec!["just", "one", "test", "more"]
            .iter()
            .copied()
            .collect();
        assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: List<i32> = (0..10).collect();
        let list2: List<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }
}
