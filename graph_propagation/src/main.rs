extern crate rand;
extern crate graph_propagation;

// use rand::{thread_rng, Rng};
// use rs_graph_derive::Graph;
// use rs_graph::{traits::*};
// use rs_graph::linkedlistgraph::*;
// use rs_graph::classes;
// use rs_graph::attributes::{NodeAttributes, EdgeAttributes, AttributedGraph};
// use rs_graph::builder::*;
// use crate::Builder;
use graph_propagation::*;

/*
//MyGraph
//This is an undirected graph
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

enum WeightSet{
    OneOverOutdegree, // 1 / outdegree
    random,         // random
    equal(f64),          //equal
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

// supported data file
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
    fn get_node(){

    }

    //TODO
    fn get_edge(){
    
    }

    //node id to node
    fn get_id2node(&mut self, id: usize) -> rs_graph::linkedlistgraph::Node{
        self.graph.id2node(id)
    }

    //edge id to edge
    fn get_id2edge(&mut self, id: usize) -> rs_graph::linkedlistgraph::Edge{
        self.graph.id2edge(id)
    }

    //get all nodes
    fn get_nodes(&mut self) -> rs_graph::linkedlistgraph::NodeIter<u32>{
        self.graph.nodes()
    }

    //get all edges
    fn get_edges(&mut self) -> rs_graph::linkedlistgraph::EdgeIter<u32>{
        self.graph.edges()
    }

    //get nodes number
    fn get_nodes_number(&mut self) -> usize{
        self.graph.num_nodes()
    }

    //get edges number
    fn get_edges_number(&mut self) -> usize{
        self.graph.num_edges()
    }

    //add new node with attribute
    fn add_node(&mut self, data: MyNodeData) -> rs_graph::linkedlistgraph::Node{
        let new_node = self.graph.add_node();
        self.graph.node_mut(new_node).label = data.label;
        self.graph.node_mut(new_node).threshold = data.threshold;
        self.graph.node_mut(new_node).influence = data.influence;
        new_node
    }

    //get node id
    fn get_node_id(&mut self, n: rs_graph::linkedlistgraph::Node) -> usize{
        self.graph.node_id(n)
    }

    //get edge id
    fn get_edge_id(&mut self, e: rs_graph::linkedlistgraph::Edge) -> usize{
        self.graph.edge_id(e)
    }

    ///given edge, get node from and node to
    fn get_edge_nodes(&mut self, e : rs_graph::linkedlistgraph::Edge) -> (rs_graph::linkedlistgraph::Edge, rs_graph::linkedlistgraph::Edge){
        (self.get_id2edge(self.graph.edge(e).from), self.get_id2edge(self.graph.edge(e).to))
    }

    //get neighbors
    fn get_neighbors(&mut self, n: rs_graph::linkedlistgraph::Node) -> rs_graph::linkedlistgraph::NeighIter<u32, MyEdgeData, ()>{
        self.graph.neighs(n)
    }

    //get outedges

    //get node label
    fn get_node_label(& self, n: rs_graph::linkedlistgraph::Node) -> usize{
        self.graph.node(n).label
    }

    //get edge label 1t2
    fn get_edge_label(&self, e: rs_graph::linkedlistgraph::Edge) -> usize{
        self.graph.edge(e).label_1t2
    }

    //add edge
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

    //initialize node label to 0
    fn Initialize_Node_Label(&mut self){
        let nodes = self.graph.nodes();

        for i in nodes{
            self.graph.node_mut(i).label = 0;
        }
    }

    //initialize nodes' threshold to random between 0 and 1, or to a baseline
    fn Initialize_Node_Threshold(&mut self, set: ThresholdSet){
        let nodes = self.graph.nodes();
        for i in nodes{
            match set{
                ThresholdSet::rand => self.graph.node_mut(i).threshold = rand::thread_rng().gen(),
                ThresholdSet::baseline(baseline1) => self.graph.node_mut(i).threshold = baseline1,
            }
        }
    }

    //initialize edge label to 0
    fn Initialize_Edge_Label(&mut self){
        let edges = self.graph.edges();

        for i in edges{
            self.graph.edge_mut(i).label_1t2 = 0;
            self.graph.edge_mut(i).label_2t1 = 0;
        }

    }

    //initialized edge weight which means n1 to n2, is directed
    fn Initialize_Weight(&mut self, way: WeightSet){
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
    fn setWeight(&mut self, e: rs_graph::linkedlistgraph::Edge, weight: f64){
        self.graph.edge_mut(e).weight_1t2 = weight;
        let reverse_edge = self.graph.id2edge(self.graph.edge(e).reverse_edge);
        self.graph.edge_mut(reverse_edge).weight_2t1 = weight;
    }

    //set node's threshold
    fn setThreshold(&mut self, n: rs_graph::linkedlistgraph::Node, threshold: f64){
        self.graph.node_mut(n).threshold = threshold;
    }

    //set node's label
    fn setNodeLabel(&mut self, n: rs_graph::linkedlistgraph::Node, label: usize){
        self.graph.node_mut(n).label = label;
    }

    //TODO maybe we don't need this one
    // fn setNodeInfluence(){

    // }

    //maybe we don't need this one
    //set edge labe
    fn setEdgeLabel(&mut self, e: rs_graph::linkedlistgraph::Edge, label: usize){
        self.graph.edge_mut(e).label_1t2 = label;
        let reverse_edge = self.graph.edge(e).reverse_edge;
        let reverse_edge = self.get_id2edge(reverse_edge);
        self.graph.edge_mut(reverse_edge).label_2t1 = label;
    }

    //TODO
    fn selectSeed(&mut self, way: SeedSelection, num: usize, label: usize){
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
    fn initialize_Propagate(&mut self){

        self.next_to_propagate = self.seed.clone();
    }

    //TODO
    fn propagte(&mut self, runs: usize){
        
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
*/

