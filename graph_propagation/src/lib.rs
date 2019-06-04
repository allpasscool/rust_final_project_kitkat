extern crate rand;

// use rand::{thread_rng, Rng};
use rand::Rng;
use rs_graph_derive::Graph;
use rs_graph::{traits::*};
use rs_graph::linkedlistgraph::*;
// use rs_graph::classes;
// use rs_graph::attributes::{NodeAttributes, EdgeAttributes, AttributedGraph};
use rs_graph::attributes::{NodeAttributes, EdgeAttributes};
use rs_graph::builder::*;
use crate::Builder;
// use crate::num::iter::{range, range_step, Range, RangeStep};
// use graph_propagation;

pub struct Node(pub rs_graph::linkedlistgraph::Node);

pub struct Edge(pub rs_graph::linkedlistgraph::Edge);

impl Clone for Node{
    fn clone(&self) -> Node{
        Node(self.0.clone())
    }
}

impl Clone for Edge{
    fn clone(&self) -> Edge{
        Edge(self.0.clone())
    }
}

///MyGraph
///This is an undirected graph
#[derive(Graph)]
pub struct Graph {
    #[graph] pub graph: LinkedListGraph::<u32, MyNodeData, MyEdgeData, ()>,
    #[nodeattrs(MyNodeData)] pub nodedata: Vec<MyNodeData>,
    #[edgeattrs(MyEdgeData)] pub edgedata: Vec<MyEdgeData>,
    pub propagation_model: PropagationModel,
    pub seed: Vec<usize>,
    pub next_to_propagate: Vec<usize>,
}

pub enum PropagationModel{
    LT, //linear threshold
    IC, //independent cascade
}

pub enum SeedSelection{
    MaxDegree,
    MinDegree,
    Random,
}

pub enum ThresholdSet{
    Random,
    Baseline(f64),
}

pub enum WeightSet{
    OneOverOutdegree,   // 1 / outdegree
    Random,             // random
    Equal(f64),         //equal
}

#[derive(Clone, Default)]
pub struct MyNodeData {
    pub label: usize,
    pub threshold: f64,
    // pub influence: f64,
}

#[derive(Clone, Default)]
pub struct MyEdgeData {
    pub from: usize,
    pub to: usize,
    pub label_1t2: usize,
    pub weight_1t2: f64,
    pub label_2t1: usize,
    pub weight_2t1: f64,
    pub reverse_edge: usize,
}

// supported data file
//pub const supported_file: &'static [&'static str] = &["ego-Facebook"];

/// An iterator over all nodes of a linked list graph.
// pub struct NodeIter1<u32>(pub Range<u32>);

// impl<u32> Iterator for NodeIter1<u32>
// {
//     type Item = rs_graph::linkedlistgraph::Node<ID>;

//     fn next(&mut self) -> Option<Node(Self::Item)> {
//         self.0.next().map(Node)
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.0.size_hint()
//     }
// }


impl Graph{
    /*
    //build a graph from open data
    pub fn build_graph_from_famous_open_data(file_name: &str, file_loc: &str){
        let read_file = false;
        
        //get it from supported_file
        //set read_file = true
        //TODO

        //read file
        //TODO
        let file_name = String::from(file_name);
        if read_file{
            match file_name.as_ref(){
                "ego-Facebook" => (),
                &_ => (),
            };
        }
    }
    */

    /// Creates a new `Graph` for a given model.
    pub fn new(propa_model: PropagationModel) -> Graph{
        Graph{
            graph: rs_graph::linkedlistgraph::LinkedListGraph::<u32, MyNodeData, MyEdgeData, ()>::new(),
            nodedata: Vec::new(),
            edgedata: Vec::new(),
            propagation_model: propa_model,
            seed: Vec::new(),
            next_to_propagate: Vec::new(),
        }
    }

    //TODO
    // pub fn get_node(){

    // }

    //TODO
    // pub fn get_edge(){
    
    // }

    /// Looks up a `Node` by its ID.
    pub fn get_id2node(&self, id: usize) -> Node{
        Node(self.graph.id2node(id))
    }

