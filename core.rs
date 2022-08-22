struct Node {
    id: i32,
}

struct UndirectedEdge{
    end_nodes: (Node, Node),
    length: f64
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
    let n1: Node = Node { id:1 , };
    let n2: Node = Node { id:2 , };

    let e1: UndirectedEdge = UndirectedEdge { end_nodes : (n1,n2), length:1.0 };

    let g1: UndirectedGraph = UndirectedGraph { edges: vec![e1] };

    g1.print()

}
