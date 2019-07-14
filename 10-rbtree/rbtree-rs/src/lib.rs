#![feature(test)]

mod rbtree {
    #![allow(dead_code)]
    use std::cell::RefCell;
    use std::cmp::Ordering;
    use std::fmt::Display;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    pub struct Node<T: Ord + Eq + Copy + Display> {
        pub value: T,
        pub left: Option<Rc<RefCell<Node<T>>>>,
        pub right: Option<Rc<RefCell<Node<T>>>>,
        pub parent: Weak<RefCell<Node<T>>>,
        pub is_red: bool,
    }

    pub trait TreeNode<T: Ord + Eq + Copy + Display> {
        fn value(&self) -> T;
        fn left(&self) -> Self;
        fn right(&self) -> Self;
        fn parent(&self) -> Self;
        fn uncle(&self) -> Self;
        fn is_red(&self) -> bool;
        fn is_black(&self) -> bool;
        fn is_equal(&self, other: &Self) -> bool;
        fn grandparent(&self) -> Self;
        fn set_left(&self, src: &Self);
        fn set_right(&self, src: &Self);
        fn set_parent(&self, src: &Self);
        fn set_black(&self);
        fn set_red(&self);
        fn copy_parent_from(&self, src: &Self);
        fn dump(&self) -> String;
    }

    impl<T: Ord + Eq + Copy + Display> TreeNode<T> for Option<Rc<RefCell<Node<T>>>> {
        fn dump(&self) -> String {
            let left;
            let right;
            if self.left().is_some() {
                left = format!(
                    "Lv{}c{}pv{}",
                    self.left().value(),
                    if self.left().is_red() { "R" } else { "B" },
                    if self.left().parent().is_some() {
                        self.left().parent().value().to_string()
                    } else {
                        "None".to_string()
                    }
                )
            } else {
                left = "None".to_string();
            }
            if self.right().is_some() {
                right = format!(
                    "Rv{}c{}pv{}",
                    self.right().value(),
                    if self.right().is_red() { "R" } else { "B" },
                    if self.right().parent().is_some() {
                        self.right().parent().value().to_string()
                    } else {
                        "None".to_string()
                    }
                )
            } else {
                right = "None".to_string();
            }

            format!(
                "{}-[v{}c{}pv{}]-{}",
                left,
                self.value(),
                if self.is_red() { "R" } else { "B" },
                if self.parent().is_some() {
                    self.parent().value().to_string()
                } else {
                    "None".to_string()
                },
                right
            )
        }

        fn is_equal(&self, other: &Self) -> bool {
            if self.is_none() || other.is_none() {
                false
            } else {
                Rc::ptr_eq(self.as_ref().unwrap(), other.as_ref().unwrap())
            }
        }

        fn is_red(&self) -> bool {
            if self.is_none() {
                false
            } else {
                self.as_ref().unwrap().borrow().is_red
            }
        }

        fn is_black(&self) -> bool {
            !self.is_red()
        }

        fn set_black(&self) {
            self.as_ref().unwrap().borrow_mut().is_red = false;
        }

        fn set_red(&self) {
            self.as_ref().unwrap().borrow_mut().is_red = true;
        }

        fn value(&self) -> T {
            self.as_ref().unwrap().borrow().value
        }

        fn left(&self) -> Self {
            if self.as_ref().unwrap().borrow().left.is_some() {
                Some(Rc::clone(
                    self.as_ref().unwrap().borrow().left.as_ref().unwrap(),
                ))
            } else {
                None
            }
        }

        fn right(&self) -> Self {
            if self.as_ref().unwrap().borrow().right.is_some() {
                Some(Rc::clone(
                    self.as_ref().unwrap().borrow().right.as_ref().unwrap(),
                ))
            } else {
                None
            }
        }

        fn parent(&self) -> Self {
            self.as_ref().unwrap().borrow().parent.upgrade()
        }

        fn uncle(&self) -> Self {
            if self.grandparent().is_some() {
                if self.grandparent().left().is_some() {
                    if self.grandparent().left().is_equal(&self.parent()) {
                        return self.grandparent().right();
                    } else {
                        return self.grandparent().left();
                    }
                } else {
                    return self.grandparent().left();
                }
            } else {
                return self.grandparent();
            }
        }

        fn grandparent(&self) -> Self {
            if self.parent().is_some() {
                return self.parent().parent();
            } else {
                return self.parent();
            }
        }

        fn set_left(&self, other: &Self) {
            if self.left().is_none() && other.is_none() {
                return;
            } else if other.is_none() {
                self.as_ref().unwrap().borrow_mut().left = None;
            } else {
                self.as_ref().unwrap().borrow_mut().left = Some(Rc::clone(other.as_ref().unwrap()));
            }
        }