    /// Looks up an `Edge` by its ID.
    pub fn get_id2edge(&self, id: usize) -> Edge{
        Edge(self.graph.id2edge(id))
    }

    ///get all nodes
    // pub fn get_nodes(&mut self) -> rs_graph::linkedlistgraph::NodeIter<u32>{
    //     self.graph.nodes()
    // }
    // pub fn get_nodes(&mut self) -> rs_graph::linkedlistgraph::NodeIter<u32>{
    pub fn get_nodes(&self) -> std::vec::IntoIter<Node>{
        let node_iter: Vec<Node> = self.graph.nodes().map(|a| Node(a)).collect();
        let node_iter = node_iter.into_iter();
        node_iter
    }

    ///get all edges
    // pub fn get_edges(&mut self) -> rs_graph::linkedlistgraph::EdgeIter<u32>{
    pub fn get_edges(&self) -> std::vec::IntoIter<Edge>{
        let edge_iter: Vec<Edge> = self.graph.edges().map(|a| Edge(a)).collect();
        let edge_iter = edge_iter.into_iter();
        edge_iter
    }

    ///Return the number of nodes in the graph.
    pub fn get_nodes_number(&self) -> usize{
        self.graph.num_nodes()
    }

    ///Return the number of edges in the graph.
    pub fn get_edges_number(&self) -> usize{
        self.graph.num_edges()
    }

    ///get node id
    pub fn get_node_id(&self, n: Node) -> usize{
        self.graph.node_id(n.0)
    }

    ///get edge id
    pub fn get_edge_id(&self, e: Edge) -> usize{
        self.graph.edge_id(e.0)
    }

    ///given edge, get node from and node to
    pub fn get_edge_nodes(&self, e : Edge) -> (Node, Node){
        (self.get_id2node(self.graph.edge(e.0).from), self.get_id2node(self.graph.edge(e.0).to))
    }

    ///get neighbors
    /// (edge, node) in neighiter
    pub fn get_neighbors(&self, n: Node) -> std::vec::IntoIter<(Edge, Node)>{
        let neightbors: Vec<(Edge, Node)> = self.graph.neighs(n.0).map(|(a, b)| (Edge(a), Node(b))).collect();
        neightbors.into_iter()
    }

    ///get outdegrees
    pub fn get_outdegrees(&self, n: Node) -> usize{
        let mut count = 0;
        for (_edge, _node) in self.graph.neighs(n.0){
            // println!("neighbor node id: {:?} neighbor edge id: {:?}", node, edge);
            count += 1;
        }
        count / 2
    }

    ///get node threshold
    pub fn get_node_threshold(&self, n: Node) -> f64{
        self.graph.node(n.0).threshold
    }

    /*
    ///get outedges
    pub fn get_outedges_and_nodes(&mut self, n: rs_graph::linkedlistgraph::Node) -> rs_graph::linkedlistgraph::NeighIter{
        self.graph.neighs(n)
    }
    */
    

    ///get node label
    pub fn get_node_label(&self, n: Node) -> usize{
        self.graph.node(n.0).label
    }

    ///get edge label 1t2
    ///if we can visualize the graph, we will need labels for edges. keep this one for the future
    pub fn get_edge_label(&self, e: Edge) -> usize{
        self.graph.edge(e.0).label_1t2
    }

    ///get edge weight 1t2
    pub fn get_edge_weight(&self, e: Edge) -> f64{
        self.graph.edge(e.0).weight_1t2
    }
    
    ///add new node with attribute
    pub fn add_node(&mut self, data: MyNodeData) -> Node{
        let new_node = self.graph.add_node();
        self.graph.node_mut(new_node).label = data.label;
        self.graph.node_mut(new_node).threshold = data.threshold;
        // self.graph.node_mut(new_node).influence = data.influence;
        Node(new_node)
    }

