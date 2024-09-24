/* use sqlx::FromRow;
use std::collections::HashMap;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances = HashMap::new();
        distances.insert(from_node_id, 0);

        for _ in 0..self.nodes.len() {
            for node_id in self.nodes.keys() {
                if let Some(edges) = self.edges.get(node_id) {
                    for edge in edges {
                        let new_distance = distances
                            .get(node_id)
                            .and_then(|d: &i32| d.checked_add(edge.weight))
                            .unwrap_or(i32::MAX);
                        let current_distance = distances.get(&edge.node_b_id).unwrap_or(&i32::MAX);
                        if new_distance < *current_distance {
                            distances.insert(edge.node_b_id, new_distance);
                        }
                    }
                }
            }
        }

        distances.get(&to_node_id).cloned().unwrap_or(i32::MAX)
    }
}

// use std::collections::{HashMap, BinaryHeap};
// use std::cmp::Ordering;

// #[derive(Debug)]
// struct Edge {
//     node_b_id: i32,
//     weight: i32,
// }

// #[derive(Debug)]
// struct NodeDistance {
//     node_id: i32,
//     distance: i32,
// }

// impl Ord for NodeDistance {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.distance.cmp(&self.distance) // 反向排序，最小距离优先
//     }
// }

// impl PartialOrd for NodeDistance {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for NodeDistance {
//     fn eq(&self, other: &Self) -> bool {
//         self.node_id == other.node_id && self.distance == other.distance
//     }
// }

// impl Eq for NodeDistance {}

// pub struct Graph {
//     edges: HashMap<i32, Vec<Edge>>,  // 邻接表表示的图
// }

// impl Graph {

//     pub fn new() -> Self {
//         Graph {
//             nodes: HashMap::new(),
//             edges: HashMap::new(),
//         }
//     }

//     pub fn add_node(&mut self, node: Node) {
//         self.nodes.insert(node.id, node);
//     }

//     pub fn add_edge(&mut self, edge: Edge) {
//         self.edges
//             .entry(edge.node_a_id)
//             .or_default()
//             .push(edge.clone());

//         let reverse_edge = Edge {
//             node_a_id: edge.node_b_id,
//             node_b_id: edge.node_a_id,
//             weight: edge.weight,
//         };
//         self.edges
//             .entry(reverse_edge.node_a_id)
//             .or_default()
//             .push(reverse_edge);
//     }

//     pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> Result<i32, String> {
//         let mut distances = HashMap::new();
//         let mut heap = BinaryHeap::new();

//         distances.insert(from_node_id, 0);
//         heap.push(NodeDistance { node_id: from_node_id, distance: 0 });

//         while let Some(NodeDistance { node_id, distance }) = heap.pop() {
//             if distance > *distances.get(&node_id).unwrap_or(&i32::MAX) {
//                 continue;
//             }

//             if let Some(edges) = self.edges.get(&node_id) {
//                 for edge in edges {
//                     let new_distance = distance + edge.weight;
//                     if new_distance < *distances.get(&edge.node_b_id).unwrap_or(&i32::MAX) {
//                         distances.insert(edge.node_b_id, new_distance);
//                         heap.push(NodeDistance { node_id: edge.node_b_id, distance: new_distance });
//                     }
//                 }
//             }
//         }

//         distances.get(&to_node_id).cloned().ok_or_else(|| "Path not found".to_string())
//     }
// }
 */
use sqlx::FromRow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    // Optimized shortest path using Dijkstra's algorithm
    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut distances: HashMap<i32, i32> =
            self.nodes.keys().map(|&id| (id, i32::MAX)).collect();
        let mut heap = BinaryHeap::new();

        distances.insert(from_node_id, 0);
        heap.push(State {
            cost: 0,
            position: from_node_id,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == to_node_id {
                return cost; // We found the shortest path
            }

            if cost > *distances.get(&position).unwrap_or(&i32::MAX) {
                continue;
            }

            if let Some(edges) = self.edges.get(&position) {
                for edge in edges {
                    let next = State {
                        cost: cost + edge.weight,
                        position: edge.node_b_id,
                    };

                    if next.cost < *distances.get(&next.position).unwrap_or(&i32::MAX) {
                        heap.push(next);
                        distances.insert(next.position, next.cost);
                    }
                }
            }
        }

        *distances.get(&to_node_id).unwrap_or(&i32::MAX)
    }
}

// Custom struct to manage heap ordering by distance
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
