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

//MyGraph
//This is an undirected graph
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
    OneOverOutdegree, // 1 / outdegree
    random,         // random
    equal(f64),          //equal
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
pub const supported_file: &'static [&'static str] = &["ego-Facebook"];

impl MyGraph_final{
    pub fn new(file_name: Option<String>, file_loc: Option<String>, propa_model: PropagationModel) -> MyGraph_final{
        let read_file = false;
        
        //if supported file
        if file_name.is_some(){
            //get it from supported_file
            //set read_file = true
            //TODO

        }

        //if read_file is correct
        //read file
        //TODO
        if read_file{
            match file_name{
                Some(a) => {
                    let ego_Facebook = String::from("ego-Facebook");
                    match a{
                        ego_Facebook => {
                            //read file and create graph
                            //TODO
                            //return MyGraph_final here
                            }, 
                    }
                    
                }
                _ => (),
            }
        } 

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

    //node id to node
    pub fn get_id2node(&mut self, id: usize) -> rs_graph::linkedlistgraph::Node{
        self.graph.id2node(id)
    }

    //edge id to edge
    pub fn get_id2edge(&mut self, id: usize) -> rs_graph::linkedlistgraph::Edge{
        self.graph.id2edge(id)
    }

    //get all nodes
    pub fn get_nodes(&mut self) -> rs_graph::linkedlistgraph::NodeIter<u32>{
        self.graph.nodes()
    }

    //get all edges
    pub fn get_edges(&mut self) -> rs_graph::linkedlistgraph::EdgeIter<u32>{
        self.graph.edges()
    }

    //get nodes number
    pub fn get_nodes_number(&mut self) -> usize{
        self.graph.num_nodes()
    }

    //get edges number
    pub fn get_edges_number(&mut self) -> usize{
        self.graph.num_edges()
    }

    //add new node with attribute
    pub fn add_node(&mut self, data: MyNodeData) -> rs_graph::linkedlistgraph::Node{
        let new_node = self.graph.add_node();
        self.graph.node_mut(new_node).label = data.label;
        self.graph.node_mut(new_node).threshold = data.threshold;
        self.graph.node_mut(new_node).influence = data.influence;
        new_node
    }

    //get node id
    pub fn get_node_id(&mut self, n: rs_graph::linkedlistgraph::Node) -> usize{
        self.graph.node_id(n)
    }

    //get edge id
    pub fn get_edge_id(&mut self, e: rs_graph::linkedlistgraph::Edge) -> usize{
        self.graph.edge_id(e)
    }

    ///given edge, get node from and node to
    pub fn get_edge_nodes(&mut self, e : rs_graph::linkedlistgraph::Edge) -> (rs_graph::linkedlistgraph::Edge, rs_graph::linkedlistgraph::Edge){
        (self.get_id2edge(self.graph.edge(e).from), self.get_id2edge(self.graph.edge(e).to))
    }

    //get neighbors
    pub fn get_neighbors(&mut self, n: rs_graph::linkedlistgraph::Node) -> rs_graph::linkedlistgraph::NeighIter<u32, MyEdgeData, ()>{
        self.graph.neighs(n)
    }

    //get outedges

    //get node label
    pub fn get_node_label(& self, n: rs_graph::linkedlistgraph::Node) -> usize{
        self.graph.node(n).label
    }

    //get edge label 1t2
    pub fn get_edge_label(&self, e: rs_graph::linkedlistgraph::Edge) -> usize{
        self.graph.edge(e).label_1t2
    }

    //add edge
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

    //initialize node label to 0
    pub fn Initialize_Node_Label(&mut self){
        let nodes = self.graph.nodes();

        for i in nodes{
            self.graph.node_mut(i).label = 0;
        }
    }

    //initialize nodes' threshold to random between 0 and 1, or to a baseline
    pub fn Initialize_Node_Threshold(&mut self, set: ThresholdSet){
        let nodes = self.graph.nodes();
        for i in nodes{
            match set{
                ThresholdSet::rand => self.graph.node_mut(i).threshold = rand::thread_rng().gen(),
                ThresholdSet::baseline(baseline1) => self.graph.node_mut(i).threshold = baseline1,
            }
        }
    }

    //initialize edge label to 0
    pub fn Initialize_Edge_Label(&mut self){
        let edges = self.graph.edges();

        for i in edges{
            self.graph.edge_mut(i).label_1t2 = 0;
            self.graph.edge_mut(i).label_2t1 = 0;
        }

    }

    //initialized edge weight which means n1 to n2, is directed
    pub fn Initialize_Weight(&mut self, way: WeightSet){
        match way{
            WeightSet::random => {
                let edges = self.graph.edges();

                for i in edges{
                    let weight1 = rand::thread_rng().gen();
                    let weight2 = rand::thread_rng().gen();
                    self.graph.edge_mut(i).weight_1t2 = weight1;
                    self.graph.edge_mut(i).weight_2t1 = weight2;

                    let reverse_edge = self.graph.id2edge(self.graph.edge(i).reverse_edge);

                    self.graph.edge_mut(reverse_edge).weight_2t1 = weight1;
                    self.graph.edge_mut(reverse_edge).weight_1t2 = weight2;
                }
            },
            WeightSet::equal(equal) => {()},
            WeightSet::OneOverOutdegree => {()},
        }
    }

    //set weight n1 to n2
    pub fn setWeight(&mut self, e: rs_graph::linkedlistgraph::Edge, weight: f64){
        self.graph.edge_mut(e).weight_1t2 = weight;
        let reverse_edge = self.graph.id2edge(self.graph.edge(e).reverse_edge);
        self.graph.edge_mut(reverse_edge).weight_2t1 = weight;
    }

    //set node's threshold
    pub fn setThreshold(&mut self, n: rs_graph::linkedlistgraph::Node, threshold: f64){
        self.graph.node_mut(n).threshold = threshold;
    }

    //set node's label
    pub fn setNodeLabel(&mut self, n: rs_graph::linkedlistgraph::Node, label: usize){
        self.graph.node_mut(n).label = label;
    }

    //TODO maybe we don't need this one
    // fn setNodeInfluence(){

    // }

    //maybe we don't need this one
    //set edge labe
    pub fn setEdgeLabel(&mut self, e: rs_graph::linkedlistgraph::Edge, label: usize){
        self.graph.edge_mut(e).label_1t2 = label;
        let reverse_edge = self.graph.edge(e).reverse_edge;
        let reverse_edge = self.get_id2edge(reverse_edge);
        self.graph.edge_mut(reverse_edge).label_2t1 = label;
    }

    //TODO
    pub fn selectSeed(&mut self, way: SeedSelection, num: usize, label: usize){
        match way{
            SeedSelection::max_degree => {

            },
            SeedSelection::min_degree => {

            },
            SeedSelection::random => {
                let node_size = self.get_nodes_number();
                for _ in 0..num{
                    let new_pick = rand::thread_rng().gen_range(0, node_size);
                    let n = self.get_id2node(new_pick);
                    self.setNodeLabel(n, label);
                    self.seed.push(new_pick);
                    //need to consider pick duplicate one
                    //if label != 0, reselect another one
                    //TODO
                }
            }
        }
    }


    //before propagation, put seed into next to propagate
    pub fn initialize_Propagate(&mut self){

        self.next_to_propagate = self.seed.clone();
    }

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
                            self.setNodeLabel(n, new_label);
                            let id = self.get_node_id(n);
                            self.next_to_propagate.push(id);
                        }

