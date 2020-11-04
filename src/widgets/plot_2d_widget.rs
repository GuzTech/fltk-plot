use std::ops::{Deref, DerefMut};

use fltk::draw::*;

use crate::widgets::graph_2d_widget::*;
use crate::widgets::widget::MyWidget;

#[derive(Clone, Debug)]
pub struct Plot2DWidget {
    pub widget: Graph2DWidget,
}

#[allow(dead_code)]
impl Plot2DWidget {
    pub fn new(x: i32, y: i32, width: i32, height: i32, caption: &str) -> Plot2DWidget {
        let mut x = Plot2DWidget {
            widget: Graph2DWidget::new(x, y, width, height, caption),
        };
        x.draw();
        x.handle();
        x
    }

    pub fn draw(&mut self) {
        let limit_c = self.limit_c.clone();
        // TODO: These two below are not cloned, maybe they should be. I need to check this!
        let xn = *self.xn_grid.borrow();
        let yn = *self.yn_grid.borrow();
        let grid = self.grid.clone();
        let data = self.data.clone();
        let data_tips = self.widget.data_tips.clone();
        let caption = self.widget.caption.clone();
        let x_label = self.widget.x_label.clone();
        let y_label = self.widget.y_label.clone();
        let zooming = self.widget.zooming.clone();
        let zoom_x = self.widget.zoom_x.clone();
        let zoom_y = self.widget.zoom_y.clone();

        self.widget.draw2(move |p| {
            let lim_width = limit_c.borrow().x_right - limit_c.borrow().x_left;
            let lim_height = limit_c.borrow().y_right - limit_c.borrow().y_left;

            let x_left = limit_c.borrow().x_left;
            let x_right = limit_c.borrow().x_right;
            let y_left = limit_c.borrow().y_left;
            let y_right = limit_c.borrow().y_right;

            let widget_x = p.x();
            let widget_y = p.y();
            let widget_width = p.width();
            let widget_height = p.height();

            // Background
            draw_rect_fill(
                widget_x,
                widget_y,
                widget_width,
                widget_height,
                Color::White,
            );
            set_line_style(LineStyle::Solid, 1);
            draw_rect_with_color(
                widget_x,
                widget_y,
                widget_width,
                widget_height,
                Color::Black,
            );

            // Captions and labels
            set_font(Font::Helvetica, 12);
            set_draw_color(Color::Black);
            let text_width = width(caption.borrow().as_str()) as i32;
            draw_text(
                caption.borrow().as_str(),
                widget_x + (widget_width as i32 / 2) - (text_width / 2),
                widget_y - 7,
            );
            let text_width = width(x_label.borrow().as_str()) as i32;
            draw_text(
                x_label.borrow().as_str(),
                widget_x + (widget_width as i32 / 2) - (text_width / 2),
                widget_y + widget_height as i32 + 12,
            );
            let text_width = width(y_label.borrow().as_str()) as i32;
            draw_text_angled(
                90,
                y_label.borrow().as_str(),
                widget_x - 5,
                widget_y + (widget_height as i32 / 2) + (text_width / 2),
            );

            push_clip(widget_x, widget_y, widget_width, widget_height);

            // Draw grid lines
            set_font(Font::Helvetica, 10);
            set_draw_color(Color::Black);

            let dx = widget_width / (xn + 1);
            for i in 1..=xn {
                if *grid.borrow() {
                    set_draw_color(Color::Light2);
                    set_line_style(LineStyle::Dash, 1);
                    draw_line(
                        widget_x + dx * i,
                        widget_y,
                        widget_x + dx * i,
                        widget_y + widget_height,
                    );
                }

                set_draw_color(Color::Black);
                set_line_style(LineStyle::Solid, 1);
                draw_line(
                    widget_x + dx * i,
                    widget_y,
                    widget_x + dx * i,
                    widget_y + 10,
                );
                draw_line(
                    widget_x + dx * i,
                    widget_y + widget_height as i32,
                    widget_x + dx * i,
                    widget_y + widget_height as i32 - 10,
                );
                draw_text(
                    format!(
                        "{:.2}",
                        i as f64 * (x_right - x_left) / (xn as f64 + 1.0) + x_left
                    )
                    .as_str(),
                    widget_x + dx * i + 2,
                    widget_y + widget_height as i32 - 2,
                );
            }

            let dy = widget_height / (yn + 1);
            for i in 1..=yn {
                if *grid.borrow() {
                    set_draw_color(Color::Light2);
                    set_line_style(LineStyle::Dash, 1);
                    draw_line(
                        widget_x,
                        widget_y + dy * i,
                        widget_x + widget_width as i32,
                        widget_y + dy * i,
                    );
                }

                set_draw_color(Color::Black);
                set_line_style(LineStyle::Solid, 1);
                draw_line(
                    widget_x,
                    widget_y + dy * i,
                    widget_x + 10,
                    widget_y + dy * i,
                );
                draw_line(
                    widget_x + widget_width as i32,
                    widget_y + dy * i,
                    widget_x + widget_width as i32 - 10,
                    widget_y + dy * i,
                );
                draw_text(
                    format!(
                        "{:.2}",
                        i as f64 * (y_right - y_left) / (yn as f64 + 1.0) + y_left
                    )
                    .as_str(),
                    widget_x + 2,
                    widget_height as i32 + widget_y - dy * i - 5,
                );
            }

            // Plot the data
            for plot_data in &*data.borrow() {
                if plot_data.is_some() {
                    let plot = plot_data.as_ref().unwrap();
                    set_draw_color(plot.color);
                    set_line_style(plot.style, plot.width);

                    begin_line();
                    for j in 0..plot.length {
                        if let Some((mut px, mut py)) = plot.get_value(j) {
                            // Scale and shift
                            px =
                                ((px - x_left) / lim_width) * widget_width as f64 + widget_x as f64;
                            py = widget_height as f64
                                - ((py - y_left) / lim_height) * widget_height as f64
                                + widget_y as f64;
                            vertex(px, py);
                        };
                    }
                    end_line();
                }
            }

            // Draw data tips
            set_line_style(LineStyle::Solid, 1);
            for tip in &*data_tips.borrow() {
                let mut px = tip.x;
                let mut py = tip.y;

                px = ((px - x_left) / lim_width) * widget_width as f64 + widget_x as f64;
                py = widget_height as f64 - ((py - y_left) / lim_height) * widget_height as f64
                    + widget_y as f64;
                let px = px as i32;
                let py = py as i32;

                if px >= widget_x
                    && px <= (widget_x + widget_width)
                    && py >= widget_y
                    && py <= (widget_y + widget_height)
                {
                    draw_rect_fill(px - 5, py - 5, 10, 10, Color::Black);
                    draw_text(
                        format!("x: {:.2} y: {:.2}", tip.x, tip.y).as_str(),
                        px + tip.lx,
                        py + tip.ly,
                    );
                }
            }

            // Zoom box
            if *zooming.borrow() {
                let mx = fltk::app::event_x();
                let my = fltk::app::event_y();
                let dx = mx - *zoom_x.borrow();
                let dy = my - *zoom_y.borrow();

                set_line_style(LineStyle::Dash, 1);
                draw_rect_with_color(
                    i32::min(*zoom_x.borrow(), *zoom_x.borrow() + dx),
                    i32::min(*zoom_y.borrow(), *zoom_y.borrow() + dy),
                    i32::abs(dx),
                    i32::abs(dy),
                    Color::Black,
                );
            }

            pop_clip();
        });
    }
}

impl MyWidget for Plot2DWidget {
    fn put_data(&mut self, x: &[f64], y: &[f64], style: LineStyle, width: i32, color: Color) {
        self.widget.put_data(x, y, style, width, color);
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

impl Deref for Plot2DWidget {
    type Target = Graph2DWidget;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl DerefMut for Plot2DWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}
