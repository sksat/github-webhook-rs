use std::collections::{HashMap, HashSet};

pub type Node<'a> = &'a str;

struct DAG<'a> {
    nodes: HashSet<Node<'a>>,
    edges: HashMap<Node<'a>, Vec<Node<'a>>>,
}

impl<'a> DAG<'a> {
    fn new() -> Self {
        DAG {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: Node<'a>, to: Node<'a>) {
        self.nodes.insert(from);
        self.nodes.insert(to);
        self.edges.entry(from).or_insert(Vec::new()).push(to);
    }

    fn topo_sort(&self) -> Vec<Node<'a>> {
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
            if let Some(children) = self.edges.get(node) {
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

pub struct CoDAG<'a> {
    dag: DAG<'a>,
}

impl<'a> CoDAG<'a> {
    pub fn new() -> Self {
        Self { dag: DAG::new() }
    }

    pub fn add_edge(&mut self, from: Node<'a>, to: Node<'a>) {
        self.dag.add_edge(to, from);
    }

    pub fn topo_sort(&self) -> Vec<Node<'a>> {
        self.dag.topo_sort()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topo_sort() {
        let mut dag = DAG::new();
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

        let topo_order = dag.topo_sort();
        assert_eq!(topo_order, vec!["E", "D", "C", "B", "A"]);
    }
}
