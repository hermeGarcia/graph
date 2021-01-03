



type Node<NC> = Box<NC>;
type Edge<AC> = Box<AC>;
pub type NodeId = usize;
pub type AdjacencyList<AC> = Vec<Edge<AC>>;


#[derive(Clone)]
pub struct Graph<N,A> {
    nodes: Vec<Node<N>>,
    edges: Vec<AdjacencyList<A>>
}

impl<N,A> Default for Graph<N,A> {
    fn default() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new() }
    }
}

impl<N,A> Graph<N,A> {
    pub fn new() -> Graph<N,A> { Graph::default() }
    pub fn with(n_nodes: usize, n_edges: usize) -> Graph<N,A> {
        Graph {
            nodes: Vec::with_capacity(n_nodes),
            edges: Vec::with_capacity(n_edges)
        }
    }
    pub fn get_node(&self,id: NodeId) -> Option<&N> {
        if self.nodes.len() <= id {Option::None}
        else {Option::Some(&self.nodes[id])}
    }
    pub fn get_mut_node(&mut self,id: NodeId) -> Option<&mut N> {
        if self.nodes.len() <= id {Option::None}
        else {Option::Some(&mut self.nodes[id])}
    }
    pub fn get_edges_of(&self, id:  NodeId) -> Option<&AdjacencyList<A>> {
        if self.nodes.len() <= id {Option::None}
        else {Option::Some(&self.edges[id])}
    }
    pub fn get_mut_edges_of(&mut self, id:  NodeId) -> Option<&mut AdjacencyList<A>> {
        if self.nodes.len() <= id {Option::None}
        else {Option::Some(&mut self.edges[id])}
    }
    pub fn add_node(&mut self,value: N) -> NodeId {
        let node = Node::new(value);
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }
    pub fn add_edge(&mut self,to: NodeId,edge: A) {
        if self.nodes.len() > to { self.edges[to].push(Edge::new(edge)); }
    }
}