    ///add edge
    pub fn add_edge(&mut self, n1: Node, n2: Node, data: MyEdgeData) -> Edge{
        let new_edge = self.graph.add_edge(n2.0, n1.0);
        self.graph.edge_mut(new_edge).from = self.graph.node_id(n2.0);
        self.graph.edge_mut(new_edge).to = self.graph.node_id(n1.0);
        self.graph.edge_mut(new_edge).label_1t2 = data.label_2t1;
        self.graph.edge_mut(new_edge).weight_1t2 = data.weight_2t1;
        self.graph.edge_mut(new_edge).label_2t1 = data.label_1t2;
        self.graph.edge_mut(new_edge).weight_2t1 = data.weight_1t2;
        
        let new_edge1 = self.graph.add_edge(n1.0, n2.0);
        self.graph.edge_mut(new_edge1).from = self.graph.node_id(n1.0);
        self.graph.edge_mut(new_edge1).to = self.graph.node_id(n2.0);
        self.graph.edge_mut(new_edge1).label_1t2 = data.label_1t2;
        self.graph.edge_mut(new_edge1).weight_1t2 = data.weight_1t2;
        self.graph.edge_mut(new_edge1).label_2t1 = data.label_2t1;
        self.graph.edge_mut(new_edge1).weight_2t1 = data.weight_2t1;

        self.graph.edge_mut(new_edge).reverse_edge = self.graph.edge_id(new_edge1);
        self.graph.edge_mut(new_edge1).reverse_edge = self.graph.edge_id(new_edge);

        Edge(new_edge1)
    }

    ///initialize node label to 0
    pub fn initialize_node_label(&mut self){
        let nodes = self.graph.nodes();

        for i in nodes{
            self.graph.node_mut(i).label = 0;
        }
    }

    ///initialize nodes' threshold to random between 0 and 1, or to a baseline
    pub fn initialize_node_threshold(&mut self, set: ThresholdSet){
        let nodes = self.graph.nodes();
        for i in nodes{
            match set{
                ThresholdSet::Random => self.graph.node_mut(i).threshold = rand::thread_rng().gen(),
                ThresholdSet::Baseline(baseline1) => self.graph.node_mut(i).threshold = baseline1,
            }
        }
    }

    ///initialize edge label to 0
    pub fn initialize_edge_label(&mut self){
        let edges = self.graph.edges();

        for i in edges{
            self.graph.edge_mut(i).label_1t2 = 0;
            self.graph.edge_mut(i).label_2t1 = 0;
        }

    }

    ///initialized edge weight which means n1 to n2, is directed
    pub fn initialize_weight(&mut self, way: WeightSet){
        match way{
            WeightSet::Random => {
                let edges = self.graph.edges();
                let mut count = 0;

                for i in edges{
                    if count % 2 == 1{
                        count += 1;
                        continue;
                    }

                    count += 1;

                    let weight1 = rand::thread_rng().gen();
                    let weight2 = rand::thread_rng().gen();
                    self.graph.edge_mut(i).weight_1t2 = weight1;
                    self.graph.edge_mut(i).weight_2t1 = weight2;

                    let reverse_edge = self.graph.id2edge(self.graph.edge(i).reverse_edge);

                    self.graph.edge_mut(reverse_edge).weight_2t1 = weight1;
                    self.graph.edge_mut(reverse_edge).weight_1t2 = weight2;
                }
            },
            WeightSet::Equal(equal) => {
                let edges = self.graph.edges();
                let mut count = 0;

                for i in edges{
                    if count % 2 == 1{
                        count += 1;
                        continue;
                    }

                    count += 1;

                    let weight1 = equal;
                    let weight2 = equal;
                    self.graph.edge_mut(i).weight_1t2 = weight1;
                    self.graph.edge_mut(i).weight_2t1 = weight2;

                    let reverse_edge = self.graph.id2edge(self.graph.edge(i).reverse_edge);

                    self.graph.edge_mut(reverse_edge).weight_2t1 = weight1;
                    self.graph.edge_mut(reverse_edge).weight_1t2 = weight2;
                }
            },
            WeightSet::OneOverOutdegree => {
                let edges = self.graph.edges();
                let mut count = 0;

                for i in edges{
                    if count % 2 == 1{
                        count += 1;
                        continue;
                    }

                    count += 1;
                    
                    let (n1, n2) = self.get_edge_nodes(Edge(i));
                    let n1_outdegree = self.get_outdegrees(n1) as f64;
                    let n2_outdegree = self.get_outdegrees(n2) as f64;
                    
                    let weight1 = 1.0 / (n1_outdegree);
                    let weight2 = 1.0 / (n2_outdegree);
                    self.graph.edge_mut(i).weight_1t2 = weight1;
                    self.graph.edge_mut(i).weight_2t1 = weight2;

                    let reverse_edge = self.graph.id2edge(self.graph.edge(i).reverse_edge);

                    self.graph.edge_mut(reverse_edge).weight_2t1 = weight1;
                    self.graph.edge_mut(reverse_edge).weight_1t2 = weight2;
                }
            },
        }
    }

