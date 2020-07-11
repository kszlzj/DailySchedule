// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root{
            let node = node.borrow();
            let now ={
                if let Some(left) = node.left.clone(){
                    let left=left.borrow();
                    if left.left==None&&left.right==None{
                        left.val
                    }else{
                        0
                    }
                } else{
                    0
                }
            };
            now + Self::sum_of_left_leaves(node.left.clone())+Self::sum_of_left_leaves(node.right.clone())
        }else{
            0
        }
    }
}