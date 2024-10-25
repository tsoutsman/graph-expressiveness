use nalgebra::DMatrix;
use std::cmp::max;

pub(crate) fn embed(n: usize, A: &DMatrix<u32>) -> [u8; 32] {
    let mut result = Vec::new();
    let mut An = A.clone();

    // At the i-th iteration we are computing $A^{i + 2}$.
    // We can skip A^1 because $\diag[A^1] = 0$.
    for _ in 0..=(max(2, n) - 2) {
        An *= A;
        result.push(
            An.clone()
                .diagonal()
                .into_iter()
                .copied()
                .collect::<Vec<_>>(),
        );
    }

    hash(n, result)
}

pub(crate) fn hash(n: usize, input: Vec<Vec<u32>>) -> [u8; 32] {
    let mut zeta_0s = Vec::new();
    for i in 0..n {
        let mut zeta_0 = Vec::new();

        for diagonal in input.iter() {
            zeta_0.push(diagonal[i]);
        }

        zeta_0s.push(zeta_0);
    }

    let mut hasher = blake3::Hasher::new();
    // Only the top vec is treated as a multiset.
    // Each of the zeta_0 is still treated as an order tuple.
    multiset_hash(&mut hasher, zeta_0s);
    hasher.finalize().into()
}

fn multiset_hash(hasher: &mut blake3::Hasher, mut set: Vec<Vec<u32>>) {
    set.sort_unstable();
    for s in set {
        hasher.update(u32_to_u8(&s));
    }
}

// Taken from:
// https://users.rust-lang.org/t/transmute-u32-to-u8/63937/2
fn u32_to_u8(arr: &[u32]) -> &[u8] {
    let len = 4 * arr.len();
    let ptr = arr.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

// The two strongly regular graphs we test have order 16.
// If we don't convert to u64, `An` overflows.

pub(crate) fn embed_large(n: usize, A: &DMatrix<u32>) -> [u8; 32] {
    // Convert the `DMatrix<u32>` to a `DMatrix<u64>`.
    let mut matrix = DMatrix::from_element(n, n, 0u64);
    for (from, to) in A
        .data
        .as_vec()
        .iter()
        // SAFETY: We do not change the size of the vector.
        .zip(unsafe { matrix.data.as_vec_mut() }.iter_mut())
    {
        *to = (*from).into();
    }
    let A = &matrix;

    let mut result = Vec::new();
    let mut An = A.clone();

    // At the i-th iteration we are computing $A^{i + 2}$.
    // We can skip A^1 because $\diag[A^1] = 0$.
    for _ in 0..=(n - 2) {
        An *= A;
        result.push(
            An.clone()
                .diagonal()
                .into_iter()
                .copied()
                .collect::<Vec<_>>(),
        );
    }

    hash_large(result)
}

pub(crate) fn hash_large(input: Vec<Vec<u64>>) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    for diagonal in input.into_iter() {
        multiset_hash_large(&mut hasher, diagonal);
    }
    hasher.finalize().into()
}

fn multiset_hash_large(hasher: &mut blake3::Hasher, mut set: Vec<u64>) {
    set.sort_unstable();
    hasher.update(u64_to_u8(&set));
}

fn u64_to_u8(arr: &[u64]) -> &[u8] {
    let len = 8 * arr.len();
    let ptr = arr.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, len) }
}