    ///set weight n1 to n2
    pub fn set_weight(&mut self, e: Edge, weight: f64){
        self.graph.edge_mut(e.0).weight_1t2 = weight;
        let reverse_edge = self.graph.id2edge(self.graph.edge(e.0).reverse_edge);
        self.graph.edge_mut(reverse_edge).weight_2t1 = weight;
    }

    ///set node's threshold
    pub fn set_threshold(&mut self, n: Node, threshold: f64){
        self.graph.node_mut(n.0).threshold = threshold;
    }

    ///set node's label
    pub fn set_node_label(&mut self, n: Node, label: usize){
        self.graph.node_mut(n.0).label = label;
    }

    //TODO maybe we don't need this one
    // fn setNodeInfluence(){

    // }

    ///maybe we don't need this one
    ///set edge label
    /// if we can visualize our graph, then we will need edges' label. keep this one for the future.
    pub fn set_edge_label(&mut self, e: Edge, label: usize){
        self.graph.edge_mut(e.0).label_1t2 = label;
        let reverse_edge = self.graph.edge(e.0).reverse_edge;
        let reverse_edge = self.get_id2edge(reverse_edge);
        self.graph.edge_mut(reverse_edge.0).label_2t1 = label;
    }

    ///select seeds
    pub fn select_seeds(&mut self, way: SeedSelection, num: usize, label: usize){
        match way{
            SeedSelection::MaxDegree => {
                let mut nodes: Vec<rs_graph::linkedlistgraph::Node> = self.graph.nodes().collect();
                // println!("nodes: {:?}", nodes);
                nodes.sort_by(|a, b| self.get_outdegrees(Node(*a)).cmp(&self.get_outdegrees(Node(*b))));

                // println!("nodes: {:?}", nodes);

                for i in 0..num{
                    // println!("num - i - 1: {}", (self.get_nodes_number() - i - 1));
                    let n = nodes[self.get_nodes_number() - i - 1];
                    self.set_node_label(Node(n), label);
                    let new_pick = self.get_node_id(Node(n));
                    self.seed.push(new_pick);
                }
            },
            SeedSelection::MinDegree => {
                let mut nodes: Vec<rs_graph::linkedlistgraph::Node> = self.graph.nodes().collect();
                // println!("nodes: {:?}", nodes);
                nodes.sort_by(|a, b| self.get_outdegrees(Node(*a)).cmp(&self.get_outdegrees(Node(*b))));

                // println!("nodes: {:?}", nodes);

                for i in 0..num{
                    let n = nodes[i];
                    self.set_node_label(Node(n), label);
                    let new_pick = self.get_node_id(Node(n));
                    self.seed.push(new_pick);
                }
            },
            SeedSelection::Random => {
                let node_size = self.get_nodes_number();
                for _ in 0..num{
                    let new_pick = rand::thread_rng().gen_range(0, node_size);
                    let n = self.get_id2node(new_pick);
                    self.set_node_label(n, label);
                    self.seed.push(new_pick);
                }
            }
        }
    }


    ///before propagation, put seed into next to propagate
    pub fn initialize_propagation(&mut self){

        self.next_to_propagate = self.seed.clone();
    }

