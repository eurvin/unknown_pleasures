use nannou::noise::*;
use nannou::prelude::*;

const ROWS: u32 = 50;
const SIZE: u32 = 15;
const MARGIN: u32 = 50;
const HEIGHT: u32 = 800;
const WIDTH: u32 = 600;
const LINE_WIDTH: f32 = 0.04;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::loop_once())
        .run();
}

#[derive(Debug)]
struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    fn new(y: f32) -> Self {
        let start = pt2(-1.0, y);
        let end = pt2(1.0, y);
        Line { start, end }
    }
}

#[derive(Debug)]
struct Model {
    lines: Vec<Line>,
    random_seed: u32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let mut lines = Vec::new();

    for y in 0..ROWS {
        let line = Line::new(y as f32);
        println!("{:?}", line);
        lines.push(line);
    }

    Model {
        lines,
        random_seed: 43,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let noise = Perlin::new();

    let gdraw = draw
        .scale_y(SIZE as f32)
        .scale_x(WIDTH as f32 / 2.0 - MARGIN as f32 * 2.0)
        .scale_y(-1.0)
        .y(ROWS as f32 / -2.0 + 0.5);

    draw.background().color(BLACK);

    for line in &model.lines {
        let ldraw = &gdraw;
        ldraw
            .line()
            .start(line.start)
            .end(line.end)
            .weight(LINE_WIDTH)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
