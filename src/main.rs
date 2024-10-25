#![feature(slice_partition_dedup, iter_map_windows)]
#![allow(non_snake_case)]

mod gen;
mod id_gnn_fast;
mod mathematica;
mod wl;

use fxhash::FxBuildHasher;
use nalgebra::DMatrix;

fn main() {
    let connected = false;
    let max_vertices = 10;

    for n in 1..=max_vertices {
        println!("------- {n} -------");
        let id_result = find_indistinguishable(n, connected, id_gnn_fast::embed);
        display(id_result.clone());
        let id_flattened = id_result.into_iter().flatten();
        let id_count = id_flattened.clone().count();
        println!("     id: {id_count}");

        let wl_result = find_indistinguishable(n, connected, wl::embed);
        display(wl_result.clone());
        let wl_flattened = wl_result.into_iter().flatten();
        let wl_count = wl_flattened.clone().count();
        println!("     wl: {wl_count}");

        // There are better ways of doing this, but this is not the bottleneck.
        use std::collections::HashSet;

        let id_hash = HashSet::<_, FxBuildHasher>::from_iter(id_flattened);
        let wl_hash = HashSet::<_, FxBuildHasher>::from_iter(wl_flattened);
        let union = id_hash.intersection(&wl_hash);

        println!("overlap: {}", union.count());
    }
}

fn display(result: Vec<Vec<usize>>) {
    let mut result = result.into_iter().map(|c| c.len()).collect::<Vec<_>>();
    result.sort_unstable();

    let mut previous = 2;
    let mut count = 0;

    for i in result {
        if i != previous {
            if count > 0 {
                println!("{:?}: {:?}", previous, count);
            }
            previous = i;
            count = 1;
        }
        count += 1;
    }

    if count > 0 {
        println!("{:?}: {:?}", previous, count);
    }
}

pub(crate) fn find_indistinguishable<T, F>(n: usize, connected: bool, embed: F) -> Vec<Vec<usize>>
where
    T: Ord + PartialEq,
    F: Fn(usize, &DMatrix<u32>) -> T,
{
    _find_indistinguishable(n, gen::adjacency_matrices(n, connected).into_iter(), embed)
}

pub(crate) fn _find_indistinguishable<I, T, F>(n: usize, iter: I, embed: F) -> Vec<Vec<usize>>
where
    I: Iterator<Item = DMatrix<u32>>,
    T: Ord + PartialEq,
    F: Fn(usize, &DMatrix<u32>) -> T,
{
    let mut hashes = iter
        .map(|A| embed(n, &A))
        .enumerate()
        .map(|(i, hash)| (hash, i))
        .collect::<Vec<_>>();

    hashes.sort_unstable();

    let mut i = 0;

    let mut collisions = Vec::new();

    while i < hashes.len() {
        let (hash, j) = &hashes[i];

        let mut next = 0;

        let mut collision = vec![*j];
        while (i + next + 1) < hashes.len() && (&hashes[i + next + 1].0 == hash) {
            collision.push(hashes[i + next + 1].1);
            next += 1;
        }

        if next > 0 {
            collisions.push(collision);
        }
        i += next;
        i += 1;
    }

    collisions
}

#[cfg(test)]
mod tests {
    use nalgebra::DMatrix;

    use super::*;

    #[test]
    fn test_strongly_regular() {
        // ID-GNN-Fast cannot distinguish regular graphs.
        assert_eq!(
            _find_indistinguishable(16, graphs().into_iter(), id_gnn_fast::embed_large),
            vec![vec![0, 1]]
        );

        pub fn graphs() -> Vec<DMatrix<u32>> {
            const STRING: [&str; 2] = [
                "0111111000000000
1011000111000000
1101000000111000
1110000000000111
1000011100100100
1000101010010010
1000110001001001
0100100011100100
0100010101010010
0100001110001001
0010100100011100
0010010010101010
0010001001110001
0001100100100011
0001010010010101
0001001001001110
",
                "0111111000000000
1011000111000000
1100100100110000
1100010010001100
1010001000101010
1001001000010101
1000110001000011
0110000001010110
0101000001101001
0100001110000011
0010100010011001
0010010100100101
0001100010100110
0001010100011010
0000101101001100
0000011011110000
",
            ];

            let mut result = Vec::new();

            for s in STRING {
                let mut temp = DMatrix::from_element(16, 16, 0);

                for (i, line) in s.lines().enumerate() {
                    for (j, c) in line.chars().enumerate() {
                        match c {
                            '0' => {}
                            '1' => temp[(i, j)] = 1,
                            _ => panic!(),
                        }
                    }
                }
                result.push(temp);
            }

            result
        }
    }
}
