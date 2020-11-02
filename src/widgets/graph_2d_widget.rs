use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use fltk::draw::*;

use crate::data::plot_2d_data::*;
use crate::widgets::{graph_widget::*, widget::*};

#[derive(Clone, Debug)]
pub struct Graph2DWidget {
    pub widget: GraphWidget,
    pub data: Rc<RefCell<Vec<Option<Plot2DData>>>>,
}

#[allow(dead_code)]
impl Graph2DWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32, caption: &str) -> Graph2DWidget {
        let mut x = Graph2DWidget {
            widget: GraphWidget::new(x, y, width, height, caption),
            data: Rc::from(RefCell::from(Vec::new())),
        };

        x.handle();
        x
    }
}

impl MyWidget for Graph2DWidget {
    fn put_data(&mut self, x: &[f64], y: &[f64], style: LineStyle, width: i32, color: Color) {
        let data = Plot2DData::new_xy(x, y, style, width, color);

        if !*self.hold.borrow() {
            self.data.borrow_mut().clear();
        }

        let mut x_min = 0.0;
        let mut x_max = 0.0;
        let mut y_min = 0.0;
        let mut y_max = 0.0;

        data.get_x_limit(&mut x_min, &mut x_max);
        data.get_y_limit(&mut y_min, &mut y_max);

        if self.data.borrow().is_empty() {
            self.limit = Limit {
                x_left: x_min,
                x_right: x_max,
                y_left: y_min,
                y_right: y_max,
            };
        //self.limit_c = self.limit;
        } else {
            self.limit = Limit {
                x_left: f64::min(self.limit.x_left, x_min),
                x_right: f64::max(self.limit.x_right, x_max),
                y_left: f64::min(self.limit.y_left, y_min),
                y_right: f64::max(self.limit.y_right, y_max),
            };
            //self.limit_c = self.limit;
        }

        self.data.borrow_mut().push(Some(data));
    }

    fn handle(&mut self) {}

    fn set_grid(&mut self, on: bool) {
        self.widget.set_grid(on);
    }

    fn set_x_label(&mut self, label: &str) {
        self.widget.set_x_label(label);
    }

    fn set_y_label(&mut self, label: &str) {
        self.widget.set_y_label(label);
    }

    fn set_caption(&mut self, caption: &str) {
        self.widget.set_caption(caption);
    }

    fn set_hold(&mut self, hold: bool) {
        self.widget.set_hold(hold);
    }
}

impl Deref for Graph2DWidget {
    type Target = GraphWidget;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl DerefMut for Graph2DWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}