        fn set_right(&self, other: &Self) {
            if self.right().is_none() && other.is_none() {
                return;
            } else if other.is_none() {
                self.as_ref().unwrap().borrow_mut().right = None;
            } else {
                self.as_ref().unwrap().borrow_mut().right =
                    Some(Rc::clone(other.as_ref().unwrap()));
            }
        }

        fn set_parent(&self, other: &Self) {
            self.as_ref().unwrap().borrow_mut().parent = Rc::downgrade(other.as_ref().unwrap());
        }

        fn copy_parent_from(&self, other: &Self) {
            self.as_ref().unwrap().borrow_mut().parent =
                other.as_ref().unwrap().borrow().parent.clone();
        }
    }

    impl<T: Ord + Eq + Copy + Display> Node<T> {
        fn new(v: T) -> Node<T> {
            Node {
                value: v,
                left: None,
                right: None,
                parent: Weak::new(),
                is_red: true,
            }
        }
    }

    #[derive(Debug)]
    pub struct RBTree<T: Ord + Eq + Copy + Display> {
        pub root: Option<Rc<RefCell<Node<T>>>>,
    }

    #[derive(Debug)]
    pub struct DuplicateErr {}

    impl<T: Ord + Eq + Copy + Display> RBTree<T> {
        pub fn new(s: &[T]) -> Result<RBTree<T>, DuplicateErr> {
            let mut tree = RBTree { root: None };
            for v in s {
                tree.insert(*v)?;
            }
            Ok(tree)
        }

        pub fn insert(&mut self, v: T) -> Result<(), DuplicateErr> {
            let inserted = Self::insert_util(Weak::new(), &mut self.root, v);
            if inserted.is_ok() {
                self.rebalance(inserted.ok().as_ref().unwrap());
                return Ok(());
            } else {
                return Err(inserted.err().unwrap());
            }
        }

        fn insert_util(
            parent: Weak<RefCell<Node<T>>>,
            node: &mut Option<Rc<RefCell<Node<T>>>>,
            v: T,
        ) -> Result<Option<Rc<RefCell<Node<T>>>>, DuplicateErr> {
            if node.is_some() {
                if v < node.value() {
                    Self::insert_util(
                        Rc::downgrade(node.as_ref().unwrap()),
                        &mut node.as_ref().unwrap().borrow_mut().left,
                        v,
                    )
                } else if v > node.value() {
                    Self::insert_util(
                        Rc::downgrade(node.as_ref().unwrap()),
                        &mut node.as_ref().unwrap().borrow_mut().right,
                        v,
                    )
                } else {
                    return Err(DuplicateErr {});
                }
            } else {
                *node = Some(Rc::new(RefCell::new(Node::new(v))));
                node.as_ref().unwrap().borrow_mut().parent = parent;
                return Ok(Some(Rc::clone(node.as_ref().unwrap())));
            }
        }

        pub fn find(&self, v: T) -> Option<Rc<RefCell<Node<T>>>> {
            Self::find_util(&self.root, v)
        }

        fn find_util(node: &Option<Rc<RefCell<Node<T>>>>, v: T) -> Option<Rc<RefCell<Node<T>>>> {
            match node {
                Some(_) => match v.cmp(&node.value()) {
                    Ordering::Less => Self::find_util(&node.left(), v),
                    Ordering::Equal => Some(Rc::clone(node.as_ref().unwrap())),
                    Ordering::Greater => Self::find_util(&node.right(), v),
                },
                None => None,
            }
        }

        fn rebalance(&mut self, node: &Option<Rc<RefCell<Node<T>>>>) {
            self.case_01(node);
        }

        fn case_01(&mut self, node: &Option<Rc<RefCell<Node<T>>>>) {
            if node.parent().is_none() {
                node.set_black();
            } else {
                self.case_02(node);
            }
        }

        fn case_02(&mut self, node: &Option<Rc<RefCell<Node<T>>>>) {
            if node.grandparent().is_none() {
                return;
            } else {
                self.case_03(node)
            }
        }

        fn case_03(&mut self, node: &Option<Rc<RefCell<Node<T>>>>) {
            if node.uncle().is_some() && node.parent().is_red() && node.uncle().is_red() {
                node.parent().set_black();
                node.uncle().set_black();
                node.grandparent().set_red();
                self.case_01(&node.grandparent());
            } else {
                self.case_04(node)
            }
        }

        fn case_04(&mut self, node: &Option<Rc<RefCell<Node<T>>>>) {
            if node.parent().right().is_equal(node)
                && node.grandparent().left().is_equal(&node.parent())
            {
                self.rotate_left(&node.parent());
                self.case_05(&node.left());
            } else if node.parent().left().is_equal(node)
                && node.grandparent().right().is_equal(&node.parent())
            {
                self.rotate_right(&node.parent());
                self.case_05(&node.right());
            } else {
                self.case_05(node);
            }
        }

