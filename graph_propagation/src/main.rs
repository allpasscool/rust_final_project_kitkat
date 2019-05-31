extern crate rand;

use rand::Rng;
use rs_graph_derive::Graph;
use rs_graph::{traits::*};
use rs_graph::linkedlistgraph::*;
// use rs_graph::classes;
use rs_graph::attributes::{NodeAttributes, EdgeAttributes, AttributedGraph};
use rs_graph::builder::*;
use crate::Builder;

// #[derive(Clone, Default)]
// struct NodeData {
//     balance: f64,
// }

// #[derive(Clone, Default)]
// struct EdgeData {
//     bound: f64,
// }

// #[derive(Graph)]
// struct MyGraph {
//     #[graph] graph: LinkedListGraph,
//     #[nodeattrs(NodeData)] nodedata: Vec<NodeData>,
//     #[edgeattrs(EdgeData)] edgedata: Vec<EdgeData>,
// }

// #[derive(Graph)]
// struct MyGraph2 {
//     #[graph] graph: LinkedListGraph,
//     #[nodeattrs(NodeData)] nodedata: Vec<NodeData>,
//     #[edgeattrs(EdgeData)] edgedata: Vec<EdgeData>,
// }

// impl From<LinkedListGraph> for MyGraph {
//     fn from(g: LinkedListGraph) -> MyGraph {
//         let n = g.num_nodes();
//         let m = g.num_edges();
//         MyGraph {
//             graph: g,
//             nodedata: vec![Default::default(); n],
//             edgedata: vec![Default::default(); m],
//         }
//     }
// }

fn main() {
    
    build_MyGraph_test();
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]
fn build_MyGraph_test(){
    println!("enter build my graph test");

    let mut myG = MyGraph_final::new(None, None, PropagationModel::LT);
    let (n1, n2, n3) = (
                        myG.add_node(MyNodeData{label: 0, threshold: 0.5, influence: 0.5}),
                        myG.add_node(MyNodeData{label: 0, threshold: 0.5, influence: 0.5}),
                        myG.add_node(MyNodeData{label: 0, threshold: 0.5, influence: 0.5})
    );
    let (e12, e13) = (
                        myG.add_edge(n1, n2, MyEdgeData{from: myG.graph.node_id(n1), to: myG.graph.node_id(n2), label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0}),
                        myG.add_edge(n1, n3, MyEdgeData{from: myG.graph.node_id(n1), to: myG.graph.node_id(n3), label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0})
    );

    myG.InitializeNodeLabel();
    myG.InitializeNodeThreshold(ThresholdSet::baseline(0.1));
    myG.InitializeEdgeLabel();
    myG.InitializeWeight();

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

}

//MyGraph
#[derive(Graph)]
struct MyGraph_final {
    #[graph] graph: LinkedListGraph::<u32, MyNodeData, MyEdgeData, ()>,
    #[nodeattrs(MyNodeData)] nodedata: Vec<MyNodeData>,
    #[edgeattrs(MyEdgeData)] edgedata: Vec<MyEdgeData>,
    propagation_model: PropagationModel,
    seed: Vec<usize>,
    next_to_propagate: Vec<usize>,
}

enum PropagationModel{
    LT, //linear threshold
    IC, //independent cascade
}

enum SeedSelection{
    max_degree,
    min_degree,
    random,
}

enum ThresholdSet{
    rand,
    baseline(f64),
}

#[derive(Clone, Default)]
struct MyNodeData {
    label: usize,
    threshold: f64,
    influence: f64,
}

#[derive(Clone, Default)]
struct MyEdgeData {
    from: usize,
    to: usize,
    label_1t2: usize,
    weight_1t2: f64,
    label_2t1: usize,
    weight_2t1: f64,
    reverse_edge: usize,
}

// supported data
const supported_file: &'static [&'static str] = &["ego-Facebook"];

impl MyGraph_final{
    fn new(file_name: Option<String>, file_loc: Option<String>, propa_model: PropagationModel) -> MyGraph_final{
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
    fn get_node(&mut self, id: usize) -> rs_graph::linkedlistgraph::Node{
        self.graph.id2node(id)
    }

    //TODO
    fn get_edge(){

    }

    //TODO
    fn get_nodes(){

    }

    //TODO
    fn get_edges(){

    }

    //TODO
    fn add_node(&mut self, data: MyNodeData) -> rs_graph::linkedlistgraph::Node{
        let new_node = self.graph.add_node();
        self.graph.node_mut(new_node).label = data.label;
        self.graph.node_mut(new_node).threshold = data.threshold;
        self.graph.node_mut(new_node).influence = data.influence;
        new_node
    }

    //TODO
    fn add_edge(&mut self, n1: rs_graph::linkedlistgraph::Node, n2: rs_graph::linkedlistgraph::Node, data: MyEdgeData) -> rs_graph::linkedlistgraph::Edge{
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

    //TODO
    fn InitializeNodeLabel(&mut self){
        let nodes = self.graph.nodes();

        for i in nodes{
            self.graph.node_mut(i).label = 0;
        }
    }

    //TODO
    fn InitializeNodeThreshold(&mut self, set: ThresholdSet){
        let nodes = self.graph.nodes();
        for i in nodes{
            match set{
                ThresholdSet::rand => self.graph.node_mut(i).threshold = rand::thread_rng().gen(),
                ThresholdSet::baseline(baseline1) => self.graph.node_mut(i).threshold = baseline1,
            }
        }
    }

    //TODO
    fn InitializeEdgeLabel(&mut self){
        let edges = self.graph.edges();

        for i in edges{
            self.graph.edge_mut(i).label_1t2 = 0;
            self.graph.edge_mut(i).label_2t1 = 0;
        }

    }

    //TODO
    fn InitializeWeight(&mut self){
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
    }

    //TODO
    fn selectSeed(&mut self, way: String, num: u32){

    }

    //TODO
    fn propagte(&mut self, runs: u32){

    }
}