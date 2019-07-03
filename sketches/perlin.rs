use nannou::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

struct Model {
    _perlin_x: Perlin,
    _perlin_y: Perlin,
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

    let _perlin = Perlin::new();

    Model {
        _perlin_x: _perlin.set_seed(34),
        _perlin_y: _perlin.set_seed(23),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}

const NOISE_DIAMETER: f32 = 500.0;

fn get_xy(_model: &Model, _coords: [f64; 2]) -> Point2 {
    let _x = map_values(_model._perlin_x.get(_coords) as f32, -1.0, 1.0, -200.0, 200.0);
    let _y = map_values(_model._perlin_y.get(_coords) as f32, -1.0, 1.0, -200.0, 200.0);
    pt2(_x, _y)
}

fn map_values(n: f32, start_1: f32, stop_1: f32, start_2: f32, stop_2: f32) -> f32 {
    ((n - start_1) / (stop_1 - start_1)) * (stop_2 - start_2) + start_2
}

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let _draw = _app.draw();
    let _time = _app.time * 0.001;
    _draw.background().color(BLACK);
    let _noise_coords = [(NOISE_DIAMETER * _time.cos()) as f64, (NOISE_DIAMETER * _time.sin()) as f64];
    let _xy = get_xy(_model, _noise_coords);
    _draw.ellipse()
        .xy(_xy)
        .radius(10.0)
        .color(WHITE);
    _draw.to_frame(&_app, &frame).unwrap();
    frame
}