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
    pub fn find(root: Rc<RefCell<TreeNode>>, sum: i32,target:i32) -> bool {
        match (&root.borrow().left,&root.borrow().right){
            (Some(left), Some(right)) => {
                Self::find(Rc::clone(&left),sum + root.borrow().val,target) 
                || Self::find(Rc::clone(&right),sum + root.borrow().val,target) 
            },
            (None,None)=>{
                sum+root.borrow().val==target
            }
            (Some(left),None) => {
                Self::find(Rc::clone(&left),sum + root.borrow().val,target) 
            }
            (None,Some(right))=>{
                Self::find(Rc::clone(&right),sum + root.borrow().val,target) 
            }
        }
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match (root){
            None=>false,
            Some(node)=>{
                Self::find(node,0,sum)
            }
        }
    }
}