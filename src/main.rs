use nannou::prelude::*;

const ROWS: u32 = 32;
const COLS: u32 = 20;
const SIZE: u32 = 26;
const HEIGHT: u32 = ROWS * SIZE;
const WIDTH: u32 = COLS * SIZE;
const LINE_WIDTH: f32 = 0.03;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.scale(SIZE as f32).x_y(ROWS as f32, COLS as f32);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