fn main() {
    
    // build_my_graph_test_ic();
    build_my_graph_test_lt();
}

// #[cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]
fn build_my_graph_test_ic(){
    println!("enter build my graph test");

    //build a graph with three nodes and 2 edges
    let mut my_g = Graph::new(PropagationModel::IC);
    let (n1, n2, n3) = (
                        my_g.add_node(MyNodeData{label: 0, threshold: 0.5}),
                        my_g.add_node(MyNodeData{label: 0, threshold: 0.5}),
                        my_g.add_node(MyNodeData{label: 0, threshold: 0.5})
    );
    let n1_id = my_g.get_node_id(n1.clone());
    let n2_id = my_g.get_node_id(n2.clone());
    let n3_id = my_g.get_node_id(n3.clone());
    let (_e12, _e13) = (
                        my_g.add_edge(n1.clone(), n2.clone(), MyEdgeData{from: n1_id, to: n2_id, label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0}),
                        my_g.add_edge(n1.clone(), n3.clone(), MyEdgeData{from: n1_id, to: n3_id, label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0})
    );
    // let (e12) = (
    //                     myG.add_edge(n1, n2, MyEdgeData{from: myG.graph.node_id(n1), to: myG.graph.node_id(n2), label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0}),
    //                     );

    //initialize graph data setting
    my_g.initialize_node_label();
    my_g.initialize_node_threshold(ThresholdSet::Baseline(0.1));
    my_g.initialize_edge_label();
    my_g.initialize_weight(WeightSet::Random);

    for i in my_g.get_nodes(){
        // println!("id:{} label:{} Threshold:{}", my_g.graph.node_id(i), my_g.graph.node(i).label, my_g.graph.node(i).threshold);
        println!("id:{} label:{} Threshold:{}", my_g.get_node_id(i.clone()), my_g.get_node_label(i.clone()), my_g.get_node_threshold(i.clone()));
    }

    for i in my_g.get_edges(){
        let (f, t) = my_g.get_edge_nodes(i.clone());
        // println!("id:{} from:{} to:{} label_1t2:{} label_2t1:{} weight_1t2:{} weight_2t1:{} reverse_edge_id:{}",
        //         my_g.graph.edge_id(i),
        //         my_g.graph.edge(i).from,
        //         my_g.graph.edge(i).to,
        //         my_g.graph.edge(i).label_1t2,
        //         my_g.graph.edge(i).label_2t1,
        //         my_g.graph.edge(i).weight_1t2,
        //         my_g.graph.edge(i).weight_2t1,
        //         my_g.graph.edge(i).reverse_edge);
        println!("id:{} from:{} to:{} label_1t2:{} weight_1t2:{}",
                my_g.get_edge_id(i.clone()),
                my_g.get_node_id(f),
                my_g.get_node_id(t),
                my_g.get_edge_label(i.clone()),
                my_g.get_edge_weight(i.clone()));
    }

    //select seeds
    // myG.select_seeds(SeedSelection::random, 1, 1);
    my_g.select_seeds(SeedSelection::MaxDegree, 1, 1);

    println!("n1 out degree: {}", my_g.get_outdegrees(n1.clone()));
    println!("n1 : {:?}", n1.0.clone());
    println!("seed");
    for i in &my_g.seed{
        println!("seed node id: {}", i);
    }

    //initialize propagation, needs to be done after select seeds
    my_g.initialize_propagation();

    //run propagataion
    my_g.propagte(10);

    let nodes = my_g.get_nodes();

    println!("node label == 1");
    let mut counter = 0;
    for i in nodes{
        if my_g.get_node_label(i.clone()) == 1{
            println!("id:{} label:{} Threshold:{}", my_g.get_node_id(i.clone()), my_g.get_node_label(i.clone()), my_g.get_node_threshold(i.clone()));
            counter += 1;
        }
    }

    println!("result: {} nodes with label 1", counter);

    println!("get out degree: n1 {}", my_g.get_outdegrees(n1));

}

