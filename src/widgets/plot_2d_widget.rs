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
        let limit_c = self.limit_c;
        // TODO: These two below are not cloned, maybe they should be. I need to check this!
        let xn = *self.xn_grid.borrow();
        let yn = *self.yn_grid.borrow();
        let grid = self.grid.clone();
        let data = self.data.clone();
        let caption = self.widget.caption.clone();
        let x_label = self.widget.x_label.clone();
        let y_label = self.widget.y_label.clone();

        self.widget.draw2(move |p| {
            let lim_width = limit_c.x_right - limit_c.x_left;
            let lim_height = limit_c.y_right - limit_c.y_left;

            let x = p.x();
            let y = p.y();
            let wd = p.width();
            let ht = p.height();

            // Background
            draw_rect_fill(x, y, wd, ht, Color::White);
            set_line_style(LineStyle::Solid, 1);
            draw_rect_with_color(x, y, wd, ht, Color::Black);

            // Captions and labels
            set_font(Font::Helvetica, 12);
            set_draw_color(Color::Black);
            let text_width = width(caption.borrow().as_str()) as i32;
            draw_text(
                caption.borrow().as_str(),
                x + (lim_width as i32 / 2) - (text_width / 2),
                y - 7,
            );
            let text_width = width(x_label.borrow().as_str()) as i32;
            draw_text(
                x_label.borrow().as_str(),
                x + (lim_width as i32 / 2) - (text_width / 2),
                y + lim_height as i32 + 12,
            );
            let text_width = width(y_label.borrow().as_str()) as i32;
            draw_text_angled(
                90,
                y_label.borrow().as_str(),
                x - 5,
                y + (lim_height as i32 / 2) + (text_width / 2),
            );

            push_clip(x, y, wd, ht);

            // Draw grid lines
            set_font(Font::Helvetica, 10);
            set_draw_color(Color::Black);

            let dx = wd / (xn + 1);
            for i in 1..=xn {
                if *grid.borrow() {
                    set_draw_color(Color::Light2);
                    set_line_style(LineStyle::Dash, 1);
                    draw_line(x + dx * i, y, x + dx * i, y + ht);
                }

                set_draw_color(Color::Black);
                set_line_style(LineStyle::Solid, 1);
                draw_line(x + dx * i, y, x + dx * i, y + 10);
                draw_line(x + dx * i, y + lim_height as i32, x + dx * i, y + ht - 10);
                draw_text(
                    format!(
                        "{:.2}",
                        i as f64 * wd as f64 / (xn as f64 + 1.0) + limit_c.x_left
                    )
                    .as_str(),
                    x + dx * i + 2,
                    y + ht as i32 - 2,
                );
            }

            let dy = ht / (yn + 1);
            for i in 1..=yn {
                if *grid.borrow() {
                    set_draw_color(Color::Light2);
                    set_line_style(LineStyle::Dash, 1);
                    draw_line(x, y + dy * i, x + lim_width as i32, y + dy * i);
                }

                set_draw_color(Color::Black);
                set_line_style(LineStyle::Solid, 1);
                draw_line(x, y + dy * i, x + 10, y + dy * i);
                draw_line(
                    x + lim_width as i32,
                    y + dy * i,
                    x + lim_width as i32 - 10,
                    y + dy * i,
                );
                draw_text(
                    format!(
                        "{:.2}",
                        i as f64 * ht as f64 / (yn as f64 + 1.0) + limit_c.y_left
                    )
                    .as_str(),
                    x + 2,
                    ht as i32 + y - dy * i - 5,
                );
            }

            // Plot the data
            for plot_data in &*data.borrow() {
                if plot_data.is_some() {
                    let plot = plot_data.as_ref().unwrap();
                    set_draw_color(plot.color);
                    set_line_style(plot.style, plot.width);

                    let (x_min, x_max) = plot.get_x_limit();
                    let (y_min, y_max) = plot.get_y_limit();
                    let x_scale = lim_width / (x_max - x_min);
                    let y_scale = lim_height / (y_max - y_min);

                    begin_line();
                    for j in 0..plot.length {
                        if let Some((mut px, mut py)) = plot.get_value(j) {
                            // Scale and shift
                            px = (px - x_min) * x_scale + x as f64;
                            py = (py - y_min) * y_scale + y as f64;
                            vertex(px, py);
                        };
                    }
                    end_line();
                }
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
