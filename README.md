# rust_final_project_kitkat
//! An undirect graph with Node attributed and Edge attribute
//! 
//! This is a linkedlist graph
//! 
//! You can use this graph to do social network analysis
//! 
//! There is a build-in propagate method
//! 
//! You can choose propagation model: IC or LT
//! 
//! You can select seeds based on MaxDegree, MinDeggree, or Random
//! 
//! You can set threshold to be Random or all to a value 
//! 
//! You can set weights to be 1/degree, Random, or the same value
//! 
//! 
//! # Example
//! ```
//! use graph_propagation::*;
//! 
//! //build a graph with three nodes and 2 edges
//! let mut my_g = Graph::new(PropagationModel::IC);
//!
//! for _ in 0..25{
//!     my_g.add_node(MyNodeData::new());
//! }
//!
//! let nodes: Vec<Node> = my_g.get_nodes().collect();
//! //add edges
//! // 0--1--2--3--4
//! // |  |  |  |  |
//! // 5--6--7--8--9
//! // |  |  |  |  |
//! //10-11-12-13-14
//! // |  |  |  |  |
//! //15-16-17-18-19
//! // |  |  |  |  |
//! //20-21-22-23-24
//! for i in 0..5{
//!     for j in 0..4{
//!         println!("{} {} {}", i.clone()*5 + j.clone(), i.clone(), j.clone());
//!         my_g.add_edge(nodes[i*5 + j].clone(), nodes[i*5 + j + 1].clone(), MyEdgeData::new(i*5+j, i*5+j+1));
//!     }
//!     for j in 0..5{
//!         if i == 4{
//!             break;
//!         }
//!         my_g.add_edge(nodes[i*5 + j].clone(), nodes[(i+1)*5 + j].clone(), MyEdgeData::new(i*5+j, (i+1)*5+j));
//!     }
//! }
//! 
//! //initialize graph data setting
//! my_g.initialize_node_label();
//! my_g.initialize_node_threshold(ThresholdSet::Random);
//! my_g.initialize_edge_label();
//! my_g.initialize_weight(WeightSet::OneOverOutdegree);
//! 
//! //select seeds
//! my_g.select_seeds(SeedSelection::MinDegree, 5, 1);
//! 
//! //initialize propagation, needs to be done after select seeds
//! my_g.initialize_propagation();
//! 
//! //run propagataion
//! my_g.propagte(10);
//! 
//! ```
//! 