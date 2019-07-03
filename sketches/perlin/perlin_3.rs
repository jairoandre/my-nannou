use nannou::prelude::*;
use noise::{NoiseFn, Perlin, Fbm, Seedable};
use nannou::app::{Draw};
use rand::Rng;

struct Model {
    _particles: Vec<Particle>,
    _perlin: Perlin,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(_app: &App) -> Model {

    let _window = _app
        .new_window()
        .with_dimensions(800, 800)
        .view(view)
        .build()
        .unwrap();
    
    let mut _particles: Vec<Particle> = Vec::new();

    let _perlin = Perlin::new();

    let _inc = 0.025;
    
    for y in 0..100 {
        for x in 0..100 {
            let _coords = [x as f64 * _inc, y as f64 * _inc];
            let _perlin_value = _perlin.get(_coords);
            let _idx = y * 100 + (x % 100);
            _particles.push(Particle::new(_idx, _perlin_value as f32))
        }
    }

    Model {
        _particles: _particles,
        _perlin: Perlin::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let _draw = _app.draw();
    let _time = _app.time * 0.1;
    _draw.background().color(BLACK);

    for particle in &_model._particles {
        particle.draw(&_draw)
    }

    _draw.to_frame(&_app, &frame).unwrap();
    frame
}

struct Particle {
    _idx: i32,
    _perlin_value: f32,
}

const SCALE: f32 = 5.0;
const BORDER: f32 = 0.0;

impl Particle {
    fn new(_idx: i32, _perlin_value: f32) -> Particle {
        let _red = map_values(_perlin_value, -1.0, 1.0, 0.0, 255.0);
        Particle {
            _idx: _idx,
            _perlin_value: _perlin_value,
        }
    }

    fn get_xy(&self, _idx: i32) -> Point2 {
        let _x = ((_idx % 100) as f32 * (SCALE + BORDER)) - 400.0 + BORDER;
        let _y = ((_idx / 100) as f32 * - (SCALE + BORDER)) + 400.0 - BORDER;
        pt2(_x, _y)
    }

    fn draw(&self, _draw: &Draw) {
        let _xy = self.get_xy(self._idx);
        let _red = map_values(self._perlin_value, -1.0, 1.0, 0.0, 1.0);
        
        _draw.quad()
            .xy(_xy)
            .width(SCALE)
            .height(SCALE)
            .rgb(_red, 0.0, 0.0);
    
    }
}

fn map_values(n: f32, start_1: f32, stop_1: f32, start_2: f32, stop_2: f32) -> f32 {
    ((n - start_1) / (stop_1 - start_1)) * (stop_2 - start_2) + start_2
}