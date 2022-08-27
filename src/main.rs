use std::collections::HashMap;
use std::collections::HashSet;
use std::ops;
use rayon::prelude::*;
use itertools::Itertools;

struct UndirectedEdge{
    end_nodes: (i32, i32),
    length: f64
}

impl UndirectedEdge{
    fn other_end(&self, n: i32) -> i32 {
        match n {
            a if a == self.end_nodes.0 => self.end_nodes.1,
            b if b == self.end_nodes.1 => self.end_nodes.0,
            _ => panic!("Given node {} not part of edge {:?}", n, self.end_nodes)
        }
    }

    fn is_adjacent(&self, other: UndirectedEdge) -> bool {
        self.end_nodes.0 == other.end_nodes.0 ||
            self.end_nodes.0 == other.end_nodes.1 ||
            self.end_nodes.1 == other.end_nodes.0 ||
            self.end_nodes.1 == other.end_nodes.1 
    }
}


#[derive(Debug,Clone)]
struct UndirectedPath{
    nodes: Vec<i32>,
}

impl ops::Add<i32> for &UndirectedPath{
    type Output = UndirectedPath;

    fn add(self, _rhs: i32) -> UndirectedPath {
        UndirectedPath { nodes: [(*self.nodes).to_vec(),  [_rhs].to_vec()].concat()}
    }
}

impl UndirectedPath{
    fn edges(&self) -> Vec<(&i32, &i32)> {
        self.nodes.iter().tuple_windows::<(_,_)>().collect()
    }

    fn length(&self, graph: &UndirectedGraph) -> f64 {
        let mut sum:f64 = 0.0;

        for item in self.edges() {
            sum += graph.edge_length_map.get(&(*item.0, *item.1)).unwrap()
        }

        sum
    }

    fn print(&self){
        print!("Path with nodes: ");
        for i in 0..self.nodes.len() {
            print!("{},", self.nodes[i]);
        }
        println!("")
    }

    fn explore(&self, graph: &UndirectedGraph, length_limit:f64) -> Vec<UndirectedPath> {
        let end_node = self.nodes.last().unwrap();
        let mut new_paths: Vec<UndirectedPath> = vec![];

        for next_node in graph.node_neighbors_map.get(&end_node).unwrap() {
            let new_path = self + *next_node;

            if new_path.length(graph) <= length_limit { 
                new_paths.push(new_path);
            }
        }

        new_paths
    }
}

struct UndirectedGraph{
    edges: Vec<UndirectedEdge>,
    edge_length_map: HashMap::<(i32,i32), f64>,
    node_neighbors_map: HashMap::<i32, HashSet<i32>>
}

impl UndirectedGraph{

    fn new(_edges: Vec<UndirectedEdge>) -> UndirectedGraph {

        let mut _edge_length_map: HashMap::<(i32,i32), f64> = HashMap::<(i32, i32), f64>::new() ;

        for e in &_edges {
            _edge_length_map.insert(e.end_nodes, e.length);
            _edge_length_map.insert((e.end_nodes.1, e.end_nodes.0), e.length);
        }

        let mut neighbors = HashMap::<i32, HashSet<i32>>::new();

        for e in &_edges {
            let (left, right) = e.end_nodes;
            let lhs = HashSet::from([left]);
            let rhs = HashSet::from([right]);

            neighbors.entry(left)
                .and_modify(|e| *e = e.union(&rhs).copied().collect::<HashSet<i32>>() )
                .or_insert(HashSet::from([right]));

            neighbors.entry(right)
                .and_modify(|e| *e = e.union(&lhs).copied().collect::<HashSet<i32>>() )
                .or_insert(HashSet::from([left]));
        };

        UndirectedGraph { edges: _edges, edge_length_map: _edge_length_map, node_neighbors_map: neighbors } 
    }

    fn get_nodes(&self) -> HashSet<i32> {
        let mut all_nodes = HashSet::new();

        for e in &self.edges {
            all_nodes.insert(e.end_nodes.0);
            all_nodes.insert(e.end_nodes.1);
        }

        return all_nodes
    }

    fn print(&self){
        print!("Graph with Edges: ");
        for i in 0..self.edges.len() {
            print!("{}--{}, ", self.edges[i].end_nodes.0, self.edges[i].end_nodes.1);
        }
        println!("")
    }

    fn compute_shortest_paths(&self, start:i32, end:i32, length_tol:f64) -> Vec<UndirectedPath> {

        if length_tol < 1.0 {
            return vec![]
        }

        let mut paths_to_explore: Vec<UndirectedPath> = vec![UndirectedPath{ nodes: vec![start] }];
        let mut results: Vec<UndirectedPath> = vec![];
        let length_limit:f64 = 0.0;

        while paths_to_explore.len() > 0 {
            let new_paths: Vec<_> = paths_to_explore.par_iter()
                .map(|p| p.explore(&self, 2.0)).into_par_iter().flatten().collect();

            // TODO: Parallelize
            for path in &new_paths {
                if path.nodes.last().unwrap() == &end {
                    results.push(path.clone());
                }
            }

            paths_to_explore = new_paths;
        }

        println!("Done!");
        return results
    }
}

fn main(){

    let e1: UndirectedEdge = UndirectedEdge { end_nodes: (1, 2), length: 1.0 };
    let e2: UndirectedEdge = UndirectedEdge { end_nodes: (1, 3), length: 1.0 };
    let e3: UndirectedEdge = UndirectedEdge { end_nodes: (2, 4), length: 1.0 };
    let e4: UndirectedEdge = UndirectedEdge { end_nodes: (3, 4), length: 2.0 };

    // let e6: UndirectedEdge = UndirectedEdge { end_nodes: (3, 1), length: 2.0 };
    // let e7: UndirectedEdge = UndirectedEdge { end_nodes: (1, 2), length: 2.0 };

    let g1: UndirectedGraph = UndirectedGraph::new(vec![e1,e2,e3,e4]) ;

    g1.print();

    for item in &g1.node_neighbors_map {
        println!("{}: {:?}", item.0, item.1);
    }

    for item in &g1.edge_length_map {
        println!("{:?}: {:?}", item.0, item.1);
    }
    //
    // let p : UndirectedPath =  UndirectedPath {nodes: vec![1,2,4]};
    // println!("{:?}", p.edges());
    //
    // let len = p.length(&g1);
    // println!("{len}");

    println!("{:?}", g1.compute_shortest_paths(1, 4, 1.0));

    // let node_map = g1.get_node_neighbors_map();

    // println!("{node_map:?}");

    // let e5: UndirectedEdge = UndirectedEdge { end_nodes: (3, 4), length: 2.0 };
    // println!("{}--{}", e5.end_nodes.0, e5.end_nodes.1);

    // println!("{}", e5.other_end(3));
    // println!("{}", e5.other_end(4));
    // println!("{}", e5.other_end(1));

    // println!("{}", e5.is_adjacent(e6));
    // println!("{}", e5.is_adjacent(e7));


}

