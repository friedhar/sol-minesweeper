use rand::{thread_rng, Rng};

pub fn grid_to_string(grid: &[u8], width: usize) -> String {
    // could have done impl ToString
    // This impl is absolutely disgusting, don't like it, allocates way more than needed, in random places, would optimize as soon as possible
    grid.chunks(width)
        .map(|row| {
            row.into_iter()
                .map(|digit| digit.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n") // can later optimize it, allocates way too much
}

pub fn random_grid(width: usize, height: usize) -> Vec<u8> {
    let mut rng = thread_rng();

    let n = width * height; // can't be greater than 100
    let mut o: Vec<u8> = Vec::with_capacity(n); // preallocate for greater perf

    for _ in 0..n {
        let v = rng.gen_range(0..10);
        o.push(v);
    }

    o
}
