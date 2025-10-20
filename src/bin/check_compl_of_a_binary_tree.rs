// 除了最后一个叶节点
// 层数为 d = ceil(log2(n + 1)) d从0开始
// 最后一层的offset = (2^((dk-1) + 1) - 1) 为第k层的第一个node的索引值。
// 每k层的索引边界[2^k -1, 2^(k+1) - 2], 每一层一共有 2^k node,
// 对于i来说其 父节点，其left and right child index is: [],
// [] i=1, child [3,4], (i=2, child =[5,6]), (i=3, child=[7,8]), (i=4, [9,10])

// 除了最后一层，其他层均为有效值。
// 最后一层连续

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        // 层序遍历，所有值连续
        let mut parent = VecDeque::new();
        parent.push_back(root);
        let mut find_null = false;
        while let Some(node_opt) = parent.pop_front() {
            match node_opt {
                Some(node) => {
                    if find_null {
                        return false;
                    }
                    parent.push_back(node.borrow_mut().left.clone());
                    parent.push_back(node.borrow_mut().right.clone());
                }
                None => {
                    find_null = true;
                }
            }
        }

        return true;
    }
}
