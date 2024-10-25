//! Module for generating (or rather reading generated) adjacency matrices.
//!
//! The adjacency matrices must first be generated using `gen.sh`.

use std::{
    io::{BufRead, BufReader, Lines},
    process::{ChildStdout, Command, Stdio},
};

use nalgebra::DMatrix;

/// Returns an iterator over the adjacency matrices
/// of all graphs (up to isomorphism) with `n` vertices.
///
/// If `connected` is true then only adjacency matrices for connected graphs are
/// generated.
pub(crate) fn adjacency_matrices(n: usize, connected: bool) -> Iter {
    let file_name = if connected {
        format!("cgraphs{n}.txt")
    } else {
        format!("graphs{n}.txt")
    };

    let cat = Command::new("cat")
        .arg(file_name)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // Would be faster if we implemented the logic to decode graphs directly in
    // Rust, but 🤷.
    let showg = Command::new("showg")
        .stdin(Stdio::from(cat.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let lines = BufReader::new(showg.stdout.unwrap()).lines().into_iter();

    Iter { n, lines }
}

pub(crate) struct Iter {
    n: usize,
    lines: Lines<BufReader<ChildStdout>>,
}

impl Iterator for Iter {
    type Item = DMatrix<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        assert!(self.lines.next()?.unwrap().is_empty());
        self.lines.next().unwrap().unwrap();

        let mut result = DMatrix::from_element(self.n, self.n, 0);

        for _ in 0..self.n {
            let line = self.lines.next().unwrap().unwrap();
            let mut split = line
                .strip_prefix("  ")
                .unwrap()
                .strip_suffix(';')
                .unwrap()
                .split(' ');

            let v: usize = split.next().unwrap().parse().unwrap();
            assert_eq!(":", split.next().unwrap());

            split.filter_map(|n| n.parse().ok()).for_each(|n| {
                result[(n, v)] = 1;
                result[(v, n)] = 1;
            });
        }

        Some(result)
    }
}
