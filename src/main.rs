use noise::{NoiseFn, Perlin, Fbm, Seedable};

fn main() {
    let _perlin = Perlin::new();

    for x in 0..10 {
        for y in 0..10 {
            println!("{}", 1000.0 * _perlin.get([x as f64, y as f64]));
        }
    }

    let _fbm =  Fbm::new();

    for x in 0..10 {
        for y in 0..10 {
            println!("{}", 1000.0 * _fbm.get([x as f64, y as f64]));
        }
    }
}