        fn case_05(&mut self, node: &Option<Rc<RefCell<Node<T>>>>) {
            node.parent().set_black();
            node.grandparent().set_red();
            if node.parent().left().is_equal(node)
                && node.grandparent().left().is_equal(&node.parent())
            {
                self.rotate_right(&node.grandparent());
            } else if node.parent().right().is_equal(node)
                && node.grandparent().right().is_equal(&node.parent())
            {
                self.rotate_left(&node.grandparent());
            }
        }

        pub fn rotate_left(&mut self, pivot: &Option<Rc<RefCell<Node<T>>>>) {
            let tmp = pivot.right();
            pivot.set_right(&tmp.left());
            if tmp.left().is_some() {
                tmp.left().set_parent(pivot);
            }
            tmp.copy_parent_from(pivot);
            if pivot.parent().is_some() {
                if pivot.parent().left().is_some()
                    && Rc::ptr_eq(
                        pivot.as_ref().unwrap(),
                        pivot.parent().left().as_ref().unwrap(),
                    )
                {
                    pivot.parent().set_left(&tmp);
                } else {
                    pivot.parent().set_right(&tmp);
                }
            } else {
                self.root = Some(Rc::clone(tmp.as_ref().unwrap()));
            }
            tmp.set_left(pivot);
            pivot.set_parent(&tmp);
        }

        pub fn rotate_right(&mut self, pivot: &Option<Rc<RefCell<Node<T>>>>) {
            let tmp = pivot.left();
            pivot.set_left(&tmp.right());
            if tmp.right().is_some() {
                tmp.right().set_parent(pivot);
            }
            tmp.copy_parent_from(pivot);
            if pivot.parent().is_some() {
                if pivot.parent().right().is_some()
                    && Rc::ptr_eq(
                        pivot.as_ref().unwrap(),
                        pivot.parent().right().as_ref().unwrap(),
                    )
                {
                    pivot.parent().set_right(&tmp);
                } else {
                    pivot.parent().set_left(&tmp);
                }
            } else {
                self.root = Some(Rc::clone(tmp.as_ref().unwrap()));
            }
            tmp.set_right(pivot);
            pivot.set_parent(&tmp);
        }
    }
}

#[cfg(test)]
mod tests {

    extern crate test;
    use crate::rbtree::RBTree;
    use crate::rbtree::TreeNode;
    use test::Bencher;

    #[test]
    fn it_works() {
        let t = RBTree::new(&[3, 2, 1]).unwrap();
        assert_eq!(t.root.value(), 2);
    }

    #[test]
    fn find_pos() {
        let t = RBTree::new(&[1, 2, 3]).unwrap();
        assert_eq!(t.find(3).value(), 3);
    }

    #[test]
    fn find_neg_1() {
        let t = RBTree::new(&[1, 2, 3]).unwrap();
        assert_eq!(t.find(4).is_none(), true);
    }

    #[test]
    fn find_neg_2() {
        let t = RBTree::new(&[]).unwrap();
        assert_eq!(t.find(4).is_none(), true);
    }

    #[test]
    fn insert_parent_1() {
        let t = RBTree::new(&[1]).unwrap();
        assert_eq!(t.root.parent().is_none(), true);
    }

    #[test]
    fn insert_parent_2() {
        let t = RBTree::new(&[1, 2]).unwrap();
        assert_eq!(t.root.right().parent().value(), 1);
    }

    #[test]
    fn insert_pos() {
        let t = RBTree::new(&[1, 2, 3]).unwrap();
        assert_eq!(t.find(3).value(), 3);
    }

    #[test]
    fn grandparent_pos_01() {
        let t = RBTree::new(&[10, 6, 2, 15]).unwrap();
        assert_eq!(t.find(15).grandparent().value(), 6);
    }

    #[test]
    fn grandparent_pos_02() {
        let t = RBTree::new(&[10, 6, 15, 20]).unwrap();
        assert_eq!(t.find(20).grandparent().value(), 10);
    }

    #[test]
    fn grandparent_neg() {
        let t = RBTree::new(&[2, 1, 3]).unwrap();
        assert_eq!(t.find(1).grandparent().is_none(), true);
    }

    #[test]
    fn uncle_pos_01() {
        let t = RBTree::new(&[10, 6, 2, 15]).unwrap();
        assert_eq!(t.find(15).uncle().value(), 2);
    }

    #[test]
    fn uncle_pos_02() {
        let t = RBTree::new(&[10, 6, 15, 20]).unwrap();
        assert_eq!(t.find(20).uncle().value(), 6);
    }

