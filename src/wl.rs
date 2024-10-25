use crate::DMatrix;

pub(crate) fn embed(n: usize, A: &DMatrix<u32>) -> [u8; 32] {
    let mut colours = vec![[0; 32]; n];
    let mut num_colours = 1;

    loop {
        let mut new_colours = vec![[0; 32]; n];

        for (v, adjacency) in A.row_iter().enumerate() {
            let mut neighbour_colours = Vec::new();
            for neighbour in adjacency
                .into_iter()
                .enumerate()
                .filter_map(|(u, is_adjacent)| if *is_adjacent == 1 { Some(u) } else { None })
            {
                neighbour_colours.push(colours[neighbour].into());
            }

            let mut hasher = blake3::Hasher::new();
            hasher.update(&colours[v]);
            multiset_hash(&mut hasher, neighbour_colours);
            let new_colour = hasher.finalize().into();

            new_colours[v] = new_colour;
        }

        let num_new_colours = count_unique(new_colours.clone());

        if num_new_colours == num_colours {
            break;
        } else {
            num_colours = num_new_colours;
            colours = new_colours;
        }
    }

    let mut hasher = blake3::Hasher::new();
    multiset_hash(&mut hasher, colours);
    hasher.finalize().into()
}

fn count_unique(mut v: Vec<[u8; 32]>) -> usize {
    v.sort_unstable();
    v.partition_dedup().0.len()
}

fn multiset_hash(hasher: &mut blake3::Hasher, mut v: Vec<[u8; 32]>) {
    v.sort_unstable();
    for colour in v {
        hasher.update(&colour);
    }
}