    ///doing propagteion k rounds
    pub fn propagte(&mut self, rounds: usize){
        
        match self.propagation_model{
            PropagationModel::IC => {
                for _num_rounds in 0..rounds{
                    if self.next_to_propagate.len() == 0{
                        // println!("converge! rounds:{}", num_rounds);
                        break;
                    }

                    let next_to_propagate_this_run = self.next_to_propagate.clone();
                    self.next_to_propagate = vec![];

                    for j in next_to_propagate_this_run{
                        let node_j = self.get_id2node(j);
                        let mut triggered_node: Vec<rs_graph::linkedlistgraph::Node> = vec![];
                        let mut triggered_edge: Vec<rs_graph::linkedlistgraph::Edge> = vec![];
                        let new_label = self.get_node_label(Node(node_j.0.clone()));

                        //get triggered neighbors
                        for (e, n) in self.graph.outedges(node_j.0){
                            //has been triggered
                            if self.get_node_label(Node(n)) > 0{
                                continue;
                            }

                            let dice: f64 = rand::thread_rng().gen();
                            if dice > self.graph.edge(e).weight_1t2{
                                triggered_node.push(n.clone());
                                triggered_edge.push(e.clone());
                            }
                        }
                        
                        //set new node label and put into next_run
                        for n in triggered_node{
                            self.set_node_label(Node(n), new_label);
                            let id = self.get_node_id(Node(n));
                            self.next_to_propagate.push(id);
                        }

                        //set new edge label
                        for e in triggered_edge{
                            self.set_edge_label(Edge(e), new_label);
                        }

                    }

                }
            },
            PropagationModel::LT => {
                for _num_rounds in 0..rounds{
                    println!("propaget rounds: {} next_to_propagate:{}", _num_rounds, self.next_to_propagate.len());
                    println!("{:?}", self.next_to_propagate);
                    if self.next_to_propagate.len() == 0{
                        println!("converge! rounds:{}", _num_rounds);
                        break;
                    }

                    let next_to_propagate_this_run = self.next_to_propagate.clone();
                    self.next_to_propagate = vec![];
                    
                    //provide new influence
                    for j in next_to_propagate_this_run{
                        let node_j = self.get_id2node(j);
                        let mut triggered_node: Vec<rs_graph::linkedlistgraph::Node> = vec![];
                        // let mut triggered_edge: Vec<rs_graph::linkedlistgraph::Edge> = vec![];
                        let new_label = self.get_node_label(Node(node_j.0.clone()));
                        
                        let mut potential_triggered_node: Vec<rs_graph::linkedlistgraph::Node> = vec![];
                        
                        
                        //nodes affected by new triggered nodes
                        //get_neighbors will give the same node neighbor twice because of edges n1ton2 and n2ton1
                        let mut count = 0;
                        for (_neighbors_e, neighbors_n) in self.graph.neighs(node_j.0) {
                            if count % 2 == 1{
                                continue;
                            }
                            count += 1;

                            potential_triggered_node.push(neighbors_n);
                        }

                        println!("potential triggered nodes: {:?}", potential_triggered_node);

                        for neighbors_n in potential_triggered_node{
                            if self.get_node_label(Node(neighbors_n)) > 0{
                                println!("label: {}", self.get_node_label(Node(neighbors_n)));
                                continue;
                            }
                            println!("before aggregate influence");
                            let mut aggreated_influence: f64 = 0.0;

                            let influence_en: Vec<(rs_graph::linkedlistgraph::Edge, rs_graph::linkedlistgraph::Node)> 
                                                                = self.graph.neighs(neighbors_n).collect();
                            //calculate the aggregated neighbors' influence
                            for (influence_e, influence_n) in influence_en{
                                let (_n_from, n_to) = self.get_edge_nodes(Edge(influence_e));
                                if self.get_node_label(Node(influence_n)) != 0 &&
                                                 self.get_node_id(n_to) == self.get_node_id(Node(neighbors_n)){
                                    aggreated_influence += self.get_edge_weight(Edge(influence_e));
                                }

                                if aggreated_influence >= self.get_node_threshold(Node(neighbors_n)){
                                    triggered_node.push(neighbors_n.clone());
                                    break;
                                }
                            }
                        }

                        //set new node label and put into next_run
                        for n in triggered_node{
                            self.set_node_label(Node(n), new_label);
                            let id = self.get_node_id(Node(n));
                            self.next_to_propagate.push(id);
                        }
                    }
                }
            },
        }
    }
}
