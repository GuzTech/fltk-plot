# fltk-plot

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build](https://github.com/GuzTech/fltk-plot/workflows/Build/badge.svg?branch=main)](https://github.com/GuzTech/fltk-plot/actions)

An interactive plotting library for Rust, similar to Matlab plotting.

The fltk-plot crate uses the [fltk-rs](https://github.com/fltk-rs/fltk-rs) crate and adds simple, interactive 2D plotting widgets.

# Usage

Add the following to your project's Cargo.toml file:

```toml
[dependencies]
fltk-plot = { git = "https://github.com/GuzTech/fltk-plot", branch = "main" }
fltk = { version = "^1.3", features = ["fltk-bundled"] }
```

An example [hello world](https://github.com/GuzTech/fltk-plot/blob/main/examples/hello_world.rs) application:

```rust

use fltk_plot::windows::figure_window::FigureWindow;
use fltk::{app::*, draw::*, enums::*, prelude::*};

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

    const N: usize = 100;
    let mut t: Vec<f64> = Vec::with_capacity(N);
    let mut x: Vec<f64> = Vec::with_capacity(N);
    let mut y: Vec<f64> = Vec::with_capacity(N);

    linspace(&mut t, 0.0, 2.0 * std::f64::consts::PI, N);
    arrayfun(&t, &mut x, &f64::sin);
    //arrayfun(&t, &mut x, &|x| 10.0 * f64::sin(x));
    arrayfun(&t, &mut y, &|x: f64| 0.5 - 0.5 * f64::exp(-x));

    let mut fig = FigureWindow::new(640, 480, "Figure 1", 1, 2);
    fig.plot(&t, &x, LineStyle::Solid, 2, Color::Red, 0);
    fig.set_hold(true, 0);
    fig.plot(&t, &y, LineStyle::Dash, 2, Color::Blue, 0);
    fig.set_hold(false, 0);
    fig.plot(&x, &y, LineStyle::Solid, 2, Color::Red, 1);

    fig.set_x_label("Time [s]", 0);
    fig.set_y_label("Voltage [V]", 0);
    fig.set_caption("Votage vs. time", 0);
    fig.set_grid(true, 0);

    fig.set_x_label("Voltage [V]", 1);
    fig.set_y_label("Voltage [V]", 1);
    fig.set_caption("Signal 1 vs Signal 2", 1);
    fig.set_grid(true, 1);

    fig.end();
    fig.show();

    let mut fig2 = FigureWindow::new(640, 480, "Figure 2", 1, 1);
    fig2.stem(&t, &x, LineStyle::Solid, 2, Color::Green, 0);
    fig2.set_grid(true, 0);
    fig2.set_x_label("X label", 0);
    fig2.set_y_label("Y label", 0);
    fig2.set_caption("Caption", 0);

    fig2.end();
    fig2.show();

    app.run().unwrap();
}
```

This will display two figure windows which can be interacted with separately:

- ![](https://github.com/GuzTech/fltk-plot/raw/main/screenshots/hello_world.png)

The mouse interactions are as follows:

- Panning: click and hold middle mouse button
- Resetting back to the initial view: click right mouse button
- Adding a data point: click left mouse button on the plot
- Moving a data point: click left mouse button and move to desired position
- Moving data point label: click middle mouse button on the data point marker and drag to desired position
- Remove a data point: click right mouse button on the data point marker
- Zoom in/out: mouse wheel
- Zoom area: click left mouse button and drag to select the desired area
