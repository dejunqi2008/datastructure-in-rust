// bonary search tree
// bst.rs

use std::cmp::Ordering;
use std::ops::Deref;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};

type RcTreeNode<K,V> = Rc<RefCell<TreeNode<K, V>>>;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode<K, V> {
    key: K,
    val: V,
    left: Option<RcTreeNode<K,V>>,
    right: Option<RcTreeNode<K, V>>
}

impl<K, V> TreeNode<K, V>
    where K: Clone + Ord + Debug, V: Clone + Debug {
    
    fn new(k: K, v: V) -> Self {
        Self {
            key: k,
            val: v,
            left: None,
            right: None,
        }
    }

    fn set(&mut self, _key: K, _val: V) {
        self.key = _key;
        self.val = _val;
    }

    fn add_left_child(&mut self, left_child: Option<RcTreeNode<K, V>>) {
        self.left = left_child;
    }

    fn add_right_child(&mut self, right_child: Option<RcTreeNode<K, V>>) {
        self.right = right_child;
    }
}


pub struct BST<K, V>  {
    size: u32,
    root: Option<RcTreeNode<K, V>>,
} 

impl<K,V> BST<K, V> where K: Clone + Ord + Debug, V: Clone + Debug {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn size(&self) -> u32 {
        return self.size;
    }

    pub fn insert(&mut self, _key: K, _val: V) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(TreeNode::new(_key, _val))));
            
        } else {
            self.root = Self::insert_helper(self.root.take(), _key, _val);
        }
    }

    pub fn in_order(&mut self) -> Vec<(K, V)> {
        let mut res: Vec<(K, V)> = vec![];
        Self::in_order_helper(&self.root, &mut res);
        return res;
    }

    fn in_order_helper(node: &Option<RcTreeNode<K, V>>, res: &mut Vec<(K, V)>) {
        if node.is_none() {
            return;
        }
        let left: &Option<RcTreeNode<K,V>> = &node.as_ref().unwrap().borrow().left;
        let right: &Option<RcTreeNode<K, V>> = &node.as_ref().unwrap().borrow().right;
        let key = node.as_ref().unwrap().borrow().key.clone();
        let val = node.as_ref().unwrap().borrow().val.clone();
        Self::in_order_helper(left, res);
        res.push((key, val));
        Self::in_order_helper(right, res);
    }

    fn insert_helper(node: Option<RcTreeNode<K, V>>, _key: K, _val: V) -> Option<RcTreeNode<K, V>> {
        if node.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(_key, _val))))
        }

        let node_key = node.as_ref().unwrap().borrow().key.clone();
        if node_key.cmp(&_key).is_gt() {
            let left_child: Option<RcTreeNode<K, V>> = node.as_ref().unwrap().borrow().left.clone();
            let left: Option<RcTreeNode<K, V>> = Self::insert_helper(left_child, _key, _val);
            node.as_ref().unwrap().borrow_mut().left = left;
        } else if node_key.cmp(&_key).is_lt() {
            let right_child: Option<RcTreeNode<K, V>> = node.as_ref().unwrap().borrow().right.clone();
            let right: Option<RcTreeNode<K, V>> = Self::insert_helper(right_child, _key, _val);
            node.as_ref().unwrap().borrow_mut().right = right;
        }
        return node;
    }

    // TODO: delete node


}


pub struct Solution;

impl  Solution {
    pub fn test() {
        let nums = vec![8, 2, 1, 4, 3, 5, 15, 9, 17];
        let mut tree: BST<i32, i32> = BST::new();
        for num in nums.into_iter() {
            tree.insert(num, num);
        }

        let inroder_vals = tree.in_order();
        println!("{:?}", inroder_vals);
    }
}