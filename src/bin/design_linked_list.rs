/*
Implement the MyLinkedList class:

MyLinkedList() Initializes the MyLinkedList object.
int get(int index) Get the value of the indexth node in the linked list. If the index is invalid, return -1.
void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
void addAtTail(int val) Append a node of value val as the last element of the linked list.
void addAtIndex(int index, int val) Add a node of value val before the indexth node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
void deleteAtIndex(int index) Delete the indexth node in the linked list, if the index is valid.

Example 1:

Input
["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
[[], [1], [3], [1, 2], [1], [1], [1]]
Output
[null, null, null, null, 2, null, 3]

Explanation
MyLinkedList myLinkedList = new MyLinkedList();
myLinkedList.addAtHead(1);
myLinkedList.addAtTail(3);
myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
myLinkedList.get(1);              // return 2
myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
myLinkedList.get(1);              // return 3


Constraints:

0 <= index, val <= 1000
Please do not use the built-in LinkedList library.
At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and deleteAtIndex.
*/

use std::{cell::RefCell, rc::Rc};

type INode = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: INode,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val: val,
            next: None,
        }
    }
}

enum ListNode {
    Head(Node),
    Tail(Node),
}

#[derive(Debug)]
struct MyLinkedList {
    head: INode,
    tail: INode,
    size: usize,
}

impl MyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index >= self.size as i32 || index < 0 {
            return -1;
        }
        let mut node = self.head.clone();
        for _ in 0..index {
            node = node.as_ref().and_then(|rc| rc.borrow().next.clone());
        }
        node.as_ref()
            .and_then(|rc| Some(rc.borrow().val.clone()))
            .unwrap()
    }

    fn add_at_head(&mut self, val: i32) {
        // 添加头部，更改size以及head
        let new_node = Rc::new(RefCell::new(Node::new(val)));

        if let Some(head_node) = self.head.take() {
            new_node.borrow_mut().next = Some(head_node);
            self.head = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }

        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }

        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(next) => {
                    self.head = Some(next);
                }
                None => {
                    self.tail.take();
                }
            }
            self.size -= 1;
            println!("strong_count: {}", Rc::strong_count(&old_head));
            Rc::try_unwrap(old_head)
                .ok()
                .unwrap_or_else(|| panic!("Failed to unwrap old_head"))
                .into_inner()
                .val
        })
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        if let Some(tail_node) = self.tail.take() {
            tail_node.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.size += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.size as i32 || index < 0 {
            return;
        }

        // 0表示在第一个头部插入
        if index == 0 {
            self.add_at_head(val);
            return;
        } else if index == self.size as i32 {
            self.add_at_tail(val);
            return;
        }

        let new_node = Rc::new(RefCell::new(Node::new(val)));

        self.size += 1;
        let mut node = self.head.clone();
        if index - 1 > 0 {
            for _ in 0..(index - 1) {
                node = node.as_ref().and_then(|rc| rc.borrow().next.clone());
            }
        }
        if let Some(node) = node {
            // 将node的next赋值给new_node的next, 需要将
            new_node.borrow_mut().next = node.borrow().next.clone(); // +1
            node.borrow_mut().next = Some(new_node.clone()); // -1
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index >= self.size as i32 || index < 0 {
            return;
        }
        if index == 0 {
            self.pop_front();
            return;
        }
        let mut node = self.head.clone(); // 1
        if index - 1 > 0 {
            for _ in 0..(index - 1) {
                node = node.as_ref().and_then(|rc| rc.borrow().next.clone());
            }
        }
        // node的next为删除的节点，需要将next的next赋值给node的next
        if let Some(ref node_rc) = node {
            // First, get the next pointer (to be deleted)
            let next_opt = node_rc.borrow().next.clone();
            if let Some(ref next_rc) = next_opt {
                self.size -= 1;

                node_rc.borrow_mut().next = next_rc.borrow().next.clone();
            }
        }

        //
        if index == self.size as i32 {
            self.tail = node;
        }
    }
}

// impl Drop for MyLinkedList {
//     fn drop(&mut self) {
//         while self.pop_front().is_some() {}
//     }
// }

