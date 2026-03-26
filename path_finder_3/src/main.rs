#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

use std::collections::HashSet;

struct Node {
    location: (usize, usize),
    index: usize,
    edges: Vec<usize>, // indicies of edges located in same graph
    visited: bool,
    cost: u32,
}
impl Node {
    fn new(location: (usize, usize), index: usize) -> Self {
        Self {
            location,
            index,
            edges: Vec::new(),
            visited: false,
            cost: 0,
        }
    }
}

struct Edge {
    from: usize, // index of Node in graph
    to: usize, // index of Node in graph
    cost: u32,
}

impl Edge {
    fn new(from: usize, to: usize, cost: u32) -> Self {
        Self {
            from,
            to,
            cost,
        }
    }
}

struct Graph {
    matrix: Vec<Vec<usize>>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    visited_nodes: HashSet<usize>,
    not_visited: HashSet<usize>,
    open_edges: HashSet<usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            matrix: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            visited_nodes: HashSet::new(),
            not_visited: HashSet::new(),
            open_edges: HashSet::new(),
        } 
    }
    fn visit(self: &mut Self, irow: usize, icol: usize, cost: u32) ->() {
        let node_index = self.matrix[irow][icol];
        self.visited_nodes.insert(node_index);
        self.not_visited.remove(&node_index);
        self.nodes[node_index].visited = true;
        self.nodes[node_index].cost = cost;
        self.add_edges_for_node(node_index);
    }
    fn add_edges_for_node(self: &mut Self, node_index: usize) -> () {
        let node: &Node = &self.nodes[node_index];
        for edge_index in &node.edges {
            let edge: &Edge = &self.edges[*edge_index];
            let node1_visited: bool = self.nodes[edge.from].visited;
            let node2_visited: bool = self.nodes[edge.to].visited;
            if node1_visited != node2_visited {
                self.open_edges.insert(*edge_index);
            }
        }
        self.maintain_open_edges();
    }
    fn maintain_open_edges(self: &mut Self) -> () {
  
        let mut new_edges: HashSet<usize> = HashSet::new();
        for edge_index in &self.open_edges {
            if self.is_crossover_edge(*edge_index) {
                new_edges.insert(*edge_index);
            }
        }
        self.open_edges = new_edges;
    }
    /* Return true iff one endpoint is in visited nodes
     * and the other enpoint is not in visited nodes
     */
    fn is_crossover_edge(self: &Self, edge_index: usize) -> bool {
        let edge: &Edge = &self.edges[edge_index];
        let node1_visited: bool = self.nodes[edge.from].visited;
        let node2_visited: bool = self.nodes[edge.to].visited;
        return node1_visited != node2_visited;
    }
}



fn path_finder(area: &[Vec<u32>]) -> u32 {
    // code here
    println!("");
    print_area(area);
    let mut g: Graph = make_graph(area);
    //print_matrix(&g);
    make_edges(&mut g, area);

    let best_cost = find_paths(&mut g);

    //
    best_cost
}

fn find_paths(g: &mut Graph) -> u32 {
    /* Find the pathes from the top left corner to the bottom right corner of the graph, 
    and return the cost of the cheapest path, using Dijkstra's algorithm.
    */
    // Mark the starting node a (0,0) as visited.
    g.visit(0, 0, 0); 

    let n = g.matrix.len();
    let target_node_index = g.matrix[n-1][n-1];

    loop {
        // If the bottom right corner is visited, we are done.
        if g.visited_nodes.contains(&target_node_index) {
            break;
        }

        // Otherwise, find the open edge with the lowest cost, and visit the node at the end of that edge.
        let mut best_edge_index: Option<usize> = None;
        let mut best_cost: Option<u32> = None;
        for edge_index in &g.open_edges {
            let from_node_index: usize = g.edges[*edge_index].from;
            let to_node_index: usize = g.edges[*edge_index].to;
            let closed_node_index: usize;
            if g.nodes[from_node_index].visited {
                closed_node_index = from_node_index;
            } else {
                closed_node_index = to_node_index
            }
            let trial_cost: u32 = g.nodes[closed_node_index].cost + g.edges[*edge_index].cost;
            if best_cost.is_none() || trial_cost < best_cost.unwrap() {
                best_cost = Some(trial_cost);
                best_edge_index = Some(*edge_index);
            }
        }
        let best_edge: &Edge = &g.edges[best_edge_index.unwrap()];
        let node_to_visit: usize;
        if g.nodes[best_edge.from].visited {
            node_to_visit = best_edge.to;
        } else {
            node_to_visit = best_edge.from;
        }
        let node_cost: u32 = best_cost.unwrap();
        g.visit(g.nodes[node_to_visit].location.0, g.nodes[node_to_visit].location.1, node_cost);
        
    }

    if g.nodes[target_node_index].visited {
        return g.nodes[target_node_index].cost;
    } else {
        panic!("Target node not found");
    }
}

