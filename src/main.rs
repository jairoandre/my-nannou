use noise::{NoiseFn, Perlin, Seedable};

fn main() {
    let _perlin = Perlin::new();

    _perlin.set_seed(323);

    let val = _perlin.get([42.4, 37.7, 2.8]);

    println!("{}", val);

    //let _fbm =  Fbm::new();

    for x in 0..10 {
        for y in 0..10 {
            println!("{}", _perlin.get([x as f64 + 0.2, y as f64 + 0.2]));
        }
    }
}

fn map_values(n: f32, start_1: f32, stop_1: f32, start_2: f32, stop_2: f32) -> f32 {
    ((n - start_1) / (stop_1 - start_1)) * (stop_2 - start_2) + start_2
}