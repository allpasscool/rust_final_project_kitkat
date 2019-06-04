extern crate rand;

use rand::{thread_rng, Rng};
use rs_graph_derive::Graph;
use rs_graph::{traits::*};
use rs_graph::linkedlistgraph::*;
// use rs_graph::classes;
use rs_graph::attributes::{NodeAttributes, EdgeAttributes, AttributedGraph};
use rs_graph::builder::*;
use crate::Builder;
// use graph_propagation;

///MyGraph
///This is an undirected graph
#[derive(Graph)]
pub struct MyGraph_final {
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
    max_degree,
    min_degree,
    random,
}

pub enum ThresholdSet{
    rand,
    baseline(f64),
}

pub enum WeightSet{
    OneOverOutdegree,   // 1 / outdegree
    random,             // random
    equal(f64),         //equal
}

#[derive(Clone, Default)]
pub struct MyNodeData {
    pub label: usize,
    pub threshold: f64,
    pub influence: f64,
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

impl MyGraph_final{
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
    pub fn new(propa_model: PropagationModel) -> MyGraph_final{
        MyGraph_final{
            graph: rs_graph::linkedlistgraph::LinkedListGraph::<u32, MyNodeData, MyEdgeData, ()>::new(),
            nodedata: Vec::new(),
            edgedata: Vec::new(),
            propagation_model: propa_model,
            seed: Vec::new(),
            next_to_propagate: Vec::new(),
        }
    }

    //TODO
    pub fn get_node(){

    }

    //TODO
    pub fn get_edge(){
    
    }

    /// Looks up a `Node` by its ID.
    pub fn get_id2node(&mut self, id: usize) -> rs_graph::linkedlistgraph::Node{
        self.graph.id2node(id)
    }

    /// Looks up an `Edge` by its ID.
    pub fn get_id2edge(&mut self, id: usize) -> rs_graph::linkedlistgraph::Edge{
        self.graph.id2edge(id)
    }

    ///get all nodes
    pub fn get_nodes(&mut self) -> rs_graph::linkedlistgraph::NodeIter<u32>{
        self.graph.nodes()
    }

    ///get all edges
    pub fn get_edges(&mut self) -> rs_graph::linkedlistgraph::EdgeIter<u32>{
        self.graph.edges()
    }

    ///get nodes number
    pub fn get_nodes_number(&mut self) -> usize{
        self.graph.num_nodes()
    }

    ///get edges number
    pub fn get_edges_number(&mut self) -> usize{
        self.graph.num_edges()
    }

    ///add new node with attribute
    pub fn add_node(&mut self, data: MyNodeData) -> rs_graph::linkedlistgraph::Node{
        let new_node = self.graph.add_node();
        self.graph.node_mut(new_node).label = data.label;
        self.graph.node_mut(new_node).threshold = data.threshold;
        self.graph.node_mut(new_node).influence = data.influence;
        new_node
    }

    ///get node id
    pub fn get_node_id(&mut self, n: rs_graph::linkedlistgraph::Node) -> usize{
        self.graph.node_id(n)
    }

    ///get edge id
    pub fn get_edge_id(&mut self, e: rs_graph::linkedlistgraph::Edge) -> usize{
        self.graph.edge_id(e)
    }

    ///given edge, get node from and node to
    pub fn get_edge_nodes(&mut self, e : rs_graph::linkedlistgraph::Edge) -> (rs_graph::linkedlistgraph::Node, rs_graph::linkedlistgraph::Node){
        (self.get_id2node(self.graph.edge(e).from), self.get_id2node(self.graph.edge(e).to))
    }

    ///get neighbors
    /// (edge, node) in neighiter
    pub fn get_neighbors(&mut self, n: rs_graph::linkedlistgraph::Node) -> rs_graph::linkedlistgraph::NeighIter<u32, MyEdgeData, ()>{
        self.graph.neighs(n)
    }

    ///get outdegrees
    pub fn get_outdegrees(&mut self, n: rs_graph::linkedlistgraph::Node) -> usize{
        let mut count = 0;
        for (edge, node) in self.graph.neighs(n){
            println!("neighbor node id: {:?} neighbor edge id: {:?}", node, edge);
            count += 1;
        }
        count / 2
    }

    ///get node threshold
    pub fn get_node_threshold(&mut self, n: rs_graph::linkedlistgraph::Node) -> f64{
        self.graph.node(n).threshold
    }

    /*
    ///get outedges
    pub fn get_outedges_and_nodes(&mut self, n: rs_graph::linkedlistgraph::Node) -> rs_graph::linkedlistgraph::NeighIter{
        self.graph.neighs(n)
    }
    */
    

