use nalgebra::DMatrix;

use crate::gen::adjacency_matrices;

pub(crate) fn mathematicas(n: usize, connected: bool, v: Vec<Vec<usize>>) {
    let flattened = v.clone().into_iter().flatten().collect::<Vec<_>>();

    let vs = adjacency_matrices(n, connected)
        .enumerate()
        .filter(|(i, _)| flattened.contains(i))
        .collect::<Vec<_>>();

    for u in v {
        println!("-------------------");
        for j in u {
            let found = vs.clone().into_iter().find(|(i, _)| *i == j).unwrap();
            println!("{}", mathematica(n, &found.1));
        }
    }
}

fn mathematica(n: usize, graph: &DMatrix<u32>) -> String {
    const EDGE_SEPARATOR: &str = r"\[UndirectedEdge]";

    let mut list = String::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if graph[(i, j)] == 1 {
                list.push_str(&format!("{i}{EDGE_SEPARATOR}{j},"));
            }
        }
    }
    list.pop();

    format!("Graph[{{ {list} }}, VertexLabels -> \"Name\"]")
}
