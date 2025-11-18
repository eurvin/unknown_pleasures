use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::loop_once())
        .run()
}

struct Model {}

fn model(_app: &App) {}

fn view(app: &App, _model: &Model, _frame: Frame) {}

fn update(_app: &App, model: &Model, _update: Update) {}
