use std::env;

use rand::Rng;

fn main() {
    let data_size = env::args()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap_or(100000000);
    let mut data = Vec::new();
    (0..data_size).for_each(|_| {
        data.push(random(0, 1e8 as u32));
        // data.push();
    });

    let out_path = env::current_dir().unwrap().join("data.txt");
    std::fs::write(
        out_path,
        data.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .unwrap();
}

fn random(min: u32, max: u32) -> u32 {
    rand::rng().random_range(min..max)
}
