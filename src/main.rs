pub use fltk::prelude::*;
use fltk::{app::*, draw::*};

mod data;
mod widgets;
mod windows;
use windows::figure_window::*;

fn linspace(arr: &mut Vec<f64>, l: f64, r: f64, len: usize) {
    let dx = (r - l) / (len - 1) as f64;

    arr.reserve(len);

    for i in 0..len {
        arr.push(l + (i as f64 * dx));
    }
}

fn arrayfun(input: &[f64], output: &mut Vec<f64>, fun: &dyn Fn(f64) -> f64) {
    output.reserve(input.len());

    for i in input.iter() {
        output.push(fun(*i));
    }
}

fn main() {
    let app = App::default();

    const N: usize = 1000;
    let mut t: Vec<f64> = Vec::with_capacity(N);
    let mut x: Vec<f64> = Vec::with_capacity(N);
    let mut y: Vec<f64> = Vec::with_capacity(N);

    linspace(&mut t, 0.0, 2.0 * std::f64::consts::PI, N);
    arrayfun(&t, &mut x, &f64::sin);
    arrayfun(&t, &mut y, &|x: f64| 0.5 - 0.5 * f64::exp(-x));

    let mut fig = FigureWindow::new(640, 480, "Figure 1", 1, 2);
    fig.plot(&t, &x, LineStyle::Solid, 2, Color::Red, 0);
    fig.set_hold(true, 0);
    fig.set_grid(true, 0);
    fig.plot(&t, &y, LineStyle::Dash, 2, Color::Blue, 0);
    fig.set_hold(false, 0);
    fig.plot(&x, &y, LineStyle::Solid, 2, Color::Red, 1);

    fig.end();
    fig.show();

    app.run().unwrap();
}