    #[test]
    fn uncle_neg() {
        let t = RBTree::new(&[2, 1, 3]).unwrap();
        assert_eq!(t.find(1).uncle().is_none(), true);
    }

    #[test]
    fn full_1() {
        let t = RBTree::new(&[10, 3, 15, 5, 7]).unwrap();
        assert!(t.root.dump().contains("Lv5cBpv10-[v10cBpvNone]-Rv15cBpv10"));
        assert!(t.root.right().dump().contains("None-[v15cBpv10]-None"));
        assert!(t
            .root
            .left()
            .dump()
            .contains("Lv3cRpv5-[v5cBpv10]-Rv7cRpv5"));
        assert!(t.root.left().right().dump().contains("None-[v7cRpv5]-None"));
        assert!(t.root.left().left().dump().contains("None-[v3cRpv5]-None"));
    }

    #[test]
    fn full_2() {
        let t = RBTree::new(&[10, 3, 15, 7, 5]).unwrap();

        assert!(t.root.dump().contains("Lv5cBpv10-[v10cBpvNone]-Rv15cBpv10"));
        assert!(t.root.right().dump().contains("None-[v15cBpv10]-None"));
        assert!(t
            .root
            .left()
            .dump()
            .contains("Lv3cRpv5-[v5cBpv10]-Rv7cRpv5"));
        assert!(t.root.left().right().dump().contains("None-[v7cRpv5]-None"));
        assert!(t.root.left().left().dump().contains("None-[v3cRpv5]-None"));
    }

    #[test]
    fn full_3() {
        let t = RBTree::new(&[10, 3, 15, 13, 14]).unwrap();
        assert!(t.root.dump().contains("Lv3cBpv10-[v10cBpvNone]-Rv14cBpv10"));
        assert!(t
            .root
            .right()
            .dump()
            .contains("Lv13cRpv14-[v14cBpv10]-Rv15cRpv14"));
        assert!(t.root.left().dump().contains("None-[v3cBpv10]-None"));
        assert!(t
            .root
            .right()
            .right()
            .dump()
            .contains("None-[v15cRpv14]-None"));
        assert!(t
            .root
            .right()
            .left()
            .dump()
            .contains("None-[v13cRpv14]-None"));
    }

    #[test]
    fn full_4() {
        let t = RBTree::new(&[10, 4, 15, 20, 25, 30, 35, 40]).unwrap();
        assert!(t
            .root
            .dump()
            .contains("Lv10cRpv20-[v20cBpvNone]-Rv30cRpv20"));
        assert!(t
            .root
            .left()
            .dump()
            .contains("Lv4cBpv10-[v10cRpv20]-Rv15cBpv10"));
        assert!(t
            .root
            .right()
            .dump()
            .contains("Lv25cBpv30-[v30cRpv20]-Rv35cBpv30"));
        assert!(t
            .root
            .right()
            .right()
            .dump()
            .contains("None-[v35cBpv30]-Rv40cRpv35"));
        assert!(t
            .root
            .right()
            .right()
            .right()
            .dump()
            .contains("None-[v40cRpv35]-None"));
        assert!(t
            .root
            .right()
            .left()
            .dump()
            .contains("None-[v25cBpv30]-None"));
        assert!(t.root.left().left().dump().contains("None-[v4cBpv10]-None"));
        assert!(t
            .root
            .left()
            .right()
            .dump()
            .contains("None-[v15cBpv10]-None"));
    }

    #[test]
    fn full_5() {
        let t = RBTree::new(&[30, 35, 25, 28, 26, 20, 15, 10]).unwrap();
        assert!(t
            .root
            .dump()
            .contains("Lv20cRpv26-[v26cBpvNone]-Rv30cRpv26"));
        assert!(t
            .root
            .right()
            .dump()
            .contains("Lv28cBpv30-[v30cRpv26]-Rv35cBpv30"));
        assert!(t
            .root
            .left()
            .dump()
            .contains("Lv15cBpv20-[v20cRpv26]-Rv25cBpv20"));
        assert!(t
            .root
            .left()
            .left()
            .dump()
            .contains("Lv10cRpv15-[v15cBpv20]-None"));
        assert!(t
            .root
            .left()
            .left()
            .left()
            .dump()
            .contains("None-[v10cRpv15]-None"));
        assert!(t
            .root
            .left()
            .right()
            .dump()
            .contains("None-[v25cBpv20]-None"));
        assert!(t
            .root
            .right()
            .left()
            .dump()
            .contains("None-[v28cBpv30]-None"));
        assert!(t
            .root
            .right()
            .right()
            .dump()
            .contains("None-[v35cBpv30]-None"));
    }

    #[bench]
    fn bench_insert(b: &mut Bencher) {
        let v: Vec<i32> = (1..1_000_000).collect();
        b.iter(|| RBTree::new(v.as_slice()));
    }
}
