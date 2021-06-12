//! 一棵二叉搜索树是一棵满足下面条件的二叉树
//!   1. 所有左侧分支的值都小于本节点的值
//!   2. 本节点的值小于所有右侧分支的值

use crate::tree::binary::{Node, Tree};
use std::cmp::Ordering;
use std::ptr::NonNull;

pub trait BSTree<T>
where
    T: std::cmp::PartialOrd,
{
    /// return true: insert success, false: not insert, exist k
    fn insert(&mut self, element: T) -> bool;
    fn delete(&mut self, element: T) -> bool;
    /// return node index
    fn find(&self, element: T) -> Option<NonNull<Node<T>>>;
    fn min(&self) -> Option<NonNull<Node<T>>>;
    fn max(&self) -> Option<NonNull<Node<T>>>;
    /// 查找后继元素
    fn succ(&self, element: T) -> Option<NonNull<Node<T>>>;
    /// 寻找前驱元素
    fn pred(&self, elementx: T) -> Option<NonNull<Node<T>>>;
}

impl<T> BSTree<T> for Tree<T>
where
    T: std::cmp::PartialOrd + Copy,
{
    fn insert(&mut self, element: T) -> bool {
        unsafe { insert(self, element, None, self.root) }
    }

    fn delete(&mut self, element: T) -> bool {
        unsafe { delete(element, self.root) }
    }

    fn find(&self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { find(element, self.root) }
    }

    fn min(&self) -> Option<NonNull<Node<T>>> {
        unsafe { find_min(self.root) }
    }

    fn max(&self) -> Option<NonNull<Node<T>>> {
        unsafe { find_max(self.root) }
    }

    fn succ(&self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { succ(self.root, element) }
    }

    fn pred(&self, element: T) -> Option<NonNull<Node<T>>> {
        unsafe { pred(self.root, element) }
    }
}

unsafe fn insert<T>(
    tree: &mut Tree<T>,
    element: T,
    parent: Option<NonNull<Node<T>>>,
    node: Option<NonNull<Node<T>>>,
) -> bool
where
    T: std::cmp::PartialOrd,
{
    match (parent, node) {
        //empty tree
        (None, None) => {
            let node = Node::from_element(element);
            tree.root = Some(node);
            true
        }
        (_, Some(node)) => match (*node.as_ptr()).element.partial_cmp(&element) {
            Some(Ordering::Less) => {
                let r = (*node.as_ptr()).right;
                insert(tree, element, Some(node), r)
            }
            Some(Ordering::Greater) => {
                let l = (*node.as_ptr()).left;
                insert(tree, element, Some(node), l)
            }
            _ => false,
        },
        (Some(node), None) => match (*node.as_ptr()).element.partial_cmp(&element) {
            Some(Ordering::Less) => {
                let child = Node::new_leaf(element, Some(node));
                (*node.as_ptr()).right = Some(child);
                true
            }
            Some(Ordering::Greater) => {
                let child = Node::new_leaf(element, Some(node));
                (*node.as_ptr()).left = Some(child);
                true
            }
            _ => false,
        },
    }
}

unsafe fn find<T>(element: T, node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(
        |node| match (*node.as_ptr()).element.partial_cmp(&element) {
            Some(Ordering::Less) => find(element, (*node.as_ptr()).right),
            Some(Ordering::Greater) => find(element, (*node.as_ptr()).left),
            Some(Ordering::Equal) => Some(node),
            None => None,
        },
    )
}

unsafe fn find_min<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| {
        (*node.as_ptr())
            .left
            .map_or(Some(node), |l| find_min(Some(l)))
    })
}

unsafe fn find_max<T>(node: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd,
{
    node.and_then(|node| {
        (*node.as_ptr())
            .right
            .map_or(Some(node), |r| find_max(Some(r)))
    })
}

unsafe fn succ<T>(node: Option<NonNull<Node<T>>>, mut element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    find(element, node).and_then(|node| {
        match (*node.as_ptr()).right {
            //右分支的最小值
            Some(r) => find_min(Some(r)),
            None => {
                //右分支为空，向上找
                let mut p = (*node.as_ptr()).parent;
                loop {
                    match Node::right_node(p) {
                        Some(r) if (*r.as_ptr()).element == element => {
                            let p_node = p.unwrap();
                            element = (*p_node.as_ptr()).element;
                            p = (*p_node.as_ptr()).parent;
                        }
                        _ => return p,
                    }
                }
            }
        }
    })
}

unsafe fn pred<T>(node: Option<NonNull<Node<T>>>, mut element: T) -> Option<NonNull<Node<T>>>
where
    T: std::cmp::PartialOrd + Copy,
{
    find(element, node).and_then(|node| {
        match (*node.as_ptr()).left {
            //左分支的最大值
            Some(l) => find_max(Some(l)),
            None => {
                //左分支为空，向上找
                let mut p = (*node.as_ptr()).parent;
                loop {
                    match Node::left_node(p) {
                        Some(l) if (*l.as_ptr()).element == element => {
                            let p_node = p.unwrap();
                            element = (*p_node.as_ptr()).element;
                            p = (*p_node.as_ptr()).parent;
                        }
                        _ => return p,
                    }
                }
            }
        }
    })
}

/// 从二叉搜索树中删除节点 x 的方法如下:
///   如果 x 没有子节点，或者只有一个孩子，直接将 x“切下”;
///   否则，x 有两个孩子，我们用其右子树中的最小值替换掉 x，然后将右子树中的这一最小值“切掉”。
///
/// idx, 起始node
unsafe fn delete<T>(element: T, node: Option<NonNull<Node<T>>>) -> bool
where
    T: Copy + std::cmp::PartialOrd,
{
    find(element, node).map_or(false, |node| {
        match Node::children_count(node) {
            0 => {
                let parent = (*node.as_ptr()).parent.unwrap();
                if (*parent.as_ptr()).left == Some(node) {
                    (*parent.as_ptr()).left = None;
                } else if (*parent.as_ptr()).right == Some(node) {
                    (*parent.as_ptr()).right = None;
                }

                Node::release(node);
            }
            1 => {
                //backup node child
                let node_child = if (*node.as_ptr()).left.is_some() {
                    (*node.as_ptr()).left
                } else {
                    (*node.as_ptr()).right
                };

                // rm child, setup child node
                let parent = (*node.as_ptr()).parent.unwrap();
                if (*parent.as_ptr()).left == Some(node) {
                    (*parent.as_ptr()).left = node_child;
                } else if (*parent.as_ptr()).right == Some(node) {
                    (*parent.as_ptr()).right = node_child;
                }

                Node::release(node);
            }
            _ => {
                //我们用其右子树中的最小值替换掉 x
                let right_min_node = find_min((*node.as_ptr()).right).unwrap();
                let right_min = (*right_min_node.as_ptr()).element;
                (*node.as_ptr()).element = right_min;

                //右子树中的这一最小值“切掉”
                let node_right = (*node.as_ptr()).right;
                return delete(right_min, node_right);
            }
        }

        true
    })
}
