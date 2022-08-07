use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use fltk::{prelude::*, enums::Color, app::*, draw::*, widget::*};

use crate::widgets::widget::*;

#[derive(Clone, Debug)]
pub struct GraphWidget {
    pub widget: Widget,
    pub x_label: Rc<RefCell<String>>,
    pub y_label: Rc<RefCell<String>>,
    pub caption: Rc<RefCell<String>>,
    pub limit: Rc<RefCell<Limit>>,
    pub limit_c: Rc<RefCell<Limit>>,
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
            widget: Widget::new(x, y, width, height, None).with_label(caption),
            x_label: Rc::from(RefCell::from(String::default())),
            y_label: Rc::from(RefCell::from(String::default())),
            caption: Rc::from(RefCell::from(caption.to_string())),
            limit: Rc::from(RefCell::from(l)),
            limit_c: Rc::from(RefCell::from(l)),
            zooming: Rc::from(RefCell::from(false)),
            grid: Rc::from(RefCell::from(false)),
            hold: Rc::from(RefCell::from(false)),
            zoom_x: Rc::from(RefCell::from(0)),
            zoom_y: Rc::from(RefCell::from(0)),
            xn_grid: Rc::from(RefCell::from(5)),
            yn_grid: Rc::from(RefCell::from(5)),
        }
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
        *self.x_label.borrow_mut() = label.to_string();
        redraw();
    }

    fn set_y_label(&mut self, label: &str) {
        *self.y_label.borrow_mut() = label.to_string();
        redraw();
    }

    fn set_caption(&mut self, caption: &str) {
        *self.caption.borrow_mut() = caption.to_string();
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