    ///get node label
    pub fn get_node_label(& self, n: rs_graph::linkedlistgraph::Node) -> usize{
        self.graph.node(n).label
    }

    ///get edge label 1t2
    pub fn get_edge_label(&self, e: rs_graph::linkedlistgraph::Edge) -> usize{
        self.graph.edge(e).label_1t2
    }

    ///get edge weight 1t2
    pub fn get_edge_weight(&mut self, e: rs_graph::linkedlistgraph::Edge) -> f64{
        self.graph.edge(e).weight_1t2
    }

    ///add edge
    pub fn add_edge(&mut self, n1: rs_graph::linkedlistgraph::Node, n2: rs_graph::linkedlistgraph::Node, data: MyEdgeData) -> rs_graph::linkedlistgraph::Edge{
        let new_edge = self.graph.add_edge(n2, n1);
        self.graph.edge_mut(new_edge).from = self.graph.node_id(n2);
        self.graph.edge_mut(new_edge).to = self.graph.node_id(n1);
        self.graph.edge_mut(new_edge).label_1t2 = data.label_2t1;
        self.graph.edge_mut(new_edge).weight_1t2 = data.weight_2t1;
        self.graph.edge_mut(new_edge).label_2t1 = data.label_1t2;
        self.graph.edge_mut(new_edge).weight_2t1 = data.weight_1t2;
        
        let new_edge1 = self.graph.add_edge(n1, n2);
        self.graph.edge_mut(new_edge1).from = self.graph.node_id(n1);
        self.graph.edge_mut(new_edge1).to = self.graph.node_id(n2);
        self.graph.edge_mut(new_edge1).label_1t2 = data.label_1t2;
        self.graph.edge_mut(new_edge1).weight_1t2 = data.weight_1t2;
        self.graph.edge_mut(new_edge1).label_2t1 = data.label_2t1;
        self.graph.edge_mut(new_edge1).weight_2t1 = data.weight_2t1;

        self.graph.edge_mut(new_edge).reverse_edge = self.graph.edge_id(new_edge1);
        self.graph.edge_mut(new_edge1).reverse_edge = self.graph.edge_id(new_edge);

        new_edge1
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
                ThresholdSet::rand => self.graph.node_mut(i).threshold = rand::thread_rng().gen(),
                ThresholdSet::baseline(baseline1) => self.graph.node_mut(i).threshold = baseline1,
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
            WeightSet::random => {
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
            WeightSet::equal(equal) => {
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
                    
                    let (n1, n2) = self.get_edge_nodes(i);
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
    pub fn set_weight(&mut self, e: rs_graph::linkedlistgraph::Edge, weight: f64){
        self.graph.edge_mut(e).weight_1t2 = weight;
        let reverse_edge = self.graph.id2edge(self.graph.edge(e).reverse_edge);
        self.graph.edge_mut(reverse_edge).weight_2t1 = weight;
    }

    ///set node's threshold
    pub fn set_threshold(&mut self, n: rs_graph::linkedlistgraph::Node, threshold: f64){
        self.graph.node_mut(n).threshold = threshold;
    }

    ///set node's label
    pub fn set_node_label(&mut self, n: rs_graph::linkedlistgraph::Node, label: usize){
        self.graph.node_mut(n).label = label;
    }

    //TODO maybe we don't need this one
    // fn setNodeInfluence(){

    // }

    ///maybe we don't need this one
    ///set edge label
    pub fn set_edge_label(&mut self, e: rs_graph::linkedlistgraph::Edge, label: usize){
        self.graph.edge_mut(e).label_1t2 = label;
        let reverse_edge = self.graph.edge(e).reverse_edge;
        let reverse_edge = self.get_id2edge(reverse_edge);
        self.graph.edge_mut(reverse_edge).label_2t1 = label;
    }

    ///select seeds
    //need to finish random
    pub fn select_seeds(&mut self, way: SeedSelection, num: usize, label: usize){
        match way{
            SeedSelection::max_degree => {
                let mut nodes: Vec<rs_graph::linkedlistgraph::Node> = self.graph.nodes().collect();
                println!("nodes: {:?}", nodes);
                nodes.sort_by(|a, b| self.get_outdegrees(*a).cmp(&self.get_outdegrees(*b)));

                println!("nodes: {:?}", nodes);

                for i in 0..num{
                    println!("num - i - 1: {}", (self.get_nodes_number() - i - 1));
                    let n = nodes[self.get_nodes_number() - i - 1];
                    self.set_node_label(n, label);
                    let new_pick = self.get_node_id(n);
                    self.seed.push(new_pick);
                }
            },
            SeedSelection::min_degree => {
                let mut nodes: Vec<rs_graph::linkedlistgraph::Node> = self.graph.nodes().collect();
                println!("nodes: {:?}", nodes);
                nodes.sort_by(|a, b| self.get_outdegrees(*a).cmp(&self.get_outdegrees(*b)));

                println!("nodes: {:?}", nodes);

                for i in 0..num{
                    let n = nodes[i];
                    self.set_node_label(n, label);
                    let new_pick = self.get_node_id(n);
                    self.seed.push(new_pick);
                }
            },
            SeedSelection::random => {
                let node_size = self.get_nodes_number();
                for _ in 0..num{
                    let new_pick = rand::thread_rng().gen_range(0, node_size);
                    let n = self.get_id2node(new_pick);
                    self.set_node_label(n, label);
                    self.seed.push(new_pick);
                    //need to consider pick duplicate one
                    //if label != 0, reselect another one
                    //TODO
                }
            }
        }
    }


    ///before propagation, put seed into next to propagate
    pub fn initialize_propagation(&mut self){

        self.next_to_propagate = self.seed.clone();
    }

    ///doing propagteion k runs
    //TODO
    pub fn propagte(&mut self, runs: usize){
        
        match self.propagation_model{
            PropagationModel::IC => {
                for num_runs in 0..runs{
                    if self.next_to_propagate.len() == 0{
                        println!("converge! runs:{}", num_runs);
                        break;
                    }

                    let next_to_propagate_this_run = self.next_to_propagate.clone();
                    self.next_to_propagate = vec![];

                    for j in next_to_propagate_this_run{
                        let node_j = self.get_id2node(j);
                        let mut triggeredNode: Vec<rs_graph::linkedlistgraph::Node> = vec![];
                        let mut triggeredEdge: Vec<rs_graph::linkedlistgraph::Edge> = vec![];
                        let new_label = self.get_node_label(node_j);

                        //get triggered neighbors
                        for (e, n) in self.graph.outedges(node_j){
                            //has been triggered
                            if self.get_node_label(n) != 0{
                                continue;
                            }

                            let dice: f64 = rand::thread_rng().gen();
                            if dice > self.graph.edge(e).weight_1t2{
                                triggeredNode.push(n.clone());
                                triggeredEdge.push(e.clone());
                            }
                        }
                        
                        //set new node label and put into next_run
                        for n in triggeredNode{
                            self.set_node_label(n, new_label);
                            let id = self.get_node_id(n);
                            self.next_to_propagate.push(id);
                        }

                        //set new edge label
                        for e in triggeredEdge{
                            self.set_edge_label(e, new_label);
                        }

                    }

                }
            },
            PropagationModel::LT => {
                for num_runs in 0..runs{
                    if self.next_to_propagate.len() == 0{
                        println!("converge! runs:{}", num_runs);
                        break;
                    }

                    let next_to_propagate_this_run = self.next_to_propagate.clone();
                    self.next_to_propagate = vec![];
                    
                    //provide new influence
                    for j in next_to_propagate_this_run{
                        let node_j = self.get_id2node(j);
                        let mut triggeredNode: Vec<rs_graph::linkedlistgraph::Node> = vec![];
                        let mut triggeredEdge: Vec<rs_graph::linkedlistgraph::Edge> = vec![];
                        let new_label = self.get_node_label(node_j);
                        
                        let mut potentialTriggeredNode: Vec<rs_graph::linkedlistgraph::Node> = vec![];
                        
                        
                        //nodes affected by new triggered nodes
                        for (neighborsE, neighborsN) in self.get_neighbors(node_j) {
                            potentialTriggeredNode.push(neighborsN);
                        }

                        for neighborsN in potentialTriggeredNode{
                            let mut aggreated_influence: f64 = 0.0;

                            let mut influence_EN: Vec<(rs_graph::linkedlistgraph::Edge, rs_graph::linkedlistgraph::Node)> = self.get_neighbors(neighborsN).collect();
                            //calculate the aggregated neighbors' influence
                            for (influenceE, influenceN) in influence_EN{
                                let (n_from, n_to) = self.get_edge_nodes(influenceE);
                                if self.get_node_label(influenceN) != 0 && self.get_node_id(n_to) == self.get_node_id(neighborsN){
                                    aggreated_influence += self.get_edge_weight(influenceE);
                                }

                                if aggreated_influence >= self.get_node_threshold(neighborsN){
                                    triggeredNode.push(neighborsN.clone());
                                    break;
                                }
                            }
                        }

                        //set new node label and put into next_run
                        for n in triggeredNode{
                            self.set_node_label(n, new_label);
                            let id = self.get_node_id(n);
                            self.next_to_propagate.push(id);
                        }
                    }
                }
            },
        }
    }
}
