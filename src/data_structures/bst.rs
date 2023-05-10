// bonary search tree
// bst.rs
#![allow(unused)]

use std::fmt::Debug;
use std::rc::Rc;
use std::cell::{RefCell};



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

        let mut new_root: Option<RcTreeNode<K, V>> = None;

        if self.root.is_none() {
            new_root = Some(Rc::new(RefCell::new(TreeNode::new(_key, _val))));
        } else {
            new_root = Self::insert_helper(self.root.take(), _key, _val);
        }
        if new_root.is_some() {
            self.root = new_root;
            self.size += 1;
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
            if left.is_none() {
                return None;
            }
            node.as_ref().unwrap().borrow_mut().left = left;
        } else if node_key.cmp(&_key).is_lt() {
            let right_child: Option<RcTreeNode<K, V>> = node.as_ref().unwrap().borrow().right.clone();
            let right: Option<RcTreeNode<K, V>> = Self::insert_helper(right_child, _key, _val);
            if right.is_none() {
                return None;
            }
            node.as_ref().unwrap().borrow_mut().right = right;
        } else {
            // If user try to insert a key that is already exist
            return None;
        }
        return node;
    }

    pub fn delete(&mut self, _key: K) {
        if self.root.is_none() {
            return;
        }
        let new_root: Option<RcTreeNode<K, V>> = Self::delete_helper(self.root.take(), _key);
        self.root = new_root;
    }

    fn delete_helper(node: Option<RcTreeNode<K, V>>, _key: K) -> Option<RcTreeNode<K, V>> {
        if node.is_none() {
            return None;
        }
        let key = node.as_ref().unwrap().borrow().key.clone();
        if _key.cmp(&key).is_lt() {
            let left: Option<RcTreeNode<K, V>> = node.as_ref().unwrap().borrow().left.clone();
            let returned_left: Option<RcTreeNode<K, V>> = Self::delete_helper(left, _key);
            node.as_ref().unwrap().borrow_mut().left = returned_left;
        } else if _key.cmp(&key).is_gt() {
            let right: Option<RcTreeNode<K, V>> = node.as_ref().unwrap().borrow().right.clone();
            let returned_right: Option<RcTreeNode<K, V>> = Self::delete_helper(right, _key);
            node.as_ref().unwrap().borrow_mut().right = returned_right;
        } else {
            if node.as_ref().unwrap().borrow().left.is_none() {
                return node.as_ref().unwrap().borrow().right.clone();
            }
            if node.as_ref().unwrap().borrow().right.is_none() {
                return node.as_ref().unwrap().borrow().left.clone();
            }

            let mut cur: Option<RcTreeNode<K, V>> = node.as_ref().unwrap().borrow_mut().left.clone();
            while cur.as_ref().unwrap().borrow().right.is_some() {
                let right: Option<RcTreeNode<K, V>> = cur.as_ref().unwrap().borrow_mut().right.take();
                cur = right;
            }

            node.as_ref().unwrap().borrow_mut().key = cur.as_ref().unwrap().borrow().key.clone();
            let left: Option<RcTreeNode<K, V>> = node.as_ref().unwrap().borrow().left.clone();
            let returned_left: Option<RcTreeNode<K, V>> = Self::delete_helper(left, cur.as_ref().unwrap().borrow().key.clone());
            node.as_ref().unwrap().borrow_mut().left = returned_left;   
        }

        return node;
    }


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