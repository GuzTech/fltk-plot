use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use fltk::app::MouseWheel;
use fltk::{prelude::*, enums::*, draw::*};

use crate::data::plot_2d_data::*;
use crate::widgets::{graph_widget::*, widget::*};

#[derive(Clone)]
pub struct Graph2DWidget {
    pub widget: GraphWidget,
    pub data: Rc<RefCell<Vec<Option<Plot2DData>>>>,
    pub data_tips: Rc<RefCell<Vec<DataTip>>>,
    pub closest_data_tip: Rc<RefCell<Option<usize>>>,
}

#[allow(dead_code)]
impl Graph2DWidget {
    const LEFT_BUTTON: i32 = 1;
    const RIGHT_BUTTON: i32 = 3;
    const MIDDLE_BUTTON: i32 = 2;

    pub fn new(x: i32, y: i32, width: i32, height: i32, caption: &str) -> Graph2DWidget {
        let mut x = Graph2DWidget {
            widget: GraphWidget::new(x, y, width, height, caption),
            data: Rc::from(RefCell::from(Vec::new())),
            data_tips: Rc::from(RefCell::from(Vec::new())),
            closest_data_tip: Rc::from(RefCell::from(None)),
        };

        x.handle();
        x
    }

    pub fn get_closest_point(
        data: &[Option<Plot2DData>],
        mx: i32,
        my: i32,
        width: f64,
        height: f64,
        wid_width: f64,
        wid_height: f64,
        wid_x: f64,
        wid_y: f64,
        limit_c: &Limit,
    ) -> Option<DataTip> {
        let mut m_dist = wid_width * wid_width + wid_height * wid_height;

        let mut tip = DataTip::default();

        for (j, d) in data.iter().enumerate() {
            if d.is_some() {
                let d = d.as_ref().unwrap();

                for i in 0..d.length {
                    if let Some((mut px, mut py)) = d.get_value(i) {
                        // Scale and shift
                        px = ((px - limit_c.x_left) / width) * wid_width + wid_x;
                        py = wid_height - ((py - limit_c.y_left) / height) * wid_height + wid_y;

                        let dist = f64::max(f64::abs(px - mx as f64), f64::abs(py - my as f64));
                        if dist < m_dist {
                            if let Some((px, py)) = d.get_value(i) {
                                m_dist = dist;
                                tip.x = px;
                                tip.y = py;
                                tip.plot_idx = j;
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
        data_tips: Rc<RefCell<Vec<DataTip>>>,
        mx: i32,
        my: i32,
        width: f64,
        height: f64,
        wid_width: f64,
        wid_height: f64,
        wid_x: f64,
        wid_y: f64,
        limit_c: &Limit,
    ) -> Option<usize> {
        let mut closest: Option<usize> = None;

        let height = if height != 0.0 { height } else { 1.0 };

        for (i, c) in data_tips.borrow().iter().enumerate() {
            let px = ((c.x - limit_c.x_left) / width) * wid_width + wid_x;
            let py = wid_height - ((c.y - limit_c.y_left) / height) * wid_height + wid_y;
            let distance = i32::max(i32::abs(px as i32 - mx), i32::abs(py as i32 - my));

            if distance < 15 {
                closest = Some(i);
                break;
            }
        }

        closest
    }
}

impl MyWidget for Graph2DWidget {
    fn put_data(&mut self, x: &[f64], y: &[f64], style: LineStyle, width: i32, color: Color) {
        let data = Plot2DData::new_xy(x, y, style, width, color);

        if !*self.hold.borrow() {
            self.data.borrow_mut().clear();
        }

        let (x_min, x_max) = data.get_x_limit();
        let (y_min, y_max) = data.get_y_limit();

        if self.data.borrow().is_empty() {
            *self.limit.borrow_mut() = Limit {
                x_left: x_min,
                x_right: x_max,
                y_left: y_min,
                y_right: y_max,
            };
            *self.limit_c.borrow_mut() = Limit {
                x_left: x_min,
                x_right: x_max,
                y_left: y_min,
                y_right: y_max,
            };
        } else {
            let tmp_x_min = f64::min(self.limit.borrow().x_left, x_min);
            let tmp_x_max = f64::max(self.limit.borrow().x_right, x_max);
            let tmp_y_min = f64::min(self.limit.borrow().y_left, y_min);
            let tmp_y_max = f64::max(self.limit.borrow().y_right, y_max);

            *self.limit.borrow_mut() = Limit {
                x_left: tmp_x_min,
                x_right: tmp_x_max,
                y_left: tmp_y_min,
                y_right: tmp_y_max,
            };
            *self.limit_c.borrow_mut() = Limit {
                x_left: tmp_x_min,
                x_right: tmp_x_max,
                y_left: tmp_y_min,
                y_right: tmp_y_max,
            };
        }

        self.data.borrow_mut().push(Some(data));
    }

    fn handle(&mut self) {
        let wid = self.widget.clone();
        let data = self.data.clone();
        let closest_data_tip = self.closest_data_tip.clone();
        let data_tips = self.data_tips.clone();

        self.widget.widget.handle(move |_, event| {
            let (mx, my) = fltk::app::event_coords();
            let widget_width = wid.width();
            let widget_height = wid.height();
            let widget_x = wid.x();
            let widget_y = wid.y();
            let limit_width = wid.limit_c.borrow().x_right - wid.limit_c.borrow().x_left;
            let limit_height = wid.limit_c.borrow().y_right - wid.limit_c.borrow().y_left;

            match event {
                Event::Push => {
                    let button = fltk::app::event_button();
                    *closest_data_tip.borrow_mut() = Graph2DWidget::get_closest_datatip(
                        data_tips.clone(),
                        mx,
                        my,
                        limit_width,
                        limit_height,
                        widget_width as f64,
                        widget_height as f64,
                        widget_x as f64,
                        widget_y as f64,
                        &wid.limit_c.borrow(),
                    );

                    match button {
                        Graph2DWidget::LEFT_BUTTON => {
                            // First check if user clicked on a data tip.

                            // Find a data tip point close to the mouse pointer.
                            if closest_data_tip.borrow().is_none() {
                                // User didn't click on an existing data tip.
                                let temp_datatip = Graph2DWidget::get_closest_point(
                                    &*data.borrow(),
                                    mx,
                                    my,
                                    limit_width,
                                    limit_height,
                                    widget_width as f64,
                                    widget_height as f64,
                                    widget_x as f64,
                                    widget_y as f64,
                                    &wid.limit_c.borrow(),
                                );

                                if let Some(mut temp_datatip) = temp_datatip {
                                    temp_datatip.lx = 10;
                                    temp_datatip.ly = -10;
                                    (*data_tips.borrow_mut()).push(temp_datatip);
                                } else {
                                    *wid.zooming.borrow_mut() = true;
                                    *wid.zoom_x.borrow_mut() = mx;
                                    *wid.zoom_y.borrow_mut() = my;
                                }

                                fltk::app::redraw();
                            }
                        }
                        Graph2DWidget::MIDDLE_BUTTON => {
                            *wid.zoom_x.borrow_mut() = mx;
                            *wid.zoom_y.borrow_mut() = my;
                        }
                        Graph2DWidget::RIGHT_BUTTON => {
                            if closest_data_tip.borrow().is_some() {
                                data_tips
                                    .borrow_mut()
                                    .remove(closest_data_tip.borrow().unwrap());
                            } else {
                                *wid.limit_c.borrow_mut() = *wid.limit.borrow();
                            }

                            fltk::app::redraw();
                        }
                        _ => {}
                    }

                    true
                }
                Event::Drag => {
                    let button = fltk::app::event_button();

                    match button {
                        Graph2DWidget::LEFT_BUTTON => {
                            if !*wid.zooming.borrow() && closest_data_tip.borrow().is_some() {
                                let mut mdist =
                                    widget_width * widget_width + widget_height * widget_height;

                                let mut data_tip = DataTip::default();

                                if let Some(tip) =
                                    data_tips.borrow().get(closest_data_tip.borrow().unwrap())
                                {
                                    data_tip = *tip;
                                };

                                if data_tip.plot_idx < usize::MAX {
                                    if let Some(dat) = data.borrow().get(data_tip.plot_idx) {
                                        if let Some(d) = dat {
                                            for i in 0..d.length {
                                                if let Some((mut px, mut py)) = d.get_value(i) {
                                                    px = ((px - wid.limit_c.borrow().x_left)
                                                        / limit_width)
                                                        * widget_width as f64
                                                        + widget_x as f64;
                                                    py = widget_height as f64
                                                        - ((py - wid.limit_c.borrow().y_left)
                                                            / limit_height)
                                                            * widget_height as f64
                                                        + widget_y as f64;

                                                    let dist = i32::max(
                                                        i32::abs(px as i32 - mx),
                                                        i32::abs(py as i32 - my),
                                                    );
                                                    if dist < mdist {
                                                        if let Some((px, py)) = d.get_value(i) {
                                                            mdist = dist;

                                                            if let Some(tip_idx) =
                                                                *closest_data_tip.borrow()
                                                            {
                                                                if let Some(tip) = data_tips
                                                                    .borrow_mut()
                                                                    .get_mut(tip_idx)
                                                                {
                                                                    tip.x = px;
                                                                    tip.y = py;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Graph2DWidget::MIDDLE_BUTTON => {
                            if closest_data_tip.borrow().is_some() {
                                if let Some(tip_idx) = *closest_data_tip.borrow() {
                                    if let Some(tip) = data_tips.borrow_mut().get_mut(tip_idx) {
                                        let mut px = tip.x;
                                        let mut py = tip.y;
                                        px = ((px - wid.limit_c.borrow().x_left) / limit_width)
                                            * widget_width as f64
                                            + widget_x as f64;
                                        py = widget_height as f64
                                            - ((py - wid.limit_c.borrow().y_left) / limit_height)
                                                * widget_height as f64
                                            + widget_y as f64;

                                        tip.lx = mx - px as i32;
                                        tip.ly = my - py as i32;
                                    }
                                }
                            } else {
                                let mut dx = (mx - *wid.zoom_x.borrow()) as f64;
                                let mut dy = (my - *wid.zoom_y.borrow()) as f64;

                                if f64::abs(dx) >= 1.0 || f64::abs(dy) >= 1.0 {
                                    dx *= limit_width / widget_width as f64;
                                    dy *= limit_height / widget_height as f64;

                                    *wid.zoom_x.borrow_mut() = mx;
                                    *wid.zoom_y.borrow_mut() = my;

                                    wid.limit_c.borrow_mut().x_left -= dx;
                                    wid.limit_c.borrow_mut().x_right -= dx;
                                    wid.limit_c.borrow_mut().y_left += dy;
                                    wid.limit_c.borrow_mut().y_right += dy;
                                }
                            }
                        }
                        _ => {}
                    }

                    fltk::app::redraw();

                    true
                }
                Event::Released => {
                    let button = fltk::app::event_button();

                    match button {
                        Graph2DWidget::LEFT_BUTTON if *wid.zooming.borrow() => {
                            let zoom_x = *wid.zoom_x.borrow();
                            let zoom_y = *wid.zoom_y.borrow();

                            let mut dx = i32::abs(mx - zoom_x);
                            let mut dy = i32::abs(my - zoom_y);
                            *wid.zooming.borrow_mut() = false;
                            *wid.zoom_x.borrow_mut() = i32::min(zoom_x, mx);
                            *wid.zoom_y.borrow_mut() = i32::min(zoom_y, my);

                            if dx > 2 || dy > 2 {
                                // Clip
                                let zoom_x = *wid.zoom_x.borrow();
                                let zoom_y = *wid.zoom_y.borrow();
                                *wid.zoom_x.borrow_mut() = i32::max(zoom_x, widget_x);
                                *wid.zoom_y.borrow_mut() = i32::max(zoom_y, widget_y);

                                let zoom_x = *wid.zoom_x.borrow();
                                let zoom_y = *wid.zoom_y.borrow();
                                dx = i32::min(dx, widget_width + widget_x - zoom_x);
                                dy = i32::min(dy, widget_height + widget_y - zoom_y);

                                // Scale + shift
                                let x_left = wid.limit_c.borrow().x_left;
                                let y_left = wid.limit_c.borrow().y_left;

                                wid.limit_c.borrow_mut().x_left = ((zoom_x - widget_x) as f64
                                    / widget_width as f64)
                                    * limit_width
                                    + x_left;
                                let x_left = wid.limit_c.borrow().x_left;
                                wid.limit_c.borrow_mut().x_right =
                                    x_left + (dx as f64 / widget_width as f64) * limit_width;
                                wid.limit_c.borrow_mut().y_left =
                                    ((widget_y + widget_height - dy - zoom_y) as f64
                                        / widget_height as f64)
                                        * limit_height
                                        + y_left;
                                let y_left = wid.limit_c.borrow().y_left;
                                wid.limit_c.borrow_mut().y_right =
                                    y_left + (dy as f64 / widget_height as f64) * limit_height;
                            }

                            fltk::app::redraw();
                        }
                        _ => {}
                    }

                    true
                }
                Event::MouseWheel => {
                    let coefficient = match fltk::app::event_dy() {
                        MouseWheel::None => 0.0,
                        MouseWheel::Down => 0.1,
                        MouseWheel::Up => -0.1,
                        MouseWheel::Right => 0.0,
                        MouseWheel::Left => 0.0,
                    };

                    // mx, my are in window space.
                    // widget_x, widget_y are in plot widget coordinates.
                    // widget_width, widget_height are the absolute widget sizes in pixels.
                    let mxx = (mx as f64 - widget_x as f64) / widget_width as f64 * limit_width;
                    let myy = (widget_height as f64 + widget_y as f64 - my as f64)
                        / widget_height as f64
                        * limit_height;
                    let dxr = limit_width - mxx as f64;
                    let dyr = limit_height - myy as f64;

                    let tmp_limit = *wid.limit_c.borrow();

                    wid.limit_c.borrow_mut().x_left = (mxx as f64 * coefficient) + tmp_limit.x_left;
                    wid.limit_c.borrow_mut().x_right = tmp_limit.x_right - (dxr * coefficient);
                    wid.limit_c.borrow_mut().y_left = (myy as f64 * coefficient) + tmp_limit.y_left;
                    wid.limit_c.borrow_mut().y_right = tmp_limit.y_right - (dyr * coefficient);

                    fltk::app::redraw();

                    true
                }
                _ => false,
            }
        });
    }

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
