use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};
use uuid::Uuid;

pub struct CircularDependancyError;

impl fmt::Debug for CircularDependancyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in toposort - circular dependency")
    }
}

pub fn toposort(
    input: &HashMap<Uuid, HashSet<Uuid>>,
) -> Result<Vec<Uuid>, CircularDependancyError> {
    if input.is_empty() {
        Ok(Vec::new())
    } else {
        let mut in_degree: HashMap<Uuid, i32> = HashMap::new();

        for (key, val) in input.iter() {
            let _ = *in_degree.entry(*key).or_insert(0);
            for adj in val {
                *in_degree.entry(*adj).or_insert(0) += 1;
            }
        }

        let mut deque: VecDeque<Uuid> = VecDeque::new();

        for (key, val) in in_degree.iter() {
            if *val == 0 {
                deque.push_back(*key);
            }
        }

        let mut ret: Vec<Uuid> = Vec::new();

        while let Some(node) = deque.pop_front() {
            ret.push(node);

            for adj_node in input[&node].iter() {
                *in_degree.entry(*adj_node).or_insert(0) -= 1;

                if in_degree[adj_node] == 0 {
                    deque.push_back(*adj_node);
                }
            }
        }

        if ret.len() > input.len() {
            return Err(CircularDependancyError);
        }

        Ok(ret)
    }
}
