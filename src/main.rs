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

struct UndirectedPath{
    nodes: Vec<i32>,
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
    let e5: UndirectedEdge = UndirectedEdge { end_nodes: (3, 4), length: 2.0 };

    let e6: UndirectedEdge = UndirectedEdge { end_nodes: (3, 1), length: 2.0 };
    let e7: UndirectedEdge = UndirectedEdge { end_nodes: (1, 2), length: 2.0 };

    let g1: UndirectedGraph = UndirectedGraph { edges: vec![e1,e2,e3,e4] };

    g1.print();

    println!("{}--{}", e5.end_nodes.0, e5.end_nodes.1);

    // println!("{}", e5.other_end(3));
    // println!("{}", e5.other_end(4));
    // println!("{}", e5.other_end(1));

    println!("{}", e5.is_adjacent(e6));
    println!("{}", e5.is_adjacent(e7));

}
