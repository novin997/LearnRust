use std::collections::HashMap;
use std::hash::Hash;

// trait Graph<T> {
//     fn new(self) -> Self;
//     fn add_edge(self,src: T, dest: T) -> bool;
//     // fn remove_edge(src: T, dest: T) -> bool;
// }

pub struct Graph<T> {
    adj_list: HashMap<T, Vec<T>>,
}

impl<T: Eq + PartialEq + Hash> Graph<T> {
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: T, dest: T) -> bool {
        if let Some(x) = self.adj_list.get_mut(&src) {
            x.push(dest);
        } else {
            self.adj_list.insert(src, vec![dest]);
        }
        true
    }
}
