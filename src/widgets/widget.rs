use fltk::draw::*;

pub trait MyWidget {
    fn put_data(&mut self, x: &[f64], y: &[f64], style: LineStyle, width: i32, color: Color);
    fn handle(&mut self);
    fn set_grid(&mut self, on: bool);
    fn set_x_label(&mut self, label: &str);
    fn set_y_label(&mut self, label: &str);
    fn set_caption(&mut self, caption: &str);
    fn set_hold(&mut self, hold: bool);
}

impl std::fmt::Debug for dyn MyWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyWidget")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DataTip {
    pub x: f64,
    pub y: f64,
    pub plot_idx: usize,
    pub lx: i32,
    pub ly: i32,
}

impl Default for DataTip {
    fn default() -> Self {
        DataTip {
            x: 0.0,
            y: 0.0,
            plot_idx: usize::MAX,
            lx: 0,
            ly: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Limit {
    pub x_left: f64,
    pub x_right: f64,
    pub y_left: f64,
    pub y_right: f64,
}
