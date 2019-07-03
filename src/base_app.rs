use nannou::prelude::*;

struct Model {}

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
    
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

}


fn view(_app: &App, _model: &Model, frame: Frame) -> Frame {
    let _draw = _app.draw();
    _draw.background().color(BLACK);
    _draw.to_frame(&_app, &frame).unwrap();
    frame
}