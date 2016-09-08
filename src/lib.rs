use std::fmt::Debug;

struct Entry<K, V> {
    key: K,
    val: V,
}

struct CTree<K: Copy + PartialOrd, V> {
    entries: [Option<Entry<K, V>>; 2],
    children: [Option<Box<CTree<K, V>>>; 3],
}

impl<K, V> CTree<K, V> where K: Debug + Copy + PartialOrd {
    fn new() -> CTree<K, V> {
        CTree {
            entries: [None, None],
            children: [None, None, None],
        }
    }

    fn find_node_for_key(&self, query: &K) -> &Self {
        let mut idx = 0;
        for entry in self.entries.iter() {
            if entry.is_none() {
                break
            }

            let &Entry { ref key, ref val } = entry.as_ref().unwrap();
            if *query == *key {
                return &self
            } else if *query < *key {
                return match self.children[idx].as_ref() {
                    Some(ctree) => ctree.find_node_for_key(query),
                    None => &self
                }
            }

            idx += 1
        }

        if let Some(ref child) = self.children[idx].as_ref() {
            child.find_node_for_key(query)
        } else {
            &self
        }
    }

    fn search<'a>(&'a self, query: &K) -> Option<&'a V> {
        let node = self.find_node_for_key(query);
        if let Some(idx) = self.find_key_in_entries(query, &node.entries) {
            Some(&node.entries[idx].as_ref().unwrap().val)
        } else {
            None
        }
    }

    fn find_key_in_entries(&self, query: &K, entries: &[Option<Entry<K, V>>; 2]) -> Option<usize> {
        for (i, entry) in entries.iter().enumerate() {
            if entry.is_none() {
                return None
            }
            let &Entry { ref key, ref val } = entry.as_ref().unwrap();
            if *query == *key {
                return Some(i);
            }
        }
        None
    }

    fn insert(&self, key: K, val: V) {
        let mut node = self.find_node_for_key(&key);
        if node.entries[node.entries.len() - 1].is_some() {
            // split
                    unimplemented!();
        } else {
            match self.find_key_in_entries(&key, &node.entries) {
                Some(idx) => {
                    node.entries[idx] = Some(Entry{key: key, val: val});
                },
                None => {
                    // shifting
                    unimplemented!();
                }
            }
        }
    }

    fn delete(&mut self, key: &K) {
        unimplemented!()
    }
}

#[test]
fn test_search() {
    let mut ctree = CTree::new();
    let mut ctree2 = Box::new(CTree::new());
    ctree.entries[0] = Some(Entry{key:5, val:20});
    ctree2.entries[0] = Some(Entry{key:4, val:10});
    ctree.children[0] = Some(ctree2);
    assert_eq!(ctree.search(&5), Some(&20));
    assert_eq!(ctree.search(&6), None);
}

#[test]
fn test_recursive_search() {
    let mut ctree = CTree::new();
    let mut ctree2 = Box::new(CTree::new());
    ctree.entries[0] = Some(Entry{key:5, val:20});
    ctree2.entries[0] = Some(Entry{key:4, val:10});
    ctree.children[0] = Some(ctree2);
    assert_eq!(ctree.search(&5), Some(&20));
    assert_eq!(ctree.search(&4), Some(&10));
}

#[test]
fn test_insert() {
    let mut ctree = CTree::new();
    ctree.insert(5, 20);
    assert_eq!(ctree.search(&5), Some(&20));
}
