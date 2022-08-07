use fltk::{enums::Color, draw::*};

#[derive(Clone)]
pub struct Plot2DData {
    pub style: LineStyle,
    pub width: i32,
    pub color: Color,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub length: usize,
}

#[allow(dead_code)]
impl Plot2DData {
    pub fn new(length: usize, style: LineStyle, width: i32, color: Color) -> Plot2DData {
        Plot2DData {
            style,
            width,
            color,
            x: Vec::with_capacity(length),
            y: Vec::with_capacity(length),
            x_min: 0.0,
            y_min: 0.0,
            x_max: 0.0,
            y_max: 0.0,
            length,
        }
    }

    pub fn new_xy(x: &[f64], y: &[f64], style: LineStyle, width: i32, color: Color) -> Plot2DData {
        let mut x_min = 0.0;
        let mut x_max = 0.0;
        let mut y_min = 0.0;
        let mut y_max = 0.0;

        if !x.is_empty() {
            x_min = x[0];
            x_max = x_min;
            y_min = y[0];
            y_max = y_min;

            for i in 0..x.len() {
                x_min = if x_min > x[i] { x[i] } else { x_min };
                x_max = if x_max < x[i] { x[i] } else { x_max };
                y_min = if y_min > y[i] { y[i] } else { y_min };
                y_max = if y_max < y[i] { y[i] } else { y_max };
            }
        }

        Plot2DData {
            style,
            width,
            color,
            x: x.to_vec(),
            y: y.to_vec(),
            x_min,
            y_min,
            x_max,
            y_max,
            length: x.len(),
        }
    }

    pub fn get_x_limit(&self) -> (f64, f64) {
        (self.x_min, self.x_max)
    }

    pub fn get_y_limit(&self) -> (f64, f64) {
        (self.y_min, self.y_max)
    }

    pub fn set_value(&mut self, index: usize, nx: f64, ny: f64) {
        if index < self.length {
            self.x[index] = nx;
            self.y[index] = ny;

            self.x_min = if self.x_min > nx { nx } else { self.x_min };
            self.x_max = if self.x_max < nx { nx } else { self.x_max };
            self.y_min = if self.y_min > ny { ny } else { self.y_min };
            self.y_max = if self.y_max < ny { ny } else { self.y_max };
        }
    }

    pub fn get_value(&self, index: usize) -> Option<(f64, f64)> {
        if index < self.length {
            Some((self.x[index], self.y[index]))
        } else {
            None
        }
    }
}
