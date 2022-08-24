#[derive(Clone, Copy)]
struct Node {
    id: i32,
}

struct UndirectedEdge{
    end_nodes: (Node, Node),
    length: f64
}

impl UndirectedEdge{
    fn other_end(&self, n: Node) -> Node {
        let leftid = self.end_nodes.0.id;
        let rightid = self.end_nodes.1.id;
        match n.id {
            a if a == leftid => self.end_nodes.1,
            b if b == rightid => self.end_nodes.0,
            _ => panic!("Given node {} not part of edge {}--{}", n.id, leftid, rightid)
        }
    }

    fn is_adjacent(&self, other: UndirectedEdge) -> bool {
        self.end_nodes.0.id == other.end_nodes.0.id ||
            self.end_nodes.0.id == other.end_nodes.1.id ||
            self.end_nodes.1.id == other.end_nodes.0.id ||
            self.end_nodes.1.id == other.end_nodes.1.id 
    }
}

struct UndirectedGraph{
    edges: Vec<UndirectedEdge>,
}

struct UndirectedPath{
    nodes: Vec<Node>,
    length: f64
}

impl UndirectedGraph{
    fn print(&self){
        print!("Graph with Edges: ");
        for i in 0..self.edges.len() {
            print!("{}--{}, ", self.edges[i].end_nodes.0.id, self.edges[i].end_nodes.1.id);
        }
        println!("")
    }
}

fn main(){

    let n1: Node = Node { id:1 };
    let n2: Node = Node { id:2 };
    let n3: Node = Node { id:3 };
    let n4: Node = Node { id:4 };

    let e1: UndirectedEdge = UndirectedEdge { end_nodes: (n1, n2), length: 1.0 };
    let e2: UndirectedEdge = UndirectedEdge { end_nodes: (n1, n3), length: 1.0 };
    let e3: UndirectedEdge = UndirectedEdge { end_nodes: (n2, n4), length: 1.0 };
    let e4: UndirectedEdge = UndirectedEdge { end_nodes: (n3, n4), length: 2.0 };
    let e5: UndirectedEdge = UndirectedEdge { end_nodes: (n3, n4), length: 2.0 };

    let e6: UndirectedEdge = UndirectedEdge { end_nodes: (n3, n1), length: 2.0 };
    let e7: UndirectedEdge = UndirectedEdge { end_nodes: (n1, n2), length: 2.0 };

    let g1: UndirectedGraph = UndirectedGraph { edges: vec![e1,e2,e3,e4] };

    g1.print();

    println!("{}--{}", e5.end_nodes.0.id, e5.end_nodes.1.id);

    // println!("{}", e5.other_end(n3).id);
    // println!("{}", e5.other_end(n4).id);
    // println!("{}", e5.other_end(n1).id);

    println!("{}", e5.is_adjacent(e6));
    println!("{}", e5.is_adjacent(e7));

}
