use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct DirectedAcyclicGraph<Node> {
    nodes: HashSet<Node>,
    edges: HashMap<Node, Vec<Node>>,
}

impl<Node: Copy + Hash + Eq + std::fmt::Debug> DirectedAcyclicGraph<Node> {
    fn new() -> Self {
        DirectedAcyclicGraph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: Node, to: Node) {
        self.nodes.insert(from);
        self.nodes.insert(to);
        self.edges.entry(from).or_default().push(to);
    }

    fn topo_sort(&self) -> Result<Vec<Node>, Vec<Node>> {
        let mut in_degree = HashMap::new();
        for node in self.nodes.iter() {
            in_degree.insert(*node, 0);
        }
        for child in self.edges.values().flatten() {
            *in_degree.entry(*child).or_insert(0) += 1;
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
                for child in children {
                    let m = in_degree.get_mut(child).unwrap();
                    *m -= 1;
                    if *m == 0 {
                        queue.push(*child);
                    }
                }
            }
        }
        #[cfg(debug_assertions)]
        if in_degree.values().any(|&i| i != 0) {
            // the graph has cycle
            return Err(self.find_cycle());
        }
        Ok(result)
    }

    #[cfg(debug_assertions)]
    fn find_cycle(&self) -> Vec<Node> {
        let mut visited = HashSet::new();
        let mut seen = HashSet::new();

        for node in self.nodes.iter() {
            if visited.contains(node) {
                continue;
            }
            enum Traverse<N> {
                Pre(N),
                Post,
            }
            use Traverse::*;

            let mut stack = vec![Pre(node)];
            let mut path = Vec::new();
            seen.clear();

            while let Some(t) = stack.pop() {
                match t {
                    Pre(current) => {
                        path.push(current);
                        seen.insert(current);

                        stack.push(Post);
                        if let Some(children) = self.edges.get(current) {
                            for child in children {
                                if !seen.contains(child) {
                                    stack.push(Pre(child));
                                } else if let Some(cycle_start) =
                                    path.iter().rposition(|&&x| x == *child)
                                {
                                    return path[cycle_start..].iter().map(|&&x| x).collect();
                                }
                            }
                        }
                    }
                    Post => {
                        visited.insert(path.pop().unwrap());
                    }
                }
            }
        }
        unreachable!("the graph has no cycle")
    }
}

pub struct CoDirectedAcyclicGraph<Node> {
    dag: DirectedAcyclicGraph<Node>,
}

impl<Node: Copy + Hash + Eq + std::fmt::Debug> CoDirectedAcyclicGraph<Node> {
    pub fn new() -> Self {
        Self {
            dag: DirectedAcyclicGraph::new(),
        }
    }

    pub fn add_edge(&mut self, from: Node, to: Node) {
        self.dag.add_edge(to, from);
    }

    pub fn co_topo_sort(&self) -> Result<Vec<Node>, Vec<Node>> {
        self.dag.topo_sort()
    }
}

impl<Node: Copy + Hash + Eq + std::fmt::Debug> Default for CoDirectedAcyclicGraph<Node> {
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

        let topo_order = dag.topo_sort().unwrap();
        assert_eq!(topo_order, vec!["A", "C", "B", "D", "E"]);
    }

    #[test]
    fn test_co_topo_sort() {
        let mut dag = CoDirectedAcyclicGraph::new();
        dag.add_edge("A", "B");
        dag.add_edge("A", "C");
        dag.add_edge("B", "D");
        dag.add_edge("C", "D");
        dag.add_edge("C", "E");
        dag.add_edge("D", "E");

        let topo_order = dag.co_topo_sort().unwrap();
        assert_eq!(topo_order, vec!["E", "D", "C", "B", "A"]);
    }

    #[test]
    fn test_find_cycle() {
        let mut graph = DirectedAcyclicGraph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 2);

        let cycle = graph.find_cycle();
        assert!(
            [vec![2, 3, 4], vec![3, 4, 2], vec![4, 2, 3],].contains(&cycle),
            "{cycle:?}"
        );

        graph.add_edge(1, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 5);
        let cycle = graph.find_cycle();
        assert!(
            [vec![2, 3, 4], vec![3, 4, 2], vec![4, 2, 3],].contains(&cycle),
            "{cycle:?}"
        );
    }
}
