use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::cmp::{Ordering, Ord};

#[derive(Eq, PartialEq, PartialOrd)]
struct ProQue {
    vertex: usize,
    cost: i32
}

struct Graph {
    grp: Vec<Vec<i32>>
}

impl Ord for ProQue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl ProQue {
    fn new(ver: usize, ct: i32) -> Self {
        ProQue {
            vertex: ver,
            cost: ct
        }
    }
}

impl Graph {
    fn new(size: usize) -> Self {
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); size];

        for i in 0..size {
            graph[i] = vec![-1; size];
        }

        Graph { grp: graph }
    }

    fn insert(&mut self, ver: usize, ed: usize, ct: i32) {
        let grp: &mut Vec<i32> = &mut self.grp[ver];
        
        grp[ed] = ct;
    }

    fn show(&self) {
        let mut node: usize = 0;
        
        for i in &self.grp {
            print!("{} -> [", node);
            node += 1;

            for k in i {
                print!(" {} ", k);    
            }

            println!("]");
        }

    }
}

fn do_dijkstra_algo(graph: &Graph, srt_pnt: usize, size: usize) -> Vec<i32> {
    let mut each_node_visit: Vec<i32> = vec![i32::MAX; size];
    let mut que: BinaryHeap<ProQue> = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; size];

    que.push(ProQue::new(srt_pnt, 0));
    each_node_visit[0] = 0;
    
    while let Some(element) = que.pop() {

        if visited[element.vertex] {
            continue;
        }

        visited[element.vertex] = true;
        let eds: &Vec<i32> = &graph.grp[element.vertex];

        for (i, c) in eds.iter().enumerate() {

            if *c != -1 {
                let tlt_sum: i32 = each_node_visit[element.vertex] + *c;
                
                if each_node_visit[i] > tlt_sum {
                    each_node_visit[i] = tlt_sum;
                    que.push(ProQue::new(i, tlt_sum));
                }

            }

        }

    }

    println!("each node lest cost: {:?}", each_node_visit);
    each_node_visit
}

fn find_path(graph: &Graph, mut e_cost: i32, end_pnt: usize, path: &mut Vec<usize>, size: usize, e_n_visited: Vec<i32>) {
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; size];
    
    que.push_back(end_pnt);
    visited[end_pnt] = true;

    while let Some(element) = que.pop_front() {
        let eds: &Vec<i32> = &graph.grp[element];

        for (n, c) in eds.iter().enumerate() {

            if *c != -1 {

                if !visited[n] {
                    let tlt_sum: i32 = e_cost - *c;

                    if e_n_visited.contains(&tlt_sum) {
                        path.push(n);
                        que.push_back(n);
                        e_cost = tlt_sum;
                    }

                    visited[n] = true;
                }

            }

        }

    }

}

fn main() {
    let mut graph: Graph = Graph::new(5);
    graph.show();

    graph.insert(0, 0, 0);
    graph.insert(0, 1, 10);
    graph.insert(0, 2, 7);
    graph.insert(1, 0, 10);
    graph.insert(1, 2, 5);
    // graph.insert(1, 3, 5);
    graph.insert(1, 4, 12);
    graph.insert(2, 1, 5);
    graph.insert(2, 4, 9);
    graph.insert(2, 3, 5);
    graph.insert(3, 4, 10);
    graph.insert(3, 2, 5);
    // graph.insert(3, 1, 5);
    graph.insert(4, 1, 12);
    graph.insert(4, 2, 9);
    graph.insert(4, 3, 10);

    println!("\n");
    graph.show();

    let each_node_visited: Vec<i32> = do_dijkstra_algo(&graph, 0, 5);
    let mut path: Vec<usize> = Vec::new();

    path.push(0);
    find_path(&graph, 16, 4, &mut path, 5, each_node_visited);
    path.push(4);

    println!("{:?}", path);
}