use nannou::prelude::*;
use nannou::app::{Draw};
use rand::Rng;

struct Model {
    _particles: Vec<Particle>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const WINDOW_SIZE: u32 = 800;


fn model(_app: &App) -> Model {

    let _window = _app
        .new_window()
        .with_dimensions(WINDOW_SIZE, WINDOW_SIZE)
        .view(view)
        .build()
        .unwrap();
    let mut rng = rand::thread_rng();

    let mut _particles: Vec<Particle> = Vec::new();

    for _ in 0..10 {
        let _x = map_range(rng.gen(), 0.0, 1.0, -200.0, 200.0);
        let _y_velocity = map_range(rng.gen(), 0.0, 1.0, 5.0, 10.0);
        _particles.push(Particle::new(pt2(_x, 0.0), pt2(0.0, _y_velocity), true));
    }

    Model {
        _particles: _particles,
    }
}

const LIFESPAN: u32 = 200;

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let _gravity = pt2(0.0, -0.2);

    let mut rng = rand::thread_rng();

    let _rnd_x = map_range(rng.gen(), 0.0, 1.0, -400.0, 400.0);
    let _y_vel = map_range(rng.gen(), 0.0, 1.0, 5.0, 15.0);

    let _position = pt2(_rnd_x, -200.0);
    let _velocity = pt2(0.0, _y_vel);
    let _acceleration = pt2(0.0, 0.0);

    let _test: f32 = rng.gen();

    if _test < 0.1 {
        _model._particles.push(Particle::new(_position, _velocity, true));
    }

    for _particle in _model._particles.iter_mut() {
        _particle.apply_force(_gravity);
        _particle.update();
    }

    _model._particles.retain(|p| p._life_span < LIFESPAN);

}


fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let _draw = _app.draw();
    _draw.background().color(BLACK);

    for _particle in &_model._particles {
        _particle.draw(&_draw);
    }

    _draw.to_frame(&_app, &frame).unwrap();
    frame
}

struct Particle {
    _position: Point2,
    _velocity: Point2,
    _acceleration: Point2,
    _life_span: u32,
    _sparkles: Vec<Particle>,
    _is_firework: bool,
}

const SPARKLE_TRIGGER: u32 = 80;

impl Particle {

    fn new(_position: Point2, _velocity: Point2, _is_firework: bool) -> Particle {
        Particle {
            _position: _position,
            _velocity: _velocity,
            _acceleration: pt2(0.0, 0.0),
            _life_span: 0,
            _sparkles: Vec::new(),
            _is_firework: _is_firework,
        }
    }

    fn create_sparkles(&mut self) {
        let mut _sparkles: Vec<Particle> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..30 {
            let _rnd_x = map_range(rng.gen(), 0.0, 1.0, -5.0, 5.0);
            let _rnd_y = map_range(rng.gen(), 0.0, 1.0, -2.0, 10.0);
            let _velocity = pt2(_rnd_x, _rnd_y);
            _sparkles.push(Particle::new(self._position, _velocity, false));
        }
        self._sparkles = _sparkles;
    }

    fn apply_force(&mut self, _force: Point2) {
        self._acceleration = self._acceleration + _force;
        if self._life_span > SPARKLE_TRIGGER {
           for _sparkle in self._sparkles.iter_mut() {
                _sparkle.apply_force(_force);
            }
        }
    }

    fn update(&mut self) {
        self._position = self._position + self._velocity;
        self._velocity = self._velocity + self._acceleration;
        self._acceleration = pt2(0.0, 0.0);
        self._life_span += 1;

        if self._is_firework && self._life_span == SPARKLE_TRIGGER {
            self.create_sparkles();
        }

        if self._life_span > SPARKLE_TRIGGER {
            for _sparkle in self._sparkles.iter_mut() {
                _sparkle.update();
            }
        }
    }

    fn draw(&self, _draw: &Draw) {

        if self._life_span < SPARKLE_TRIGGER {
            _draw.ellipse()
                .xy(self._position)
                .radius(2.0)
                .color(WHITE);
        }

        if self._life_span > SPARKLE_TRIGGER {
            let _sparkles = &self._sparkles;
            for _sparkle in _sparkles {
                _sparkle.draw(_draw);
            }
        }
    }

}
