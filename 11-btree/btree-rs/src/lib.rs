pub mod btree {
    #![allow(dead_code)]
    extern crate bincode;
    extern crate log;
    extern crate lru_cache;
    extern crate memmap;
    extern crate serde;

    use log::{debug, info, trace};
    use lru_cache::LruCache;
    use memmap::{MmapMut, MmapOptions};
    use serde::{Deserialize, Serialize};
    use std::cell::RefCell;
    use std::clone::Clone;
    use std::collections::VecDeque;
    use std::fmt::Debug;
    use std::fmt::Write as FmtWrite;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Write as IoWrite;
    use std::mem::size_of;
    use std::path::Path as FilePath;
    use std::rc::Rc;

    type Block = u32;
    type Degree = u32;
    type Key = u32;
    type Val = u32;
    type Addr = u32;
    type NodeCache = Rc<RefCell<LruCache<Addr, Node>>>;

    pub struct Btree(Rc<RefCell<BtreeInner>>);
    struct Node(Rc<RefCell<NodeInner>>);

    #[derive(Debug)]
    struct NodeInner {
        st: NodeStored,
        addr: Addr,
        bt: Btree,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct NodeStored {
        leaf: bool,
        keys: Vec<Key>,
        vals: Vec<Val>,
        next: Option<Addr>,
    }

    #[derive(Clone, Debug)]
    struct Path(Rc<RefCell<PathInner>>);

    #[derive(Debug)]
    struct PathInner {
        steps: Vec<PathStep>,
        bt: Btree,
    }

    #[derive(Debug, Clone)]
    struct PathRef {
        index: usize,
        path: Path,
    }

    pub struct BtreeInner {
        header: BtreeHeader,
        cache: NodeCache,
        mmap: MmapMut,
        fd: File,
    }

    #[derive(Debug, Copy, Clone)]
    struct StepInfo {
        index: usize, // index of key/val in parent
        addr: Addr,   // offset of node in the file
    }

    #[derive(Debug, Copy, Clone)]
    struct PathStep {
        left: Option<StepInfo>,
        right: Option<StepInfo>,
        node: StepInfo,
    }

    #[derive(Debug)]
    enum InsertTarget {
        Ref(PathRef),
        RefNodeAddr((PathRef, Addr)),
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct BtreeHeader {
        root: Addr,
        min_degree: Degree,
        max_degree: Degree,
        block_size: Block,
    }

    struct TaskManager {
        deq: VecDeque<Task>,
    }

    enum IdxSide {
        Left(usize),
        Right(usize),
    }

    impl Clone for Btree {
        fn clone(&self) -> Self {
            Self(Rc::clone(&self.0))
        }
    }

    impl Clone for Node {
        fn clone(&self) -> Self {
            Self(Rc::clone(&self.0))
        }
    }

    impl Debug for Btree {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Btree max_degree={}, min_degree={}, root={}",
                self.max_degree(),
                self.min_degree(),
                self.root(),
            )
        }
    }

    impl PathRef {
        fn new(path: &Path, index: usize) -> Self {
            PathRef {
                path: path.clone(),
                index,
            }
        }

        fn bt(&self) -> Btree {
            self.path.bt()
        }

        fn tail(path: &Path) -> Self {
            PathRef::new(path, path.len() - 1)
        }

        fn parent_ref(&self) -> Option<PathRef> {
            if self.index == 0 {
                None
            } else {
                Some(PathRef::new(&self.path, self.index - 1))
            }
        }

        fn node_addr(&self) -> Addr {
            self.get_step().node_addr()
        }

        fn get_step(&self) -> PathStep {
            self.path.get_step(self.index)
        }

        fn node_idx(&self) -> Option<usize> {
            let parent_ref = self.parent_ref();
            if parent_ref.is_some() {
                Some(parent_ref.unwrap().get_step().node_idx())
            } else {
                None
            }
        }

        fn node(&self) -> Node {
            self.path.bt().get_node(self.node_addr())
        }

        fn left_sibling_addr(&self) -> Option<Addr> {
            let parent_ref = self.parent_ref();
            if parent_ref.is_some() {
                let step = parent_ref.unwrap().get_step();
                if step.left.is_some() {
                    Some(step.left_addr())
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn left_sibling_idx(&self) -> Option<usize> {
            let parent_ref = self.parent_ref();
            if parent_ref.is_some() {
                let step = parent_ref.unwrap().get_step();
                if step.left.is_some() {
                    Some(step.left_idx())
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn left_sibling(&self) -> Option<Node> {
            let addr = self.left_sibling_addr();
            if addr.is_some() {
                Some(self.path.bt().get_node(addr.unwrap()))
            } else {
                None
            }
        }

        fn right_sibling_idx(&self) -> Option<usize> {
            let parent_ref = self.parent_ref();
            if parent_ref.is_some() {
                let step = parent_ref.unwrap().get_step();
                if step.right.is_some() {
                    Some(step.right_idx())
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn right_sibling_addr(&self) -> Option<Addr> {
            let parent_ref = self.parent_ref();
            if parent_ref.is_some() {
                let step = parent_ref.unwrap().get_step();
                if step.right.is_some() {
                    Some(step.right_addr())
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn right_sibling(&self) -> Option<Node> {
            let addr = self.right_sibling_addr();
            if addr.is_some() {
                Some(self.path.bt().get_node(addr.unwrap()))
            } else {
                None
            }
        }

        fn parent(&self) -> Option<Node> {
            let pref = self.parent_ref();
            if pref.is_some() {
                Some(pref.unwrap().node())
            } else {
                None
            }
        }

        fn top(&self) -> bool {
            self.index == 0
        }

        fn bottom(&self) -> bool {
            (self.path.len() - 1) == self.index
        }
    }

    impl Path {
        fn new(steps: Vec<PathStep>, bt: &Btree) -> Self {
            Path(Rc::new(RefCell::new(PathInner {
                steps,
                bt: bt.clone(),
            })))
        }

        fn get_step(&self, path_index: usize) -> PathStep {
            self.0.borrow().steps[path_index]
        }

        fn len(&self) -> usize {
            self.0.borrow().steps.len()
        }

        fn bt(&self) -> Btree {
            self.0.borrow().bt.clone()
        }
    }

    impl Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Node A={}, R=({}), L=({}), keys={:?}, vals={:?}, N:{:?}",
                self.addr(),
                if self.is_root() { "+" } else { "-" },
                if self.is_leaf() { "+" } else { "-" },
                self.0.borrow().st.keys,
                self.0.borrow().st.vals,
                self.0.borrow().st.next,
            )
        }
    }

    impl Node {
        fn node_builder(bt: Btree, leaf: bool) -> Self {
            let addr = bt.expand_file();
            let node = Self(Rc::new(RefCell::new(NodeInner {
                st: NodeStored {
                    leaf,
                    next: None,
                    keys: Vec::new(),
                    vals: Vec::new(),
                },
                addr: addr,
                bt,
            })));
            node.flush();
            node
        }

        fn new(bt: &Btree) -> Self {
            debug!("Node::new: created");
            Self::node_builder(bt.clone(), false)
        }

        fn new_leaf(bt: &Btree) -> Self {
            debug!("Node::new_leaf: created");
            Self::node_builder(bt.clone(), true)
        }

        fn new_sibling(node: &Node) -> Self {
            if node.is_leaf() {
                Node::new_leaf(&node.bt())
            } else {
                Node::new(&node.bt())
            }
        }

        fn new_root(bt: &Btree) -> Self {
            let new_root = Node::new(bt);
            new_root.bt().set_root(new_root.addr());
            new_root
        }

        fn bt(&self) -> Btree {
            self.0.borrow().bt.clone()
        }

        fn split(&self, index: usize) -> (Option<Addr>, Node, IdxSide) {
            let sibling = Node::new_sibling(self);

            // compute split index and insertion index
            let middle = ((self.degree() + 1) / 2) as usize;
            let (cut_idx, idx) = if index < middle {
                (middle - 1, IdxSide::Left(index))
            } else {
                (middle, IdxSide::Right(index - middle))
            };

            // move part of elements to the new node (sibling)
            sibling.append_n_from(self, cut_idx, self.degree() as usize);

            if self.is_leaf() {
                // update next ref
                sibling.set_next(self.next());
                self.set_next(Some(sibling.addr()));
            }

            // if there is no parent - create one
            let mut new_root = None;
            if self.is_root() {
                new_root = Some(Node::new_root(&self.bt()).addr());
            }

            (new_root, sibling, idx)
        }

        fn set_addr(&self, addr: Addr) {
            trace!("Node::set_addr: old={:?}, new={:?}", self.addr(), addr);
            self.0.borrow_mut().addr = addr;
            self.flush();
        }

        fn set_next(&self, addr: Option<Addr>) {
            debug_assert!(self.is_leaf());
            trace!(
                "Node::set_next: old={:?}, new={:?}",
                self.0.borrow().st.next,
                addr
            );
            self.0.borrow_mut().st.next = addr;
            self.flush();
        }

        fn set_next_from(&self, other: &Node) {
            debug_assert!(self.is_leaf());
            trace!(
                "Node::set_next: old={:?}, new={:?}",
                self.next(),
                other.next()
            );
            self.set_next(other.next());
            self.flush();
        }

        fn next(&self) -> Option<Addr> {
            debug_assert!(self.is_leaf());
            self.0.borrow().st.next
        }

        fn is_leaf(&self) -> bool {
            self.0.borrow().st.leaf
        }

        fn append_from(&self, other: &Self) {
            trace!(
                "Node::append_from: called self={:?}, other={:?}",
                self,
                other
            );
            self.0
                .borrow_mut()
                .st
                .keys
                .append(&mut other.0.borrow_mut().st.keys);
            self.0
                .borrow_mut()
                .st
                .vals
                .append(&mut other.0.borrow_mut().st.vals);
            self.flush();
            trace!("Node::append_from: done self={:?}, other={:?}", self, other);
        }

        fn append_n_from(&self, other: &Self, start: usize, stop: usize) {
            trace!(
                "Node::append_n_from: self={:?}, other={:?}, star={}, stop={}",
                self,
                other,
                start,
                stop
            );
            self.0
                .borrow_mut()
                .st
                .keys
                .extend(other.0.borrow_mut().st.keys.drain(start..stop));
            self.0
                .borrow_mut()
                .st
                .vals
                .extend(other.0.borrow_mut().st.vals.drain(start..stop));
            self.flush();
            other.flush();
            trace!(
                "Node::append_n_from: done self={:?}, other={:?}",
                self,
                other,
            );
        }

        fn push_front_n_from(&self, other: &Self, start: usize, stop: usize) {
            trace!(
                "Node::push_front_n_from: self={:?}, other={:?}, star={}, stop={}",
                self,
                other,
                start,
                stop
            );
            {
                let mut node_inner = self.0.borrow_mut();
                for key in other.0.borrow_mut().st.keys.drain(start..stop).rev() {
                    node_inner.st.keys.insert(0, key);
                }
                for val in other.0.borrow_mut().st.vals.drain(start..stop).rev() {
                    node_inner.st.vals.insert(0, val);
                }
            }
            self.flush();
            other.flush();
        }

        fn degree(&self) -> Degree {
            self.0.borrow().st.vals.len() as Degree
        }

        fn min_key(&self) -> Key {
            if self.is_leaf() {
                self.get_key(0)
            } else {
                self.get_key(1)
            }
        }

        fn fist_child_addr(&self) -> Val {
            debug_assert!(!self.is_leaf());
            self.get_val(0)
        }

        fn child_min_key(&self) -> Key {
            debug_assert!(!self.is_leaf());
            self.get_key(0)
        }

        fn is_drained(&self) -> bool {
            if self.is_root() {
                if self.is_empty() {
                    return true;
                }
                return false;
            }

            let min_degree = self.bt().min_degree();
            if self.is_leaf() {
                self.degree() < min_degree
            } else {
                self.degree() <= min_degree
            }
        }

        fn is_full(&self) -> bool {
            self.degree() == self.bt().max_degree()
        }

        fn is_empty(&self) -> bool {
            if self.is_leaf() {
                self.degree() == 0
            } else {
                self.degree() == 1
            }
        }

        fn can_merge(&self, other: &Self) -> bool {
            self.degree() + other.degree() <= self.bt().max_degree()
        }

        fn addr(&self) -> Addr {
            self.0.borrow().addr
        }

        fn is_root(&self) -> bool {
            self.addr() == self.bt().root()
        }

        fn remove(&self, index: usize) -> (Key, Val) {
            debug!("Node::remove: index={}", index);
            let key = self.0.borrow_mut().st.keys.remove(index);
            let val = self.0.borrow_mut().st.vals.remove(index);
            self.flush();
            (key, val)
        }

        fn find(&self, key: Key) -> Result<usize, usize> {
            debug!("Node::find: key={}", key);
            let node_internal = self.0.borrow();
            let keys = if self.is_leaf() {
                node_internal.st.keys.as_slice()
            } else {
                &node_internal.st.keys[1..]
            };

            match keys.binary_search(&key) {
                Ok(idx) => {
                    if self.is_leaf() {
                        Ok(idx)
                    } else {
                        Ok(idx + 1)
                    }
                }
                Err(idx) => {
                    if self.is_leaf() {
                        Err(idx)
                    } else {
                        Err(idx + 1)
                    }
                }
            }
        }

        fn insert(&self, index: usize, key: Key, val: Val) {
            info!(
                "Node::insert: index={}, key={}, val={}, len={}",
                index,
                key,
                val,
                self.0.borrow_mut().st.keys.len()
            );
            self.0.borrow_mut().st.keys.insert(index, key);
            self.0.borrow_mut().st.vals.insert(index, val);
            self.flush();
        }

        fn update_key(&self, index: usize, new_key: Key) -> Key {
            trace!("Node::update_key: index={}, new_key={}", index, new_key);
            let old_key = self.get_key(index);
            self.0.borrow_mut().st.keys[index] = new_key;
            self.flush();
            old_key
        }

        fn update_val(&self, index: usize, new_val: Val) -> Val {
            trace!("Node::update_val: index={}, new_val={}", index, new_val);
            let old_val = self.get_val(index);
            self.0.borrow_mut().st.vals[index] = new_val;
            self.flush();
            old_val
        }

        fn flush(&self) {
            trace!("Node:flush: self={:?}", self);
            if self.bt().cache_cap() != 0 {
                if self.bt().cache_get(self.addr()).is_some() {
                    return;
                }

                let result = self.bt().cache_put(&self);
                if result.is_none() {
                    return;
                }

                self.bt().flush_node(&result.unwrap());
            } else {
                self.bt().flush_node(&self);
            }
        }

        fn get_vals(&self) -> Vec<Val> {
            self.0.borrow().st.vals.clone()
        }

        fn get_val(&self, index: usize) -> Val {
            self.0.borrow().st.vals[index]
        }

        fn get_key(&self, index: usize) -> Key {
            self.0.borrow().st.keys[index]
        }

        fn find_next_node(&self, key: Key) -> (Addr, PathStep) {
            debug_assert!(!self.is_leaf());
            trace!("Node:find_next_node: key={}", key);
            let index = match self.find(key) {
                Ok(idx) => idx,
                Err(idx) => idx - 1,
            };

            let next_node_addr = self.get_val(index);
            let step;
            if index == 0 {
                step = PathStep::new(
                    None,
                    Some((index + 1, self.get_val(index + 1))),
                    (index, self.addr()),
                )
            } else if self.degree() as usize - 1 > index {
                step = PathStep::new(
                    Some((index - 1, self.get_val(index - 1))),
                    Some((index + 1, self.get_val(index + 1))),
                    (index, self.addr()),
                )
            } else {
                step = PathStep::new(
                    Some((index - 1, self.get_val(index - 1))),
                    None,
                    (index, self.addr()),
                )
            }
            return (next_node_addr, step);
        }
    }

    impl TaskManager {
        fn new() -> Self {
            trace!("TaskManager:new: called");
            Self {
                deq: VecDeque::new(),
            }
        }

        fn add_insert(&mut self, target: InsertTarget, index: usize, key: Key, val: Val) {
            trace!(
                "TaskManager:add_insert: target={:?}, index={}, key={}, val={}",
                target,
                index,
                key,
                val,
            );
            self.deq.push_back(Task::Insert {
                target,
                index,
                key,
                val,
            });
        }

        fn add_update(&mut self, pref: PathRef, index: usize, new_key: Key) {
            trace!(
                "TaskManager:add_update: pref={:?}, index={}, new_key={}",
                pref,
                index,
                new_key,
            );
            self.deq.push_back(Task::Update {
                pref,
                index,
                new_key,
            });
        }

        fn add_remove(&mut self, pref: PathRef, index: usize) {
            trace!("TaskManager:add_remove: pref={:?}, index={}", pref, index,);
            self.deq.push_back(Task::Remove { pref, index });
        }

        fn add_rebalance(&mut self, pref: PathRef) {
            trace!("TaskManager:add_rebelance: pref={:?}", pref);
            self.deq.push_back(Task::Rebalance { pref });
        }

        fn add_split(&mut self, pref: PathRef, index: usize, key: Key, val: Val) {
            trace!(
                "TaskManager:add_split: pref={:?}, index={}, key={}, val={}",
                pref,
                index,
                key,
                val,
            );
            self.deq.push_back(Task::Split {
                pref,
                index,
                key,
                val,
            });
        }

        fn run(&mut self) {
            debug_assert!(self.deq.len() > 0);
            trace!("TaskManager:run: called");

            while self.deq.len() > 0 {
                match self.deq.pop_front().unwrap() {
                    Task::Insert {
                        target,
                        index,
                        key,
                        val,
                    } => self.insert_util(target, index, key, val),
                    Task::Rebalance { pref } => self.rebalance_util(pref),
                    Task::Remove { pref, index } => self.remove_util(pref, index),
                    Task::Split {
                        pref,
                        index,
                        key,
                        val,
                    } => self.split_util(pref, index, key, val),
                    Task::Update {
                        pref,
                        index,
                        new_key,
                    } => self.update_util(pref, index, new_key),
                }
            }
        }

        fn insert_util(&mut self, target: InsertTarget, index: usize, key: Key, val: Key) {
            debug!(
                "TaskManager:insert_util: target={:?}, i={}, k={}, v={}",
                target, index, key, val
            );

            let (pref, node) = match target {
                InsertTarget::Ref(pref) => (pref.clone(), pref.node()),
                InsertTarget::RefNodeAddr((pref, addr)) => (pref.clone(), pref.bt().get_node(addr)),
            };

            if node.is_full() {
                self.add_split(pref, index, key, val);
                return;
            } else {
                node.insert(index, key, val);
            }

            if index != 0 || pref.top() || node.is_root() || pref.node_addr() != node.addr() {
                trace!("TaskManager:insert_util: done, plain insert, short path");
                return;
            }

            // we update min_key and should update parent
            // leftmost node of the tree. special case.
            self.add_update(pref.parent_ref().unwrap(), pref.node_idx().unwrap(), key);
            trace!("TaskManager:insert_util: done, long path");
        }

        fn rebalance_util(&mut self, pref: PathRef) {
            trace!("TaskManager:rebalance_util: pref={:?}", pref);
            let mut node = pref.node();
            let from_right = pref.right_sibling_addr().is_some();
            let sibling_idx = if from_right {
                pref.right_sibling_idx().unwrap()
            } else {
                pref.left_sibling_idx().unwrap()
            };

            let mut sibling: Node;
            if from_right {
                sibling = pref.right_sibling().unwrap();
            } else {
                sibling = pref.left_sibling().unwrap();
            }

            if node.can_merge(&sibling) {
                // merge
                if from_right {
                    if node.is_leaf() && node.is_empty() {
                        self.add_update(
                            pref.parent_ref().unwrap(),
                            pref.node_idx().unwrap(),
                            sibling.min_key(),
                        );
                    }
                    node.append_from(&mut sibling);
                    if node.is_leaf() {
                        node.set_next_from(&sibling);
                    }
                } else {
                    if node.is_leaf() && sibling.is_empty() {
                        self.add_update(pref.parent_ref().unwrap(), sibling_idx, sibling.min_key());
                    }
                    sibling.append_from(&mut node);
                    if node.is_leaf() {
                        sibling.set_next_from(&node);
                    }
                }
                let parent_index = if from_right {
                    pref.right_sibling_idx().unwrap()
                } else {
                    pref.node_idx().unwrap()
                };
                self.add_remove(pref.parent_ref().unwrap(), parent_index);
            } else {
                // rebalance
                let num_taken = (node.degree() + sibling.degree()) / 2 - node.degree();
                if from_right {
                    // remove from middle/end.
                    // underflow/rebalance needed. Remove key, rebalance from right sibling. No merge.
                    let start = 0;
                    let stop = num_taken as usize;
                    node.append_n_from(&mut sibling, start, stop);
                } else {
                    // remove from middle/end.
                    // underflow/rebalance needed. Remove key, rebalance from left sibling. No merge.
                    let start = (sibling.degree() - num_taken) as usize;
                    let stop = sibling.degree() as usize;
                    node.push_front_n_from(&mut sibling, start, stop);
                }

                let node = if from_right { sibling } else { node };
                let parent_index = if from_right {
                    pref.right_sibling_idx().unwrap()
                } else {
                    pref.node_idx().unwrap()
                };
                let new_min_key = if node.is_leaf() {
                    node.min_key()
                } else {
                    node.child_min_key()
                };
                self.add_update(pref.parent_ref().unwrap(), parent_index, new_min_key);
            }
        }

        fn remove_util(&mut self, pref: PathRef, index: usize) {
            trace!("TaskManager:remove_util: pref={:?}, index={}", pref, index);
            let node = pref.node();
            debug_assert!(!(!node.is_leaf() && index == 0));

            // simple remove from the middle/end.
            // no underflow/rebalance, no min_key change.
            let (_, _) = node.remove(index);

            if node.is_drained() && node.is_root() && !node.is_leaf() {
                // underflow - rebalance needed.
                // Remove key, merge with LEFT/RIGHT sibling.
                // Remove min_key from parent. Parrent merge needed (remove root as case).
                pref.bt().set_root(node.fist_child_addr());
                return;
            }

            if node.is_root() {
                return;
            }

            // remove min_key from leaf.
            // no underflow/rebalance, remove min_key, update min_key at the parent.
            if node.is_leaf() && index == 0 && !node.is_empty() {
                let new_min_key = node.min_key();
                self.add_update(
                    pref.parent_ref().unwrap(),
                    pref.node_idx().unwrap(),
                    new_min_key,
                );
            }

            if node.is_drained() {
                // underflow/rebalance needed.
                //
                // Remove key, rebalance from LEFT/RIGHT sibling. No merge.
                //
                // Remove key, merge with LEFT/RIGHT sibling.
                // Remove min_key from parent. No parrent merge/rebalance needed.
                //
                // Remove key, merge with LEFT/RIGHT sibling.
                // Remove min_key from parent. Parrent rebalance needed.
                //
                self.add_rebalance(pref);
            }
            // happy path. No rebalance/merge/update needed.
        }

        fn split_util(&mut self, pref: PathRef, index: usize, key: Key, val: Val) {
            trace!(
                "TaskManager:split_util: pref={:?}, index={}, key={}, val={}",
                pref,
                index,
                key,
                val,
            );

            let node = pref.node();
            let (new_root_addr, sibling, direction) = node.split(index);

            // return control back to insert
            match direction {
                IdxSide::Left(idx) => {
                    self.add_insert(InsertTarget::Ref(pref.clone()), idx, key, val);
                }
                IdxSide::Right(idx) => {
                    self.add_insert(
                        InsertTarget::RefNodeAddr((pref.clone(), sibling.addr())),
                        idx,
                        key,
                        val,
                    );
                }
            }

            if new_root_addr.is_some() {
                let key = match direction {
                    IdxSide::Left(0) => key,
                    _ => node.min_key(),
                };
                // insert to the new_root min_key from node
                self.add_insert(
                    InsertTarget::RefNodeAddr((pref.clone(), *new_root_addr.as_ref().unwrap())),
                    0,
                    key,
                    node.addr(),
                );
            }

            let parent_target = if new_root_addr.is_some() {
                InsertTarget::RefNodeAddr((pref.clone(), *new_root_addr.as_ref().unwrap()))
            } else {
                InsertTarget::Ref(pref.parent_ref().unwrap())
            };
            let parent_sibling_idx = if new_root_addr.is_some() {
                1
            } else {
                pref.node_idx().unwrap() + 1
            };
            let sibling_min_key = if !sibling.is_leaf() {
                match direction {
                    IdxSide::Right(0) => key,
                    _ => sibling.child_min_key(),
                }
            } else {
                match direction {
                    IdxSide::Right(0) => key,
                    _ => sibling.min_key(),
                }
            };

            self.add_insert(
                parent_target,
                parent_sibling_idx,
                sibling_min_key,
                sibling.addr(),
            );
        }

        fn update_util(&mut self, pref: PathRef, index: usize, new_key: Addr) {
            trace!(
                "TaskManager:update_util: pref={:?}, index={}, new_key={}",
                pref,
                index,
                new_key,
            );
            let mut node = pref.node();
            let old_key = node.update_key(index, new_key);
            let mut pref = pref.parent_ref();
            while pref.is_some() {
                let rf = pref.unwrap();
                let ps = rf.get_step();
                node = rf.node();
                if node.get_key(ps.node_idx()) == old_key {
                    node.update_key(ps.node_idx(), new_key);
                }
                pref = rf.parent_ref();
            }
        }
    }

    #[derive(Debug)]
    enum Task {
        Insert {
            target: InsertTarget,
            index: usize,
            key: Key,
            val: Val,
        },
        Update {
            pref: PathRef,
            index: usize,
            new_key: Key,
        },
        Remove {
            pref: PathRef,
            index: usize,
        },
        Rebalance {
            pref: PathRef,
        },
        Split {
            pref: PathRef,
            index: usize,
            key: Key,
            val: Val,
        },
    }

    impl PathStep {
        fn new(
            left: Option<(usize, Addr)>,
            right: Option<(usize, Addr)>,
            node: (usize, Addr),
        ) -> PathStep {
            trace!(
                "PathStep:new: left={:?}, right={:?}, node={:?}",
                left,
                right,
                node
            );
            PathStep {
                left: match left {
                    Some((idx, addr)) => Some(StepInfo {
                        index: idx,
                        addr: addr,
                    }),
                    None => None,
                },
                right: match right {
                    Some((idx, addr)) => Some(StepInfo {
                        index: idx,
                        addr: addr,
                    }),
                    None => None,
                },
                node: StepInfo {
                    index: node.0,
                    addr: node.1,
                },
            }
        }

        fn right_idx(&self) -> usize {
            self.right.as_ref().unwrap().index
        }

        fn left_idx(&self) -> usize {
            self.left.as_ref().unwrap().index
        }

        fn node_idx(&self) -> usize {
            self.node.index
        }

        fn right_addr(&self) -> Addr {
            self.right.as_ref().unwrap().addr
        }

        fn left_addr(&self) -> Addr {
            self.left.as_ref().unwrap().addr
        }

        fn node_addr(&self) -> Addr {
            self.node.addr
        }
    }

    impl Btree {
        pub fn new(
            path: &FilePath,
            block_size: Block,
            alpha: u8,
            max_file_size: Block,
            cache_size: usize,
        ) -> Self {
            trace!(
                "Btree:new: path={:?}, block_size={}, alpha={}, max_file_size={}, cache_size={}",
                path,
                block_size,
                alpha,
                max_file_size,
                cache_size,
            );
            let fd = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)
                .unwrap();
            let max_degree = get_max_degree(block_size);
            let min_degree = get_min_degree(max_degree, alpha);
            let header = BtreeHeader {
                root: block_size, // addr of the root node
                block_size,
                min_degree,
                max_degree,
            };

            let mmap = unsafe {
                MmapOptions::new()
                    .len(block_size as usize * max_file_size as usize)
                    .map_mut(&fd)
                    .unwrap()
            };
            let bti = BtreeInner {
                header,
                cache: Rc::new(RefCell::new(LruCache::new(cache_size))),
                fd,
                mmap,
            };

            let bt = Btree(Rc::new(RefCell::new(bti)));
            // Allocate first block for the btree struct (header)
            bt.expand_file();
            bt.flush();
            // Allocate new space in file for the root node.
            let _ = Node::new_leaf(&bt);
            bt
        }

        fn get_file_size(&self) -> u64 {
            self.0.borrow().fd.metadata().unwrap().len()
        }

        fn cache_get(&self, addr: Addr) -> Option<Node> {
            match self.0.borrow().cache.borrow_mut().get_mut(&addr) {
                Some(node) => Some(node.clone()),
                None => None,
            }
        }

        fn set_cache_cap(&self, new_cap: usize) -> usize {
            let old_cap = self.0.borrow().cache.borrow().capacity();
            self.0.borrow().cache.borrow_mut().set_capacity(new_cap);
            old_cap
        }

        fn cache_cap(&self) -> usize {
            self.0.borrow().cache.borrow().capacity()
        }

        fn cache_is_full(&self) -> bool {
            self.0.borrow().cache.borrow().len() == self.cache_cap()
        }

        fn cache_put(&self, node: &Node) -> Option<Node> {
            trace!("Btree:cache_put: node={:?}", node);
            let result;

            if self.cache_is_full() {
                let (_, old_node) = self.0.borrow().cache.borrow_mut().remove_lru().unwrap();
                result = Some(old_node);
            } else {
                result = None;
            }
            self.0
                .borrow_mut()
                .cache
                .borrow_mut()
                .insert(node.addr(), node.clone());
            result
        }

        pub fn flush_cache(&self) {
            let cache = Rc::clone(&self.0.borrow().cache);
            for (_, node) in cache.borrow().iter() {
                self.flush_node(node);
            }
        }

        fn flush_node(&self, node: &Node) {
            trace!("Btree:flush_node: node={:?}", node);
            let se = bincode::serialize(&node.0.borrow().st).unwrap();
            let addr = node.addr() as usize;
            let range = addr..(addr + se.len());

            self.0.borrow_mut().mmap.as_mut()[range].copy_from_slice(&se);
            self.0
                .borrow_mut()
                .mmap
                .flush_range(addr, se.len())
                .unwrap();
        }

        fn flush(&self) {
            trace!("Btree:flush: called");
            let se = bincode::serialize(&self.0.borrow_mut().header).unwrap();
            self.0.borrow_mut().mmap.as_mut()[..se.len()].copy_from_slice(&se);
            self.0.borrow_mut().mmap.flush_range(0, se.len()).unwrap();
        }

        pub fn load(
            path: &FilePath,
            block_size: Block,
            max_file_size: Block,
            cache_size: usize,
        ) -> Self {
            trace!(
                "Btree:load: path={:?}, block_size={}, max_file_size={}",
                path,
                block_size,
                max_file_size
            );
            let fd = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .open(path)
                .unwrap();
            let mmap = unsafe {
                MmapOptions::new()
                    .len(block_size as usize * max_file_size as usize)
                    .map_mut(&fd)
                    .unwrap()
            };
            let header: BtreeHeader = bincode::deserialize(mmap.as_ref()).unwrap();
            debug!("load: BtreeHeader loaded={:?}", &header);
            debug_assert!(header.block_size == block_size);
            let bti = BtreeInner {
                cache: Rc::new(RefCell::new(LruCache::new(cache_size))),
                header,
                mmap,
                fd,
            };
            Btree(Rc::new(RefCell::new(bti)))
        }

        fn block_size(&self) -> Block {
            self.0.borrow().header.block_size
        }

        fn expand_file(&self) -> Addr {
            // expand file by 1 block and return address of new block
            let block_size = self.block_size() as u64;
            let fd = &mut self.0.borrow_mut().fd;
            let addr = fd.metadata().unwrap().len();
            trace!("Btree:expand_file: len before expand={}", addr);
            fd.set_len(addr + block_size).unwrap();
            addr as Addr
        }

        fn get_node(&self, addr: Addr) -> Node {
            debug_assert!(addr >= self.block_size() && addr % self.block_size() == 0);
            trace!("Btree:get_node: addr={}", addr);

            match self.cache_get(addr) {
                Some(node) => {
                    trace!("get_node: done from cache, loaded={:?}", node);
                    node
                }
                None => {
                    let bti = self.0.borrow();
                    let block = &bti.mmap.as_ref()[(addr as usize)..];
                    let st: NodeStored = bincode::deserialize(&block).unwrap();
                    let node_inner = NodeInner {
                        st,
                        addr,
                        bt: self.clone(),
                    };
                    trace!("get_node: done from storage, loaded={:?}", node_inner);
                    Node(Rc::new(RefCell::new(node_inner)))
                }
            }
        }

        fn root(&self) -> Addr {
            self.0.borrow().header.root
        }

        fn set_root(&self, addr: Addr) {
            trace!("Btree:set_root: addr={}", addr);
            self.0.borrow_mut().header.root = addr;
            self.flush();
        }

        fn find_leaf(&self, key: Key) -> (Node, PathRef) {
            trace!("Btree:find_leaf: key={}", key);
            let mut steps = Vec::new();
            let mut node = self.get_node(self.root());
            while !node.is_leaf() {
                let (next_node_addr, step) = node.find_next_node(key);
                steps.push(step);
                node = self.get_node(next_node_addr);
            }
            steps.push(PathStep::new(None, None, (0, node.addr())));
            let path = Path::new(steps, self);
            (node, PathRef::tail(&path))
        }

        fn min_degree(&self) -> Degree {
            self.0.borrow().header.min_degree
        }

        fn set_min_degree(&self, degree: Degree) {
            trace!("Btree:set_min_degree: degree={}", degree);
            self.0.borrow_mut().header.min_degree = degree;
        }

        fn max_degree(&self) -> Degree {
            self.0.borrow().header.max_degree
        }

        fn set_max_degree(&self, degree: Degree) {
            trace!("Btree:set_max_degree: degree={}", degree);
            self.0.borrow_mut().header.max_degree = degree;
        }

        pub fn set_degree(&self, min_degree: Degree, max_degree: Degree) {
            trace!(
                "Btree:set_degree: min_degree={}, max_degree={}",
                min_degree,
                max_degree
            );
            self.set_min_degree(min_degree);
            self.set_max_degree(max_degree);
            self.flush();
        }

        pub fn find(&self, key: Key) -> Result<Val, ()> {
            trace!("Btree:find: key={}", key);
            let (leaf, _) = self.find_leaf(key);
            match leaf.find(key) {
                Ok(idx) => Ok(leaf.get_val(idx)),
                Err(_) => Err(()),
            }
        }

        pub fn insert(&self, key: Key, val: Val) -> Result<(), ()> {
            debug!("Btree:insert: key={}, val={}", key, val);
            let (leaf, last_ref) = self.find_leaf(key);
            let index = match leaf.find(key) {
                Ok(_) => return Err(()),
                Err(idx) => idx,
            };

            let mut mgr = TaskManager::new();
            mgr.add_insert(
                InsertTarget::RefNodeAddr((last_ref, leaf.addr())),
                index,
                key,
                val,
            );
            mgr.run();
            Ok(())
        }

        pub fn remove(&self, key: Key) -> Result<Val, ()> {
            debug!("Btree:remove: key={}", key);
            let (leaf, last_ref) = self.find_leaf(key);
            let index = match leaf.find(key) {
                Ok(idx) => idx,
                Err(_) => return Err(()),
            };
            let mut mgr = TaskManager::new();
            let result = leaf.get_val(index);
            mgr.add_remove(last_ref, index);
            mgr.run();
            Ok(result)
        }

        pub fn compact(&self) -> Result<(), ()> {
            debug!("Btree:compact: called");
            // flush and disable cache
            self.flush_cache();
            let old_cache_cap = self.set_cache_cap(0);
            // traverse tree, DFS
            let mut node_refs = Vec::new();
            let mut stack = Vec::new();
            stack.push((PathStep::new(None, None, (0, self.root())), None));
            while stack.len() > 0 {
                let (step, pinfo) = stack.pop().unwrap();
                node_refs.push((step, pinfo));
                let node_addr = step.node_addr();
                let node: Node = self.get_node(node_addr);
                if node.is_leaf() {
                    continue;
                }
                let vals = node.get_vals();
                for (i, v) in vals.iter().enumerate() {
                    let left = if i == 0 {
                        None
                    } else {
                        Some((i - 1, vals[i - 1]))
                    };
                    let right = if i == vals.len() - 1 {
                        None
                    } else {
                        Some((i + 1, vals[i + 1]))
                    };
                    let center = (i, *v);
                    stack.push((
                        PathStep::new(left, right, center),
                        Some(StepInfo {
                            index: i,
                            addr: node.addr(),
                        }),
                    ));
                }
            }
            // sort nodes by block-addresses
            node_refs.sort_by_key(|x| x.0.node_addr());
            let block_size = self.block_size();
            let file_size = self.get_file_size() as Addr;
            // make list of all available block-addresses in the file
            let mut addrs = Vec::new();
            let mut addr = block_size;
            while addr < file_size {
                addrs.push(addr);
                addr += block_size;
            }
            debug_assert!(node_refs.len() <= addrs.len());
            // check if blocks used or not
            while node_refs.len() > 0 {
                let (nref, pinfo) = node_refs.remove(0);
                let addr = addrs.remove(0);
                if nref.node_addr() == addr {
                    // block used, skip
                    continue;
                }
                node_refs.insert(0, (nref, pinfo));
                let (last_ref, parent_info) = node_refs.pop().unwrap();
                // block is free. Move node from tail to this block and update refs
                let last_node = self.get_node(last_ref.node_addr());
                last_node.set_addr(addr);

                let left_sibling_info = last_ref.left;
                if left_sibling_info.is_some() {
                    let left_sibling = self.get_node(left_sibling_info.unwrap().addr);
                    left_sibling.set_next(Some(addr));
                }
                if parent_info.is_some() {
                    let parent_info = parent_info.unwrap();
                    let parent = self.get_node(parent_info.addr);
                    parent.update_val(parent_info.index, addr);
                } else {
                    // update root
                    self.set_root(addr)
                }
            }
            // if there is some unused blocks left - trim them
            if addrs.len() > 0 {
                let trim_size = addrs.len() as u64 * block_size as u64;
                let fd = &mut self.0.borrow_mut().fd;
                let addr = fd.metadata().unwrap().len();
                trace!("Btree:compact: len before trim={}", addr);
                fd.set_len(addr - trim_size).unwrap();
                let addr = fd.metadata().unwrap().len();
                trace!("Btree:compact: len after trim={}", addr);
            }
            self.set_cache_cap(old_cache_cap);
            Ok(())
        }
    }

    impl Btree {
        pub fn dump_to_stdout(&self) {
            write!(std::io::stdout(), "\n------------------------------\n").unwrap();
            write!(std::io::stdout(), "{}", self.dump_to_string()).unwrap();
            write!(std::io::stdout(), "------------------------------\n").unwrap();
        }

        pub fn dump_to_string(&self) -> String {
            let mut result = String::new();
            let mut stack = Vec::new();
            stack.push(self.root());
            while stack.len() != 0 {
                let node = self.get_node(stack.pop().unwrap());
                write!(result, "{:?}\n", node).unwrap();
                if !node.is_leaf() {
                    stack.extend_from_slice(&node.0.borrow().st.vals)
                }
            }
            result
        }
    }

    fn get_max_degree(block_size: Block) -> Degree {
        let degree = ((block_size as usize - size_of::<NodeStored>())
            / (size_of::<Key>() + size_of::<Val>())) as Degree;
        debug!(
            "get_degree: called with block_size={}, degree={}",
            block_size, degree
        );
        degree
    }

    fn get_min_degree(max_degree: Degree, alpha: u8) -> Degree {
        debug_assert!(alpha >= 2);
        if (max_degree / alpha as Degree) < 1 {
            1
        } else {
            max_degree / alpha as Degree
        }
    }
}

#[cfg(test)]
mod tests {

    extern crate env_logger;
    extern crate log;
    extern crate rand;
    use crate::btree;
    use log::debug;

    fn log_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn setup(path: &std::path::Path) {
        debug!("setup: checking file.");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn base_test() {
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=10 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let expected = vec![
            "Node A=3584, R=(+), L=(-), keys=[3, 5], vals=[1536, 3072], N:None",
            "Node A=3072, R=(-), L=(-), keys=[5, 7, 9], vals=[2048, 2560, 4096], N:None",
            "Node A=4096, R=(-), L=(+), keys=[9, 10], vals=[99, 110], N:None",
            "Node A=2560, R=(-), L=(+), keys=[7, 8], vals=[77, 88], N:Some(4096)",
            "Node A=2048, R=(-), L=(+), keys=[5, 6], vals=[55, 66], N:Some(2560)",
            "Node A=1536, R=(-), L=(-), keys=[1, 3], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 4], vals=[33, 44], N:Some(2048)",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn base_compact() {
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=10 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(5);
        let _ = bt.remove(6);
        let _ = bt.compact();
        let expected = vec![
            "Node A=3584, R=(+), L=(-), keys=[3, 7], vals=[1536, 3072], N:None",
            "Node A=3072, R=(-), L=(-), keys=[7, 9], vals=[2048, 2560], N:None",
            "Node A=2560, R=(-), L=(+), keys=[9, 10], vals=[99, 110], N:None",
            "Node A=2048, R=(-), L=(+), keys=[7, 8], vals=[77, 88], N:Some(2560)",
            "Node A=1536, R=(-), L=(-), keys=[1, 3], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 4], vals=[33, 44], N:Some(2048)",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn base_huge() {
        log_init();
        let path = std::path::Path::new("test.idx");
        {
            setup(&path);
            let bt = btree::Btree::new(std::path::Path::new("test.idx"), 4096, 2, 500_000, 100);
            for i in 1..=100_000 {
                let _ = bt.insert(i, i * 10 + i);
            }
            bt.flush_cache();
        }
        let bt = btree::Btree::load(path, 4096, 50_000, 100);
        assert!(bt.find(100_000).unwrap() == 1_100_000);
    }

    #[test]
    fn insert_case_01() {
        // Simple insert.
        // insertion in the middle of the leaf node.
        // no split needed. Just plain insert.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        for i in 5..=7 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let expected = vec!["Node A=512, R=(+), L=(+), keys=[5, 6, 7], vals=[55, 66, 77], N:None"];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn insert_case_02() {
        // Simple insert in the beginning of leaf.
        // Only left leaf node of the entire tree is affected.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        let _ = bt.insert(5, 55);
        let _ = bt.insert(6, 66);
        let _ = bt.insert(4, 44);
        let expected = vec!["Node A=512, R=(+), L=(+), keys=[4, 5, 6], vals=[44, 55, 66], N:None"];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn insert_case_03() {
        // Simple insert at the end of the leaf node.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 4..=6 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let expected = vec!["Node A=512, R=(+), L=(+), keys=[4, 5, 6], vals=[44, 55, 66], N:None"];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn insert_split_case_01() {
        // Insert with split.
        // Root is leaf. Insert. Split. Add new root and populate with new min keys.
        // Insert at the begin of the first half.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 5..=7 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.insert(4, 44);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[4, 6], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[6, 7], vals=[66, 77], N:None",
            "Node A=512, R=(-), L=(+), keys=[4, 5], vals=[44, 55], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn insert_split_case_02() {
        // Insert with split.
        // Insert at the end of the first half.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        let _ = bt.insert(5, 55);
        let _ = bt.insert(8, 88);
        let _ = bt.insert(9, 99);
        let _ = bt.insert(7, 77);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[5, 8], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[8, 9], vals=[88, 99], N:None",
            "Node A=512, R=(-), L=(+), keys=[5, 7], vals=[55, 77], N:Some(1024)",
        ];
        // bt.dump_to_stdout();
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn insert_split_case_03() {
        // Insert with split.
        // Insert at the begin of the second half.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        let _ = bt.insert(1, 11);
        let _ = bt.insert(2, 22);
        let _ = bt.insert(4, 44);
        let _ = bt.insert(3, 33);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 3], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 4], vals=[33, 44], N:None",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn insert_split_case_04() {
        // Insert with split.
        // Insert at the end of the second half.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=4 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 3], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 4], vals=[33, 44], N:None",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn insert_split_case_05() {
        // Insert with split, which generates insert in the parent.
        // Parent overflowed and split. New grandparent contructed.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=8 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let expected = vec![
            "Node A=3584, R=(+), L=(-), keys=[3, 5], vals=[1536, 3072], N:None",
            "Node A=3072, R=(-), L=(-), keys=[5, 7], vals=[2048, 2560], N:None",
            "Node A=2560, R=(-), L=(+), keys=[7, 8], vals=[77, 88], N:None",
            "Node A=2048, R=(-), L=(+), keys=[5, 6], vals=[55, 66], N:Some(2560)",
            "Node A=1536, R=(-), L=(-), keys=[1, 3], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 4], vals=[33, 44], N:Some(2048)",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_01() {
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        let _ = bt.insert(1, 11);
        let _ = bt.remove(1);
        let expected = vec!["Node A=512, R=(+), L=(+), keys=[], vals=[], N:None"];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_02() {
        // remove minkey from leaf. Update parent node.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=4 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(3);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 4], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[4], vals=[44], N:None",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_03() {
        // remove minkey from leaf. Update parent node.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=4 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(1);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[2, 3], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 4], vals=[33, 44], N:None",
            "Node A=512, R=(-), L=(+), keys=[2], vals=[22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_04() {
        // remove all keys from leaf. Decrease height of tree/reset root.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=4 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(3);
        let _ = bt.remove(4);
        let expected = vec!["Node A=512, R=(+), L=(+), keys=[1, 2], vals=[11, 22], N:None"];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_05() {
        // remove all keys from leaf. Decrease height of tree/reset root.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=4 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(1);
        let _ = bt.remove(2);
        let expected = vec!["Node A=512, R=(+), L=(+), keys=[3, 4], vals=[33, 44], N:None"];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_06() {
        // remove keys from leaf. Rebalance from right sibling.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(2, 4);
        for i in 1..=8 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(4);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 3, 6], vals=[512, 1024, 2048], N:None",
            "Node A=2048, R=(-), L=(+), keys=[6, 7, 8], vals=[66, 77, 88], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 5], vals=[33, 55], N:Some(2048)",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_07() {
        // remove min_key from leaf. Rebalance from right sibling.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(2, 4);
        for i in 1..=8 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(3);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 4, 6], vals=[512, 1024, 2048], N:None",
            "Node A=2048, R=(-), L=(+), keys=[6, 7, 8], vals=[66, 77, 88], N:None",
            "Node A=1024, R=(-), L=(+), keys=[4, 5], vals=[44, 55], N:Some(2048)",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_08() {
        // remove key from leaf. Rebalance from left sibling.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(2, 4);
        for i in (1..=6).rev() {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(6);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 4], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[4, 5], vals=[44, 55], N:None",
            "Node A=512, R=(-), L=(+), keys=[1, 2, 3], vals=[11, 22, 33], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_09() {
        // remove min_key from leaf. Rebalance from left sibling.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(2, 4);
        for i in (1..=6).rev() {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(5);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 4], vals=[512, 1024], N:None",
            "Node A=1024, R=(-), L=(+), keys=[4, 6], vals=[44, 66], N:None",
            "Node A=512, R=(-), L=(+), keys=[1, 2, 3], vals=[11, 22, 33], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn remove_case_10() {
        // remove min_key from leaf. Rebalance from right sibling.
        log_init();
        let path = std::path::Path::new("test.idx");
        setup(&path);
        let bt = btree::Btree::new(std::path::Path::new("test.idx"), 512, 2, 64, 8);
        bt.set_degree(1, 3);
        for i in 1..=8 {
            let _ = bt.insert(i, i * 10 + i);
        }
        let _ = bt.remove(6);
        let _ = bt.remove(5);
        let expected = vec![
            "Node A=1536, R=(+), L=(-), keys=[1, 3, 7], vals=[512, 1024, 2048], N:None",
            "Node A=2048, R=(-), L=(+), keys=[7, 8], vals=[77, 88], N:None",
            "Node A=1024, R=(-), L=(+), keys=[3, 4], vals=[33, 44], N:Some(2048)",
            "Node A=512, R=(-), L=(+), keys=[1, 2], vals=[11, 22], N:Some(1024)",
        ];
        let result_string = bt.dump_to_string();
        let result: Vec<&str> = result_string.lines().collect();
        assert!(result.len() == expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert_eq!(r, e);
        }
    }
}
