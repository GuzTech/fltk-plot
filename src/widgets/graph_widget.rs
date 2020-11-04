use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use fltk::{app::*, draw::*, widget::*};

use crate::data::plot_2d_data::*;
use crate::widgets::widget::*;

#[derive(Clone, Debug)]
pub struct GraphWidget {
    pub widget: Widget,
    pub x_label: Rc<RefCell<String>>,
    pub y_label: Rc<RefCell<String>>,
    pub caption: Rc<RefCell<String>>,
    pub data_tips: Rc<RefCell<Vec<DataTip>>>,
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
            widget: Widget::new(x, y, width, height, caption),
            x_label: Rc::from(RefCell::from(String::default())),
            y_label: Rc::from(RefCell::from(String::default())),
            caption: Rc::from(RefCell::from(caption.to_string())),
            data_tips: Rc::from(RefCell::from(Vec::new())),
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

    pub fn get_closest_point(
        &self,
        data: &[Option<Plot2DData>],
        mx: i32,
        my: i32,
        width: f64,
        height: f64,
    ) -> Option<DataTip> {
        let widget_width = self.widget.width() as f64;
        let widget_height = self.widget.height() as f64;
        let mut m_dist = widget_width * widget_width + widget_height * widget_height;

        let mut tip = DataTip::default();

        for (j, d) in data.iter().enumerate() {
            if d.is_some() {
                let d = d.as_ref().unwrap();

                for i in 0..d.length {
                    if let Some((mut px, mut py)) = d.get_value(i) {
                        // Scale and shift
                        px = ((px - self.limit_c.borrow().x_left) / width) * widget_width
                            + self.widget.x() as f64;
                        py = widget_height
                            - ((py - self.limit_c.borrow().y_left) / height) * widget_height
                            + self.widget.y() as f64;

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
        }

        if m_dist <= 10.0 {
            Some(tip)
        } else {
            None
        }
    }

    pub fn get_closest_datatip(
        &self,
        mx: i32,
        my: i32,
        width: f64,
        height: f64,
    ) -> Option<DataTip> {
        let closest = None;

        let height = if height != 0.0 { height } else { 1.0 };

        for closest in &*self.data_tips.borrow_mut() {
            let px = ((closest.x - self.limit_c.borrow().x_left) / width)
                * self.widget.width() as f64
                + self.widget.x() as f64;
            let py = self.widget.height() as f64
                - ((closest.y - self.limit_c.borrow().y_left) / height)
                    * self.widget.height() as f64
                + self.widget.y() as f64;
            let distance = i32::max(i32::abs(px as i32 - mx), i32::abs(py as i32 - my));

            if distance < 15 {
                break;
            }
        }

        closest
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
