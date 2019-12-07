use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

pub trait Graph {
    type Node;
    type Edge;
    type Adjacents: IntoIterator<Item = (Self::Edge, Self::Node)>;

    fn adjacents(&self, node: &Self::Node) -> Self::Adjacents;
}

struct NodeHolder<Node, Edge> {
    node: Node,
    parent: Option<(Edge, NodeHolderRef<Node, Edge>)>,
}

impl<Node, Edge> NodeHolder<Node, Edge> {
    pub fn new(node: Node, parent: Option<(Edge, NodeHolderRef<Node, Edge>)>) -> Self {
        NodeHolder { node, parent }
    }
}

struct NodeHolderRef<Node, Edge>(Rc<NodeHolder<Node, Edge>>);

impl<Node, Edge> NodeHolderRef<Node, Edge> {
    fn weak(&self) -> Weak<NodeHolder<Node, Edge>> {
        Rc::downgrade(&self.0)
    }
}

impl<Node, Edge> Clone for NodeHolderRef<Node, Edge> {
    fn clone(&self) -> Self {
        NodeHolderRef(self.0.clone())
    }
}

impl<Node, Edge> PartialEq for NodeHolderRef<Node, Edge>
where
    Node: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.node == other.0.node
    }
}

impl<Node, Edge> Eq for NodeHolderRef<Node, Edge> where Node: Eq {}

impl<Node, Edge> Hash for NodeHolderRef<Node, Edge>
where
    Node: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.node.hash(state)
    }
}

impl<Node, Edge> Borrow<Node> for NodeHolderRef<Node, Edge> {
    fn borrow(&self) -> &Node {
        &self.0.node
    }
}

struct GraphExplorer<G: Graph>
where
    G::Node: Eq + Hash,
{
    cache: HashSet<NodeHolderRef<G::Node, G::Edge>>,
    queue: VecDeque<NodeHolderRef<G::Node, G::Edge>>,
}

impl<G: Graph> GraphExplorer<G>
where
    G::Node: Eq + Hash,
{
    fn new() -> GraphExplorer<G> {
        GraphExplorer {
            cache: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, n: NodeHolder<G::Node, G::Edge>) {
        let node_rc = NodeHolderRef(Rc::new(n));
        self.cache.insert(node_rc.clone());
        self.queue.push_back(node_rc);
    }
}

pub fn bfs<G, F>(graph: &G, initial_node: G::Node, is_done: F) -> Option<Vec<(G::Edge, G::Node)>>
where
    G: Graph,
    G::Node: Eq + Hash + Clone + Debug,
    G::Edge: Clone + Debug,
    F: Fn(&G::Node) -> bool,
{
    let mut explorer: GraphExplorer<G> = GraphExplorer::new();

    explorer.enqueue(NodeHolder::new(initial_node, None));
    while let Some(node_holder) = explorer.queue.pop_front() {
        if is_done(&node_holder.0.node) {
            //            println!("Explored states : {}", explorer.cache.len());
            return Some(backtrack(node_holder));
        }
        for (edge, next_node) in graph.adjacents(&node_holder.0.node) {
            if !explorer.cache.contains(&next_node) {
                explorer.enqueue(NodeHolder::new(
                    next_node,
                    Some((edge, node_holder.clone())),
                ));
            }
        }
    }
    None
}

fn backtrack<Node, Edge>(holder: NodeHolderRef<Node, Edge>) -> Vec<(Edge, Node)>
where
    Node: Clone,
    Edge: Clone,
{
    let mut track: Vec<(Edge, Node)> = Vec::new();
    let mut current = holder;
    while let Some(parent_ref) = current.0.parent.as_ref() {
        track.push((parent_ref.0.clone(), current.0.node.clone()));
        current = parent_ref.1.clone();
    }
    track.reverse();
    track
}
