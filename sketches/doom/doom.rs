use nannou::prelude::*;
use rand::Rng;

struct Model {
    _pixels: Vec<Vec<usize>>,
    _current: f32,
}

const COLOR_PALET: [(u8, u8, u8); 37] = [
    (0, 0, 0),
    (31, 7, 7),
    (47, 15, 7),
    (71, 15, 7),
    (87, 23, 7),
    (103, 31, 7),
    (119, 31, 7),
    (143, 39, 7),
    (159, 47, 7),
    (175, 63, 7),
    (191, 71, 7),
    (199, 71, 7),
    (223, 79, 7),
    (223, 87, 7),
    (223, 87, 7),
    (215, 95, 7),
    (215, 95, 7),
    (215, 103, 15),
    (207, 111, 15),
    (207, 119, 15),
    (207, 127, 15),
    (207, 135, 23),
    (199, 135, 23),
    (199, 143, 23),
    (199, 151, 31),
    (191, 159, 31),
    (191, 159, 31),
    (191, 167, 39),
    (191, 167, 39),
    (191, 175, 47),
    (183, 175, 47),
    (183, 183, 47),
    (183, 183, 55),
    (207, 207, 111),
    (223, 223, 159),
    (239, 239, 199),
    (255, 255, 255),
];

fn main() {
    nannou::app(model).update(update).run();
}

fn initial_value(_x: usize, _y: usize) -> usize {
    let mut rng = rand::thread_rng();
    if _y == 0 {
        map_range(rng.gen(), 0.0, 1.0, 30.0, 36.0) as usize
    } else {
        0
    }
}

fn model(_app: &App) -> Model {
    let _window = _app
        .new_window()
        .with_dimensions(800, 800)
        .view(view)
        .build()
        .unwrap();

    let mut _pixels: Vec<Vec<usize>> = Vec::new();

    for _ in 0..DIM_Y {
        let mut _row: Vec<usize> = Vec::new();
        for _ in 0..DIM_X {
            _row.push(0);
        }
        _pixels.push(_row);
    }

    Model {
        _pixels: _pixels,
        _current: 0.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let mut rng = rand::thread_rng();
    _model._current += 0.5;

    let _d_x = (_app.mouse.x - OFFSET_X) as usize / QUAD_DIM as usize;
    let _d_y = (_app.mouse.y - OFFSET_Y) as usize / QUAD_DIM as usize;

    let _m_x = if _d_x < DIM_X { _d_x } else { DIM_X / 2 };
    let _m_y = if _d_y < DIM_Y { _d_y } else { DIM_Y / 2 };

    _model._pixels[_m_y][_m_x] = 36;

    if _m_x > 0 {
        _model._pixels[_m_y][_m_x - 1] = 36;
    }

    if _model._current < 1.0 {
        ()
    } else {
        _model._current = 0.0;
        for y in (0..DIM_Y).rev() {
            for x in 0..DIM_X {
                if y > 0 {
                    let _bellow = _model._pixels[y - 1][x] as i32;

                    let _fire_decay = map_range(rng.gen(), 0.0, 1.0, 1.0, DECAY_MAX) as i32;
                    let _delta = _bellow - _fire_decay;

                    let _new_fire_intensity = if _delta > 0 { _delta as usize } else { 0 };

                    let _new_x = if x > 0 && 0.5 < rng.gen() { x - 1 } else { x };

                    _model._pixels[y][_new_x] = _new_fire_intensity;
                }
            }
        }
    }

}

const DIM_X: usize = 80;
const DIM_Y: usize = 80;
const OFFSET_X: f32 = -400.0;
const OFFSET_Y: f32 = -400.0;
const QUAD_DIM: f32 = 10.0;
const CELLSPACING: f32 = 0.0;
const DECAY_MAX: f32 = 4.0;

fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let _draw = _app.draw();
    _draw.background().color(BLACK);

    for _y in 0..DIM_Y {
        for _x in 0..DIM_X {
            let _row = &_model._pixels[_y];
            let _rgb = COLOR_PALET[_row[_x]];
            let _xy = pt2(
                OFFSET_X + _x as f32 * (QUAD_DIM + CELLSPACING),
                OFFSET_Y + _y as f32 * (QUAD_DIM + CELLSPACING),
            );
            let _color = Rgb::new_u8(_rgb.0, _rgb.1, _rgb.2);
            _draw
                .quad()
                .xy(_xy)
                .width(QUAD_DIM)
                .height(QUAD_DIM)
                .color(_color);
        }
    }

    _draw.to_frame(&_app, &frame).unwrap();
    frame
}
