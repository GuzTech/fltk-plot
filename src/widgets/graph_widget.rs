use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use fltk::{app::*, draw::*, widget::*};

use crate::data::plot_2d_data::*;
use crate::widgets::widget::*;

#[derive(Clone, Debug)]
pub struct GraphWidget {
    pub widget: Widget,
    pub x_label: String,
    pub y_label: String,
    pub caption: String,
    pub limit: Limit,
    pub limit_c: Limit,
    pub curr_data_tip: Rc<RefCell<usize>>,
    pub zooming: Rc<RefCell<bool>>,
    pub grid: Rc<RefCell<bool>>,
    pub hold: Rc<RefCell<bool>>,
    pub zoom_x: Rc<RefCell<i32>>,
    pub zoom_y: Rc<RefCell<i32>>,
    pub xn_grid: Rc<RefCell<i32>>,
    pub yn_grid: Rc<RefCell<i32>>,
}

#[allow(dead_code)]
impl GraphWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32, caption: &str) -> GraphWidget {
        let l = Limit {
            x_left: 0.0,
            x_right: width as f64,
            y_left: 0.0,
            y_right: height as f64,
        };
        GraphWidget {
            widget: Widget::new(x, y, width, height, caption),
            x_label: String::default(),
            y_label: String::default(),
            caption: caption.into(),
            limit: l,
            limit_c: l,
            curr_data_tip: Rc::from(RefCell::from(0)),
            zooming: Rc::from(RefCell::from(false)),
            grid: Rc::from(RefCell::from(false)),
            hold: Rc::from(RefCell::from(false)),
            zoom_x: Rc::from(RefCell::from(0)),
            zoom_y: Rc::from(RefCell::from(0)),
            xn_grid: Rc::from(RefCell::from(5)),
            yn_grid: Rc::from(RefCell::from(5)),
        }
    }

    pub fn get_closest_point(
        &self,
        data: &[&Plot2DData],
        tip: &mut DataTip,
        mx: i32,
        my: i32,
        width: f64,
        height: f64,
    ) -> bool {
        let w = self.widget.width() as f64;
        let h = self.widget.height() as f64;
        let mut m_dist = w * w + h * h;

        for (j, &d) in data.iter().enumerate() {
            for i in 0..d.length {
                if let Some((mut px, mut py)) = d.get_value(i) {
                    // Scale and shift
                    px = ((px - self.limit_c.x_left) / width) * w + self.widget.x() as f64;
                    py = h - ((py - self.limit_c.y_left) / height) * h + self.widget.y() as f64;

                    let dist = f64::max(f64::abs(px - mx as f64), f64::abs(py - my as f64));
                    if dist < m_dist {
                        if let Some((px, py)) = d.get_value(i) {
                            m_dist = dist;
                            tip.x = px;
                            tip.y = py;
                            tip.plot = j;
                        }
                    }
                };
            }
        }

        m_dist <= 10.0
    }
}

impl MyWidget for GraphWidget {
    fn put_data(&mut self, _x: &[f64], _y: &[f64], _style: LineStyle, _width: i32, _color: Color) {}

    fn handle(&mut self) {}

    fn set_grid(&mut self, on: bool) {
        *self.grid.borrow_mut() = on;
        redraw();
    }

    fn set_x_label(&mut self, label: &str) {
        self.x_label = label.to_string();
        redraw();
    }

    fn set_y_label(&mut self, label: &str) {
        self.y_label = label.to_string();
        redraw();
    }

    fn set_caption(&mut self, caption: &str) {
        self.caption = caption.to_string();
        redraw();
    }

    fn set_hold(&mut self, hold: bool) {
        *self.hold.borrow_mut() = hold;
    }
}

impl Deref for GraphWidget {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl DerefMut for GraphWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}