// todo 实现drop来释放资源

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_design_linked_list() {
        let mut my_linked_list = MyLinkedList::new();
        // MyLinkedList()
        my_linked_list.add_at_head(1);
        println!("{:?}", my_linked_list);
        // addAtHead(1)
        my_linked_list.add_at_tail(3);
        println!("{:?}", my_linked_list);
        // addAtTail(3)
        my_linked_list.add_at_index(1, 2);
        println!("{:?}", my_linked_list);
        // addAtIndex(1, 2)
        let res1 = my_linked_list.get(1);
        assert_eq!(res1, 2, "get(1) should return 2");
        my_linked_list.delete_at_index(1);
        // deleteAtIndex(1)
        let res2 = my_linked_list.get(1);
        assert_eq!(res2, 3, "get(1) should return 3 after deletion");
    }

    // ["MyLinkedList","addAtHead","addAtHead","addAtHead","addAtIndex","deleteAtIndex","addAtHead","addAtTail","get","addAtHead","addAtIndex","addAtHead"]
    // [[],[7],[2],[1],[3,0],[2],[6],[4],[4],[4],[5,0],[6]]
    // output : [null,null,null,null,null,null,null,null,4,null,null,null]
    #[test]
    fn test_design_linked_list_case2() {
        let mut my_linked_list = MyLinkedList::new();
        // [null]
        my_linked_list.add_at_head(7);
        // [null, null]
        my_linked_list.add_at_head(2);
        // [null, null, null]
        my_linked_list.add_at_head(1);
        // [null, null, null, null]
        my_linked_list.add_at_index(3, 0); // 1，2，7，0
        println!("{:?}", my_linked_list);
        my_linked_list.delete_at_index(2); // 1，2，0
        println!("{:?}", my_linked_list);
        my_linked_list.add_at_head(6); // 6，1，2，0
        println!("{:?}", my_linked_list);
        my_linked_list.add_at_tail(4); // 6，1，2，0，4
        println!("{:?}", my_linked_list);
        // [null, null, null, null, null, null, null, null]
        let res = my_linked_list.get(4);
        assert_eq!(res, 4, "get(4) should return 4");
        // [null, null, null, null, null, null, null, null, 4]
        my_linked_list.add_at_head(4);
        // [null, null, null, null, null, null, null, null, 4, null]
        my_linked_list.add_at_index(5, 0);
        // [null, null, null, null, null, null, null, null, 4, null, null]
        my_linked_list.add_at_head(6);
        // [null, null, null, null, null, null, null, null, 4, null, null, null]
    }

    // ["MyLinkedList","addAtHead","addAtIndex","addAtTail","addAtHead","addAtIndex","addAtTail","addAtTail","addAtIndex","deleteAtIndex","deleteAtIndex","addAtTail"]
    // [[],[0],[1,4],[8],[5],[4,3],[0],[5],[6,3],[7],[5],[4]]
    #[test]
    fn test_design_linked_list_case3() {
        {
            let mut my_linked_list = MyLinkedList::new();
            // [null]
            my_linked_list.add_at_head(0); // [0]
            my_linked_list.add_at_index(1, 4); // [0,4]
            println!("{:?}", my_linked_list);
            my_linked_list.add_at_tail(8); // [0,4,8]
            my_linked_list.add_at_head(5); // [5,0,4,8]
            my_linked_list.add_at_index(4, 3); // [5,0,4,8,3]
            my_linked_list.add_at_tail(0); // [5,0,4,8,3,0]
            println!("{:?}", my_linked_list);
            my_linked_list.add_at_tail(5); // [5,0,4,8,3,0,5]
            my_linked_list.add_at_index(6, 3); // [5,0,4,8,3,0,3,5]
            my_linked_list.delete_at_index(7); // [5,0,4,8,3,0,3]
            println!("{:?}", my_linked_list);
            my_linked_list.delete_at_index(5); // [5,0,4,8,3,3]
            println!("{:?}", my_linked_list);
            my_linked_list.add_at_tail(4); // [5,0,4,8,3,3,4]
            println!("{:?}", my_linked_list);
            let mut vals = Vec::new();
            let mut node = my_linked_list.head.clone();
            while let Some(rc) = node {
                vals.push(rc.borrow().val);
                node = rc.borrow().next.clone();
            }
            assert_eq!(
                vals,
                vec![5, 0, 4, 8, 3, 3, 4],
                "链表内容应为 [5,0,4,8,3,3,4]"
            );
            drop(my_linked_list);
        }
        println!("test_design_linked_list_case3 passed");
    }

    #[test]
    fn test_linked_list_edge_cases() {
        let mut list = MyLinkedList::new();
        // 空链表get
        assert_eq!(list.get(0), -1);
        assert_eq!(list.get(-1), -1);
        // 空链表删除
        list.delete_at_index(0); // 不应panic
        list.delete_at_index(-1); // 不应panic
                                  // 空链表插入尾部
        list.add_at_tail(10);
        assert_eq!(list.get(0), 10);
        // 删除唯一元素
        list.delete_at_index(0);
        assert_eq!(list.get(0), -1);
        // 头部插入多个
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3); // 3,2,1
        assert_eq!(list.get(0), 3);
        assert_eq!(list.get(2), 1);
        // 尾部插入
        list.add_at_tail(4); // 3,2,1,4
        assert_eq!(list.get(3), 4);
        // 越界插入
        list.add_at_index(10, 99); // 无效
        assert_eq!(list.get(4), -1);
        // 在头部插入
        list.add_at_index(0, 9); // 9,3,2,1,4
        assert_eq!(list.get(0), 9);
        // 在尾部插入
        list.add_at_index(list.size as i32, 8); // 9,3,2,1,4,8
        assert_eq!(list.get(list.size as i32 - 1), 8);
        // 删除头
        list.delete_at_index(0); // 3,2,1,4,8
        assert_eq!(list.get(0), 3);
        // 删除尾
        list.delete_at_index(list.size as i32 - 1); // 3,2,1,4
        assert_eq!(list.get(list.size as i32 - 1), 4);
        // 删除中间
        list.delete_at_index(1); // 3,1,4
        assert_eq!(list.get(1), 1);
        // 连续删除
        list.delete_at_index(0); // 1,4
        list.delete_at_index(0); // 4
        list.delete_at_index(0); // 空
        assert_eq!(list.get(0), -1);
        // 再次插入
        list.add_at_head(42);
        assert_eq!(list.get(0), 42);
    }

    //["MyLinkedList","addAtHead","get","addAtHead","addAtHead","deleteAtIndex","addAtHead","get","get","get","addAtHead","deleteAtIndex"]
    //[[],[4],[1],[1],[5],[3],[7],[3],[3],[3],[1],[4]]
    //[null,null,-1,null,null,null,null,4,4,4,null,null]
    #[test]
    fn test_design_linked_list_case4() {
        let mut my_linked_list = MyLinkedList::new();
        // [null]
        my_linked_list.add_at_head(4); // [4]
                                       // [null, null]
        assert_eq!(my_linked_list.get(1), -1); // [null, null, -1]
        my_linked_list.add_at_head(1); // [1,4]
        my_linked_list.add_at_head(5); // [5,1,4]
        my_linked_list.delete_at_index(3); // 删除越界，无变化
        println!("{:?}", my_linked_list);
        my_linked_list.add_at_head(7); // [7,5,1,4]
        assert_eq!(my_linked_list.get(3), 4); // [null, null, -1, null, null, null, null, 4]
        assert_eq!(my_linked_list.get(3), 4); // [null, null, -1, null, null, null, null, 4, 4]
        assert_eq!(my_linked_list.get(3), 4); // [null, null, -1, null, null, null, null, 4, 4, 4]
        my_linked_list.add_at_head(1); // [1,7,5,1,4]
        my_linked_list.delete_at_index(4); // 删除最后一个 4 -> [1,7,5,1]
                                           // 验证链表内容
        let mut vals = Vec::new();
        let mut node = my_linked_list.head.clone();
        while let Some(rc) = node {
            vals.push(rc.borrow().val);
            node = rc.borrow().next.clone();
        }
        assert_eq!(vals, vec![1, 7, 5, 1], "链表内容应为 [1,7,5,1]");
    }

    // ["MyLinkedList","addAtTail","addAtTail","get"]
    //[[],[1],[3],[1]]
    //[null,null,null,3]
    #[test]
    fn test_design_linked_list_case5() {
        let mut my_linked_list = MyLinkedList::new();
        // [null]
        my_linked_list.add_at_tail(1); // [1]
        my_linked_list.add_at_tail(3); // [1,3]
        assert_eq!(my_linked_list.get(1), 3); // [null, null, null, 3]
    }
}