fn build_my_graph_test_lt(){
    println!("enter build my graph test");

    //build a graph with three nodes and 2 edges
    let mut my_g = Graph::new(PropagationModel::LT);
    let (n1, n2, n3) = (
                        my_g.add_node(MyNodeData{label: 0, threshold: 0.5}),
                        my_g.add_node(MyNodeData{label: 0, threshold: 0.5}),
                        my_g.add_node(MyNodeData{label: 0, threshold: 0.5})
    );

    let n1_id = my_g.get_node_id(n1.clone());
    let n2_id = my_g.get_node_id(n2.clone());
    let n3_id = my_g.get_node_id(n3.clone());

    let (_e12, _e13) = (
                        my_g.add_edge(n1.clone(), n2.clone(), MyEdgeData{from: n1_id, to: n2_id, label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0}),
                        my_g.add_edge(n1.clone(), n3.clone(), MyEdgeData{from: n1_id, to: n3_id, label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0})
    );
    // let (e12) = (
    //                     myG.add_edge(n1, n2, MyEdgeData{from: myG.graph.node_id(n1), to: myG.graph.node_id(n2), label_1t2: 0, weight_1t2: 1.0, label_2t1: 0, weight_2t1: 0.6, reverse_edge: 0}),
    //                     );

    //initialize graph data setting
    my_g.initialize_node_label();
    my_g.initialize_node_threshold(ThresholdSet::Baseline(0.1));
    my_g.initialize_edge_label();
    my_g.initialize_weight(WeightSet::Random);

    for i in my_g.get_nodes(){
        // println!("id:{} label:{} Threshold:{}", my_g.graph.node_id(i), my_g.graph.node(i).label, my_g.graph.node(i).threshold);
        println!("id:{} label:{} Threshold:{}", my_g.get_node_id(i.clone()), my_g.get_node_label(i.clone()), my_g.get_node_threshold(i.clone()));
    }

    for i in my_g.get_edges(){
        // println!("id:{} from:{} to:{} label_1t2:{} label_2t1:{} weight_1t2:{} weight_2t1:{} reverse_edge_id:{}",
        //         my_g.graph.edge_id(i),
        //         my_g.graph.edge(i).from,
        //         my_g.graph.edge(i).to,
        //         my_g.graph.edge(i).label_1t2,
        //         my_g.graph.edge(i).label_2t1,
        //         my_g.graph.edge(i).weight_1t2,
        //         my_g.graph.edge(i).weight_2t1,
        //         my_g.graph.edge(i).reverse_edge);
        let (f, t) = my_g.get_edge_nodes(i.clone());
        println!("id:{} from:{} to:{} label_1t2:{} weight_1t2:{}",
                my_g.get_edge_id(i.clone()),
                my_g.get_node_id(f),
                my_g.get_node_id(t),
                my_g.get_edge_label(i.clone()),
                my_g.get_edge_weight(i.clone()));
    }

    //select seeds
    // myG.select_seeds(SeedSelection::random, 1, 1);
    my_g.select_seeds(SeedSelection::MinDegree, 1, 1);

    println!("n1 out degree: {}", my_g.get_outdegrees(n1.clone()));
    println!("n1 : {:?}", n1.0.clone());
    println!("seed");
    for i in &my_g.seed{
        println!("seed node id: {}", i);
    }

    //initialize propagation, needs to be done after select seeds
    my_g.initialize_propagation();

    //run propagataion
    my_g.propagte(10);

    let nodes = my_g.get_nodes();

    println!("node label == 1");
    let mut counter = 0;
    for i in nodes{
        if my_g.get_node_label(i.clone()) == 1{
            println!("id:{} label:{} Threshold:{}", my_g.get_node_id(i.clone()), my_g.get_node_label(i.clone()), my_g.get_node_threshold(i.clone()));
            counter += 1;
        }
    }

    println!("result: {} nodes with label 1", counter);

    println!("get out degree: n1 {}", my_g.get_outdegrees(n1));

}
