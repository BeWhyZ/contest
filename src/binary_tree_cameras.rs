use std::{ cell::RefCell, rc::Rc};


pub struct TreeNode {
    pub val: i64,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}


impl TreeNode {
    pub fn new(val: i64) -> Self {
        Self { val: val, left:None, right: None }
    }
}

// build tree from vec, and return the root node
pub fn build_tree(arr: &Vec<Option<i64>>) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() || arr[0].is_none(){
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(arr[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());

    let mut i:usize = 1;
    while !queue.is_empty() && i < arr.len() {
        if let Some(node) = queue.pop_front() {
            if i < arr.len() {
                if let Some(left_val) = arr[i] {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(left_val)));
                    node.borrow_mut().left = Some(left_node.clone());
                    queue.push_back(left_node);
                }
                i += 1;
            }
            if i < arr.len(){
                if let Some(right_val) = arr[i] {
                    let rigth_node = Rc::new(RefCell::new(TreeNode::new(right_val)));
                    node.borrow_mut().right = Some(rigth_node.clone());
                    queue.push_back(rigth_node);
                }
                i += 1;
            }
        }
    }

    Some(root)

}

pub struct Solution {

}

impl Solution {
    pub fn min_camera_cover(arr: Vec<Option<i64>>) -> i64 {
        if arr.is_empty() || arr[0].is_none(){
            return 0;
        }

        let root = build_tree(&arr);

        // 三种状态
        let res = Self::solve(root);
        return res[1].min(res[2])
    }

    pub fn solve(node: Option<Rc<RefCell<TreeNode>>>) -> [i64;3] {
        // 自顶向下
        // 所有的子树都被观测，但是本节点没有
        // 所有的子树以及当前节点都被观测，但是本届点没有camera
        // 所有的子树以及当前节点被观测，且本节点有camera

        if node.is_none(){
            return [0,0,std::i64::MAX/2]
        }
        let left_res = Self::solve(node.as_ref().unwrap().borrow().left.clone());
        let right_res = Self::solve(node.as_ref().unwrap().borrow().right.clone());

        let dp0 = left_res[1] + right_res[1];
        // let dp1 =  *(&left_res[1..].iter().min().unwrap()) + *(&right_res[1..].iter().min().unwrap());
        let dp1 =  (right_res[2] + left_res[1].min(left_res[2])).min(left_res[2] + right_res[1].min(right_res[2]));
        let dp2 = 1 + left_res.iter().min().unwrap() + right_res.iter().min().unwrap();
        [dp0, dp1, dp2]
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works(){
        let root = vec![Some(0),Some(0),None,Some(0),Some(0)];
        assert_eq!(Solution::min_camera_cover(root), 1);

        let root = vec![Some(0),Some(0),None,Some(0),None,Some(0),None,None,Some(0)];
        assert_eq!(Solution::min_camera_cover(root), 2);

    }

}


