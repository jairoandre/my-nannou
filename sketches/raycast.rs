use nannou::prelude::*;
use nannou::draw::Draw;
use nannou::geom::*;
use rand::Rng;


// CONSTANTS

const RAY_SIZE: f32 = 3000.0;
const WALL_COUNT: i32 = 10;
const WALL_THICKNESS: f32 = 2.0;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    rays: Vec<Line>,
    walls: Vec<Line>,
}

fn create_rnd_line() -> Line {
    Line {
        start: rnd_point(),
        end: rnd_point(),
        half_thickness: 1.0
    }
}

fn rnd_coord() -> f32 {
    rand::thread_rng().gen_range(-500.0, 500.0)
}

fn rnd_point() -> Point2 {
    pt2(rnd_coord(), rnd_coord())
}

fn model(app: &App) -> Model {

    let _window = app
        .new_window()
        .with_dimensions(800, 800)
        .view(view)
        .build()
        .unwrap();
    
    let mut walls: Vec<Line> = Vec::new();

    for _ in 0..WALL_COUNT {
        walls.push(create_rnd_line());
    }

    Model {
        rays: create_rays(&app),
        walls: walls,
    }
}

fn create_rays(app: &App) -> Vec<Line> {
    let mut rays: Vec<Line> = Vec::new();

    for i in 0..360 {

        if i % 3 != 0 { continue; }

        let angle: f32 = i as f32;

        let rad = angle.to_radians();

        let this_x = app.mouse.x + RAY_SIZE * rad.cos();
        let this_y = app.mouse.y + RAY_SIZE * rad.sin();

        rays.push(Line::new(pt2(app.mouse.x, app.mouse.y), pt2(this_x, this_y), 1.0));
    }

    rays

}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.rays = create_rays(&app);
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {

    let draw = app.draw();

    draw.background().color(BLACK);

    draw_walls(&draw, &model);

    cast_rays(&draw, &model);

    draw.to_frame(app, &frame).unwrap();

    frame
}

fn draw_walls(draw: &Draw, model: &Model) {
    for wall in &model.walls {
        draw.line()
            .start(wall.start)
            .end(wall.end)
            .thickness(WALL_THICKNESS)
            .color(WHITE);
    }
}

fn cast_rays(draw: &Draw, model: &Model) {
    for _line in &model.rays {

        let mut _end = _line.end;

        let _intersect = find_closest_intersection(&_line, &model);

        if _intersect != None {
            _end = _intersect.unwrap();
        }

        draw.line()
            .start(_line.start)
            .end(_end)
            .thickness(_line.half_thickness)
            .color(WHITE);
    }

}

fn find_closest_intersection(_line: &Line, model: &Model) -> Option<Point2> {

    let mut _intersect: Option<Point2> = None;

    let mut _record: Option<f32> = None;

    for _wall in &model.walls {

        let _hit_point = line::join::intersect((_wall.start, _wall.end), (_line.start, _line.end)); 

        if _hit_point == None { 
            continue;
        }

        let _unwraped_hit_point = _hit_point.unwrap();

        if _wall.contains(&_unwraped_hit_point) == None {
            continue;
        }

        if _line.contains(&_unwraped_hit_point) == None {
            continue;
        }

        if _intersect == None {
            _intersect = _hit_point;
            _record = Some(points_distance(&_line.start, &_unwraped_hit_point));
            continue;
        }

        let possible_new_record = points_distance(&_line.start, &_unwraped_hit_point);

        if possible_new_record < _record.unwrap() {
            _record = Some(possible_new_record);
            _intersect = _hit_point;
        }

    }

    _intersect

}

fn points_distance(p1: &Vector2, p2: &Vector2) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}