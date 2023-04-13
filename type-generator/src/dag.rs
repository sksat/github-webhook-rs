use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct DirectedAcyclicGraph<Node> {
    nodes: HashSet<Node>,
    edges: HashMap<Node, Vec<Node>>,
}

impl<Node: Copy + Hash + Eq> DirectedAcyclicGraph<Node> {
    fn new() -> Self {
        DirectedAcyclicGraph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        self.nodes.insert(from);
        self.nodes.insert(to);
        self.edges.entry(from).or_insert(Vec::new()).push(to);
    }

    fn topo_sort(&self) -> Vec<Node> {
        let mut in_degree = HashMap::new();
        for node in self.nodes.iter() {
            in_degree.insert(*node, 0);
        }
        for children in self.edges.values() {
            for child in children {
                *in_degree.entry(*child).or_insert(0) += 1;
            }
        }
        let mut queue: Vec<_> = in_degree
            .iter()
            .filter_map(
                |(node, degree)| {
                    if *degree == 0 {
                        Some(*node)
                    } else {
                        None
                    }
                },
            )
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop() {
            result.push(node);
            if let Some(children) = self.edges.get(&node) {
                for child in children.iter() {
                    let m = in_degree.get_mut(child).unwrap();
                    *m -= 1;
                    if *m == 0 {
                        queue.push(*child);
                    }
                }
            }
        }
        result
    }
}

pub struct CoDAG<Node> {
    dag: DirectedAcyclicGraph<Node>,
}

impl<Node: Copy + Hash + Eq> CoDAG<Node> {
    pub fn new() -> Self {
        Self {
            dag: DirectedAcyclicGraph::new(),
        }
    }

    pub fn add_edge(&mut self, from: Node, to: Node) {
        self.dag.add_edge(to, from);
    }

    pub fn co_topo_sort(&self) -> Vec<Node> {
        self.dag.topo_sort()
    }
}

impl<Node: Copy + Hash + Eq> Default for CoDAG<Node> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort() {
        let mut dag = DirectedAcyclicGraph::new();
        dag.add_edge("A", "B");
        dag.add_edge("A", "C");
        dag.add_edge("B", "D");
        dag.add_edge("C", "D");
        dag.add_edge("C", "E");
        dag.add_edge("D", "E");

        let topo_order = dag.topo_sort();
        assert_eq!(topo_order, vec!["A", "C", "B", "D", "E"]);
    }

    #[test]
    fn test_co_topo_sort() {
        let mut dag = CoDAG::new();
        dag.add_edge("A", "B");
        dag.add_edge("A", "C");
        dag.add_edge("B", "D");
        dag.add_edge("C", "D");
        dag.add_edge("C", "E");
        dag.add_edge("D", "E");

        let topo_order = dag.co_topo_sort();
        assert_eq!(topo_order, vec!["E", "D", "C", "B", "A"]);
    }
}
