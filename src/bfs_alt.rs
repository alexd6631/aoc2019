use crate::bfs::Graph;
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use std::any::Any;

type Index = i32;

pub fn bfs_alt<G, F>(graph: &G, initial_node: G::Node, mut is_done: F)
    -> Option<Vec<G::Edge>>
    where
        G: Graph,
        G::Node: Eq + Hash + Clone,
        G::Edge: Clone + Debug,
        F: FnMut(&G, &G::Node) -> bool,
{


    let mut visited: HashSet<G::Node> = HashSet::new();
    let mut tracks: Vec<(G::Edge, Index)> = Vec::new();
    let mut queue: VecDeque<(G::Node, Index)> = VecDeque::new();

    visited.insert(initial_node.clone());
    queue.push_back((initial_node, -1));

    while let Some((node, index)) = queue.pop_front() {
        if is_done(graph, &node) {
            let track = Some(backtrack(index, tracks));
            return track;
        } else {
            for (edge, next_node) in graph.adjacents(&node) {
                if !visited.contains(&next_node) {
                    let next_index = tracks.len() as Index;
                    visited.insert(next_node.clone());
                    tracks.push((edge, index));
                    queue.push_back((next_node, next_index));
                }
            }
        }

    }
    None
}

fn backtrack<Edge>(index: Index, tracks: Vec<(Edge, Index)>) -> Vec<Edge>
    where
        Edge: Clone
{
    let mut track: Vec<Edge> = Vec::new();
    let mut current = index;
    while current != -1 {
        let (edge, parent) = &tracks[current as usize];
        track.push(edge.clone());
        current = *parent;
    }
    track.reverse();
    track
}


