/*************************************************************************************************************
 * This program was written to get started with learning rust. It is based on a coding challenge I
 * was recently given. The code is by no means perfect, but the algorithm should be functional.
 * Tests and helper functions should be added shortly. 
 *
 * The goal of the program is to take an undirected graph, and find all the paths (sorted) between
 * a start and end node that are less than or equal to a length limit specified by length_tol * shortest path.
 * Cyclic paths should be allowed, i.e., 1-2-1-2-4 and 1-3-1-2-4 etc should be valid paths between
 * nodes 1 and 4 as long as the total path length is within the limit.
 *************************************************************************************************************/

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops;
use rayon::prelude::*;
use itertools::Itertools;

struct UnEdge{
    end_nodes: (i32, i32),
    length: f64
}

impl UnEdge{

    fn new(end_nodes:(i32,i32), length:f64) -> UnEdge{
        UnEdge { end_nodes: end_nodes, length: length}
    }

    fn other_end(&self, n: i32) -> i32 {
        match n {
            a if a == self.end_nodes.0 => self.end_nodes.1,
            b if b == self.end_nodes.1 => self.end_nodes.0,
            _ => panic!("Given node {} not part of edge {:?}", n, self.end_nodes)
        }
    }

    fn is_adjacent(&self, other: UnEdge) -> bool {
        self.end_nodes.0 == other.end_nodes.0 ||
            self.end_nodes.0 == other.end_nodes.1 ||
            self.end_nodes.1 == other.end_nodes.0 ||
            self.end_nodes.1 == other.end_nodes.1 
    }
}

#[derive(Debug,Clone)]
struct UnPath{
    nodes: Vec<i32>,
}

impl ops::Add<i32> for &UnPath{
    type Output = UnPath;

    fn add(self, _rhs: i32) -> UnPath {
        UnPath { nodes: [(*self.nodes).to_vec(),  [_rhs].to_vec()].concat()}
    }
}

impl UnPath{
    fn edges(&self) -> Vec<(&i32, &i32)> {
        self.nodes.iter().tuple_windows::<(_,_)>().collect()
    }

    fn length(&self, graph: &UnGraph) -> f64 {
        let mut sum:f64 = 0.0;

        for item in self.edges() {
            sum += graph.edge_length_map.get(&(*item.0, *item.1)).unwrap()
        }

        sum
    }

    fn print(&self){
        for i in 0..self.nodes.len()-1 {
            print!("{}->", self.nodes[i]);
        }
        print!("{} \n", self.nodes.last().unwrap());
    }

    fn print_with_length(&self, g: &UnGraph) {
        for i in 0..self.nodes.len()-1 {
            print!("{}->", self.nodes[i]);
        }
        print!("{}: {} \n", self.nodes.last().unwrap(), self.length(g));
    }

    fn explore(&self, graph: &UnGraph, length_limit:Option<f64>) -> Vec<UnPath> {
        let end_node = self.nodes.last().unwrap();
        let mut new_paths: Vec<UnPath> = vec![];

        for next_node in graph.node_neighbors_map.get(&end_node).unwrap() {
            let new_path = self + *next_node;

            match length_limit {
                None => new_paths.push(new_path),
                Some(f) => if new_path.length(graph) <= f {new_paths.push(new_path)}
            };

            // if new_path.length(graph) <= length_limit { 
            //     new_paths.push(new_path);
            // }
        }

        new_paths
    }
}

struct UnGraph{
    edges: Vec<UnEdge>,
    edge_length_map: HashMap::<(i32,i32), f64>,
    node_neighbors_map: HashMap::<i32, HashSet<i32>>
}

impl UnGraph{

    fn new(_edges: Vec<UnEdge>) -> UnGraph {

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

        UnGraph { edges: _edges, edge_length_map: _edge_length_map, node_neighbors_map: neighbors } 
    }

    fn from_tuples(_edge_tuples: Vec<(i32, i32, f64)>) -> UnGraph{

        let _edges : Vec<UnEdge> = _edge_tuples.iter().map(|t| UnEdge{end_nodes: (t.0, t.1), length: t.2}).collect();

        UnGraph::new(_edges)
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

    fn compute_shortest_paths(&self, start:i32, end:i32, length_tol:f64) -> Vec<UnPath> {

        if length_tol < 1.0 {
            return vec![]
        }

        let mut paths_to_explore: Vec<UnPath> = vec![UnPath{ nodes: vec![start] }];
        let mut results: Vec<UnPath> = vec![];
        let mut length_limit:Option<f64>;
        let mut count: usize = 0;

        while paths_to_explore.len() > 0 {
            count += 1;

            let current_min_path = results.iter().min_by(|a,b| a.length(&self).partial_cmp(&b.length(&self)).unwrap());

            length_limit = match current_min_path {
                None => None,
                Some(p) => Some(p.length(&self) * length_tol)
            };

            let new_paths: Vec<_> = paths_to_explore.par_iter()
                .map(|p| p.explore(&self, length_limit)).into_par_iter().flatten().collect();


            // TODO: Parallelize
            for path in &new_paths {
                if path.nodes.last().unwrap() == &end {
                    results.push(path.clone());
                }
            }

            paths_to_explore = new_paths;

            // Early break for disconnected graphs
            if &self.edges.len() < &self.node_neighbors_map.keys().len() {
                if &count == &self.node_neighbors_map.keys().len() {
                    if results.len() == 0 {
                        return vec![]
                    }
                }
            }
        }

        // sort
        results = results.into_iter().sorted_by(|a,b| a.length(&self).partial_cmp(&b.length(&self)).unwrap()).collect_vec();

        length_limit = match results.first() {
            None => None,
            Some(p) => Some(p.length(&self) * length_tol)
        };

        // prune
        results = results.into_iter().filter(|p| p.length(&self) <= length_limit.unwrap()).collect_vec();
        
        return results
    }
}

fn main(){

    let e1: UnEdge = UnEdge { end_nodes: (1, 2), length: 1.0 };
    let e2: UnEdge = UnEdge { end_nodes: (1, 3), length: 1.0 };
    let e3: UnEdge = UnEdge { end_nodes: (2, 4), length: 1.0 };
    let e4: UnEdge = UnEdge { end_nodes: (3, 4), length: 2.0 };

    let g1: UnGraph = UnGraph::new(vec![e1,e2,e3,e4]) ;

    let g2: UnGraph = UnGraph::from_tuples(vec![(1,2,1.0), (2,4,1.0), (1,3,1.0), (3,4,2.0) ]);

    g1.print();
    g1.compute_shortest_paths(1, 4, 1.0).iter().for_each(|p| p.print_with_length(&g2));

    g2.print();
    g2.compute_shortest_paths(1, 4, 2.0).iter().for_each(|p| p.print_with_length(&g2));

}
