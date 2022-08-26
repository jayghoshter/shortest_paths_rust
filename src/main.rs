use std::collections::HashMap;
use std::collections::HashSet;
use std::ops;

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

struct UndirectedGraph{
    edges: Vec<UndirectedEdge>,
}

impl UndirectedGraph{
    fn get_node_neighbors_map(&self) -> HashMap::<i32, HashSet<i32>> {
        let mut neighbors = HashMap::<i32, HashSet<i32>>::new();

        for e in &self.edges {
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

        return neighbors
    }

    fn get_nodes(&self) -> HashSet<i32> {
        let mut all_nodes = HashSet::new();

        for e in &self.edges {
            all_nodes.insert(e.end_nodes.0);
            all_nodes.insert(e.end_nodes.1);
        }

        return all_nodes
    }
}

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
    fn print(&self){
        print!("Path with nodes: ");
        for i in 0..self.nodes.len() {
            print!("{},", self.nodes[i]);
        }
        println!("")
    }
}

impl UndirectedGraph{
    fn print(&self){
        print!("Graph with Edges: ");
        for i in 0..self.edges.len() {
            print!("{}--{}, ", self.edges[i].end_nodes.0, self.edges[i].end_nodes.1);
        }
        println!("")
    }
}

fn main(){

    let e1: UndirectedEdge = UndirectedEdge { end_nodes: (1, 2), length: 1.0 };
    let e2: UndirectedEdge = UndirectedEdge { end_nodes: (1, 3), length: 1.0 };
    let e3: UndirectedEdge = UndirectedEdge { end_nodes: (2, 4), length: 1.0 };
    let e4: UndirectedEdge = UndirectedEdge { end_nodes: (3, 4), length: 2.0 };

    // let e6: UndirectedEdge = UndirectedEdge { end_nodes: (3, 1), length: 2.0 };
    // let e7: UndirectedEdge = UndirectedEdge { end_nodes: (1, 2), length: 2.0 };

    let g1: UndirectedGraph = UndirectedGraph { edges: vec![e1,e2,e3,e4] };

    g1.print();
    let node_map = g1.get_node_neighbors_map();

    let start_path  = UndirectedPath{ nodes: [1].to_vec()};

    let new_paths = explore_path(start_path, node_map);


    // println!("{node_map:?}");

    // let e5: UndirectedEdge = UndirectedEdge { end_nodes: (3, 4), length: 2.0 };
    // println!("{}--{}", e5.end_nodes.0, e5.end_nodes.1);

    // println!("{}", e5.other_end(3));
    // println!("{}", e5.other_end(4));
    // println!("{}", e5.other_end(1));

    // println!("{}", e5.is_adjacent(e6));
    // println!("{}", e5.is_adjacent(e7));


}

fn explore_path(path: UndirectedPath, node_map: HashMap<i32, HashSet<i32>>) -> Vec<UndirectedPath> {
    let end_node = path.nodes.last().unwrap();
    let mut new_paths: Vec<UndirectedPath> = vec![];

    for next_node in node_map.get(&end_node).unwrap() {
        let new_path = &path + *next_node;
        new_paths.push(new_path);
    }

    new_paths
}