fn make_graph(area: &[Vec<u32>]) -> Graph {
    let mut g: Graph = Graph::new();
    for i in 0..area.len() {
        let mut graph_row: Vec<usize> = Vec::new();
        for j in 0..area.len() {
            let node_index = g.nodes.len();
            g.not_visited.insert(node_index);
            let node = Node::new((i, j), node_index);
            graph_row.push(node_index);
            g.nodes.push(node);
        }
        g.matrix.push(graph_row);
    }
    g
}

fn make_edges(g: &mut Graph, area: &[Vec<u32>]) -> () {
    // area is an n by n square matrix of Vec.
    let n = area.len();
    let n_minus_one = n - 1;

    // Create all the horizontal edges across rows,
    // linking adjacent notes.
    for irow in 0..n {
        for col1 in 0..n_minus_one {
            let col2 = col1 + 1;
            let height1: i32 = area[irow][col1] as i32;
            let height2: i32 = area[irow][col2] as i32;
            let cost: u32 = (height1 - height2).abs() as u32;
            let edge_index = g.edges.len();
            let node1_index = g.matrix[irow][col1];
            let node2_index = g.matrix[irow][col2];
            let edge = Edge::new(node1_index, node2_index, cost);
            g.edges.push(edge);
            g.nodes[node1_index].edges.push(edge_index);
            g.nodes[node2_index].edges.push(edge_index);
        }
    }

    // Create all the vertical edges down columns,
    // linking adjacent nodes.
    for irow in 0..n_minus_one {
        let irow2 = irow + 1;
        for col in 0..n {
            let height1: i32 = area[irow][col] as i32;
            let height2: i32 = area[irow2][col] as i32;
            let cost: u32 = (height1 - height2).abs() as u32;
            let edge_index = g.edges.len();
            let node1_index = g.matrix[irow][col];
            let node2_index = g.matrix[irow2][col];
            let edge = Edge::new(node1_index, node2_index, cost);
            g.edges.push(edge);
            g.nodes[node1_index].edges.push(edge_index);
            g.nodes[node2_index].edges.push(edge_index);
        }
    }       

}

fn print_area(matrix: &[Vec<u32>]) -> () {
    for row in matrix {
        println!("{:?}", row);
    }
}

fn print_matrix(g: &Graph) -> () {
    for row in &g.matrix {
        println!("{:?}", row); 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_equal(input: &[Vec<u32>], actual: u32, expected: u32) {
        assert_eq!(actual, expected, "\nFor the input: {:?}\nYour result (left) did not match the expected output (right)", input);
    }

    #[test]
    fn test_basic() {
        let area: Vec<Vec<u32>> = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0]
        ];
        test_equal(&area,path_finder(&area), 0);

        let area: Vec<Vec<u32>> = vec![
            vec![0, 1, 0],
            vec![0, 1, 0],
            vec![0, 1, 0]
        ];
        test_equal(&area,path_finder(&area), 2);

        let area: Vec<Vec<u32>> = vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0]
        ];
        test_equal(&area,path_finder(&area), 4);

        let area: Vec<Vec<u32>> = vec![
            vec![0, 7, 0, 7],
            vec![7, 0, 7, 0],
            vec![0, 7, 0, 7],
            vec![7, 0, 7, 0]
        ];
        test_equal(&area,path_finder(&area), 42);

        let area: Vec<Vec<u32>> = vec![
            vec![7, 0, 0, 0, 0, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 0, 0, 0, 0, 7]
        ];
        test_equal(&area,path_finder(&area), 14);

        let area: Vec<Vec<u32>> = vec![
            vec![7, 7, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 7, 7, 7]
        ];
        test_equal(&area,path_finder(&area), 0);

        let area: Vec<Vec<u32>> = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 1, 0, 9],
            vec![0, 0, 1, 0, 1, 0]
        ];
        test_equal(&area,path_finder(&area), 4);
    }
}
