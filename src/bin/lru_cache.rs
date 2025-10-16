use std::collections::HashMap;
use std::marker::PhantomData;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

type Pair<K, V> = (K, V);

struct Node<T> {
    elem: Pair<T, T>,
    next: Link<T>,
    prev: Link<T>,
}

struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    marker: PhantomData<T>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_tail() {}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, elem: Pair<T, T>) {
        unsafe {
            let new_head = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
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

    pub fn clear(&mut self) {
        while let Some(_) = self.pop_tail() {}
    }

    pub fn pop_tail(&mut self) -> Option<Pair<T, T>> {
        self.tail.map(|node| unsafe {
            let tail = Box::from_raw(node.as_ptr());
            let result = tail.elem;
            self.tail = tail.prev;
            if let Some(new_tail) = self.tail {
                (*new_tail.as_ptr()).next = None;
            } else {
                self.head = None;
            }

            self.len -= 1;
            result
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn move_to_head(&mut self, node: &mut Link<T>) -> Option<&Pair<T, T>> {
        node.map(|node| unsafe {
            // 将node的prev和next的指向进行更新
            (*node.as_ptr()).prev.map(|prev| {
                (*prev.as_ptr()).next = (*node.as_ptr()).next;
            });
            if let Some(next) = (*node.as_ptr()).next {
                (*next.as_ptr()).prev = (*node.as_ptr()).prev;
            } else {
                self.tail = (*node.as_ptr()).prev;
            }
            // 将node移动到head
            self.head.map(|old| {
                (*old.as_ptr()).prev = Some(node);
                (*node.as_ptr()).next = Some(old);
                (*node.as_ptr()).prev = None;
            });
            self.head = Some(node);

            return &(*node.as_ptr()).elem;
        })
    }
}

struct LRUCache {
    capacity: u32,
    // key -> value is the ptr of the node in the list
    pair: HashMap<i32, Link<i32>>,
    list: LinkedList<i32>,
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        self.pair.iter_mut().for_each(|(_, node)| *node = None);
        self.list.clear();
        self.pair.clear();
    }
}

impl LRUCache {
    pub fn new(capacity: u32) -> Self {
        Self {
            capacity,
            pair: HashMap::new(),
            list: LinkedList::new(),
        }
    }

    // O(1)
    // add to head, and pop from tail
    // the head is the most recently used, the tail is the least recently used
    pub fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(node) = self.pair.get_mut(&key) {
            if let Some(elem) = self.list.move_to_head(node) {
                return Some((*elem).1);
            }
        }
        Some(-1)
    }

    // O(1)
    // if the key is in the cache, update the value
    // if the key is not in the cache, add the key-value pair to the cache
    // if the cache is full, remove the least recently used key-value pair
    // add the key-value pair to the cache
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.pair.get_mut(&key) {
            node.map(|node| unsafe {
                (*node.as_ptr()).elem = (key, value);
            });
        } else {
            if self.list.len() >= self.capacity as usize {
                if let Some((rm_key, _)) = self.list.pop_tail() {
                    self.pair.remove(&rm_key);
                }
            }
            self.list.push_front((key, value));
            self.pair.insert(key, Some(self.list.head.unwrap().clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Opt {
        Get(i32),
        Put(i32, i32),
        New(u32),
    }
    struct Case {
        opts: Vec<Opt>,
        expected: Vec<Option<i32>>,
        name: String,
    }

    #[test]
    fn rest_solution() {
        let test_cases = vec![Case {
            opts: vec![
                Opt::New(2),    // 创建容量为2的LRU缓存
                Opt::Put(1, 1), // cache: {1=1}
                Opt::Put(2, 2), // cache: {1=1, 2=2}
                Opt::Get(1),    // 返回1，cache: {2=2, 1=1} (1变为最近使用)
                Opt::Put(3, 3), // 淘汰2，cache: {1=1, 3=3}
                Opt::Get(2),    // 返回-1 (未找到)
                Opt::Put(4, 4), // 淘汰1，cache: {3=3, 4=4}
                Opt::Get(1),    // 返回-1 (未找到)
                Opt::Get(3),    // 返回3，cache: {4=4, 3=3} (3变为最近使用)
                Opt::Get(4),    // 返回4，cache: {3=3, 4=4} (4变为最近使用)
            ],
            // 对应的返回值：[null, null, null, 1, null, -1, null, -1, 3, 4]
            // 转换为Option类型，-1用None表示，数值用Some包装
            expected: vec![
                None,     // New(2) - 构造函数返回None
                None,     // Put(1,1) - put操作返回None
                None,     // Put(2,2) - put操作返回None
                Some(1),  // Get(1) - 返回1
                None,     // Put(3,3) - put操作返回None
                Some(-1), // Get(2) - 返回-1，用None表示
                None,     // Put(4,4) - put操作返回None
                Some(-1), // Get(1) - 返回-1，用None表示
                Some(3),  // Get(3) - 返回3
                Some(4),  // Get(4) - 返回4
            ],
            name: "case1".to_string(),
        }];

        for case in test_cases {
            if let Opt::New(capacity) = case.opts[0] {
                let mut cache = LRUCache::new(capacity);
                let mut result_index = 1; // 跳过第一个New操作

                for opt in case.opts.iter().skip(1) {
                    match *opt {
                        Opt::Get(key) => {
                            let res = cache.get(key);
                            // 将Option<u32>转换为期望的格式：None表示-1，Some(x)表示x
                            let expected = case.expected[result_index];
                            if res.is_none() {
                                // get返回None表示未找到，对应LeetCode的-1
                                assert_eq!(
                                    expected, None,
                                    "Expected not found (-1) for key {}",
                                    key
                                );
                            } else {
                                assert_eq!(
                                    res, expected,
                                    "Expected {:?} for key {}",
                                    expected, key
                                );
                            }
                            result_index += 1;
                        }
                        Opt::Put(key, value) => {
                            cache.put(key, value);
                            // put操作不返回值，但在expected数组中占位
                            result_index += 1;
                        }
                        Opt::New(_) => {
                            panic!("New should not be called in iteration");
                        }
                    }
                }
            }
        }
    }
}