                        //set new edge label
                        for e in triggeredEdge{
                            self.setEdgeLabel(e, new_label);
                        }

                    }

                }
            },
            PropagationModel::LT => {

            },
        }
    }
}

fn main() {
    
    build_My_Graph_test();
}

// #[cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]
fn build_My_Graph_test(){
    println!("enter build my graph test");

    //build a graph with three nodes and 2 edges
    let mut myG = MyGraph_final::new(None, None, PropagationModel::IC);
    let (n1, n2, n3) = (
                        myG.add_node(MyNodeData{label: 0, threshold: 0.5, influence: 0.5}),
                        myG.add_node(MyNodeData{label: 0, threshold: 0.5, influence: 0.5}),
                        myG.add_node(MyNodeData{label: 0, threshold: 0.5, influence: 0.5})
    );
    let (e12, e13) = (
                        myG.add_edge(n1, n2, MyEdgeData{from: myG.graph.node_id(n1), to: myG.graph.node_id(n2), label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0}),
                        myG.add_edge(n1, n3, MyEdgeData{from: myG.graph.node_id(n1), to: myG.graph.node_id(n3), label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0})
    );

    //initialize graph data setting
    myG.Initialize_Node_Label();
    myG.Initialize_Node_Threshold(ThresholdSet::baseline(0.1));
    myG.Initialize_Edge_Label();
    myG.Initialize_Weight(WeightSet::random);

    for i in myG.graph.nodes(){
        println!("id:{} label:{} Threshold:{}", myG.graph.node_id(i), myG.graph.node(i).label, myG.graph.node(i).threshold);
    }

    for i in myG.graph.edges(){
        println!("id:{} from:{} to:{} label_1t2:{} label_2t1:{} weight_1t2:{} weight_2t1:{} reverse_edge_id:{}",
                myG.graph.edge_id(i),
                myG.graph.edge(i).from,
                myG.graph.edge(i).to,
                myG.graph.edge(i).label_1t2,
                myG.graph.edge(i).label_2t1,
                myG.graph.edge(i).weight_1t2,
                myG.graph.edge(i).weight_2t1,
                myG.graph.edge(i).reverse_edge);
    }

    //select seeds
    myG.selectSeed(SeedSelection::random, 1, 1);

    println!("seed");
    for i in &myG.seed{
        println!("seed node id: {}", i);
    }

    //initialize propagation, needs to be done after select seeds
    myG.initialize_Propagate();

    //run propagataion
    myG.propagte(10);

    let nodes = myG.get_nodes();

    println!("node label == 1");
    let mut counter = 0;
    for i in nodes{
        if myG.get_node_label(i) == 1{
            println!("id:{} label:{} Threshold:{}", myG.graph.node_id(i), myG.graph.node(i).label, myG.graph.node(i).threshold);
            counter += 1;
        }
    }

    println!("result: {} nodes with label 1", counter);

}
