use nannou::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use nannou::app::{Draw};
use rand::Rng;

struct Particle {
    _perlin_x: Perlin,
    _perlin_y: Perlin,
    _noise_diameter: f32,
    _radius: f32,
}

impl Particle {
    fn new(_noise_diameter: f32, _radius: f32) -> Particle {
        let mut rng = rand::thread_rng();

        let _perlin = Perlin::new();
        let _perlin_x = _perlin.set_seed(rng.gen());
        let _perlin_y = _perlin.set_seed(rng.gen());

        Particle {
            _perlin_x: _perlin_x,
            _perlin_y: _perlin_y,
            _noise_diameter: _noise_diameter,
            _radius: _radius,
        }
        
    }

    fn get_xy(&self, _coords: [f64; 2]) -> Point2 {
        let _x = map_values(self._perlin_x.get(_coords) as f32, -1.0, 1.0, -self._noise_diameter, self._noise_diameter);
        let _y = map_values(self._perlin_y.get(_coords) as f32, -1.0, 1.0, -self._noise_diameter, self._noise_diameter);
        pt2(_x, _y)
    }

    fn draw(&self, _draw: &Draw, _coords: &[f64; 2]) {
        let _xy = self.get_xy(_coords);
        _draw.ellipse()
            .xy(_xy)
            .radius(self._radius)
            .color(WHITE);
    
    }
}

fn map_values(n: f32, start_1: f32, stop_1: f32, start_2: f32, stop_2: f32) -> f32 {
    ((n - start_1) / (stop_1 - start_1)) * (stop_2 - start_2) + start_2
}