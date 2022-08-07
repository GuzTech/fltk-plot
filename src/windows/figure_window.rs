use std::ops::{Deref, DerefMut};

use fltk::{prelude::*, enums::Color, draw::*, window::*};

use crate::widgets::plot_2d_widget::Plot2DWidget;
use crate::widgets::stem_2d_widget::Stem2DWidget;
use crate::widgets::widget::MyWidget;

#[allow(dead_code)]
#[derive(Debug)]
pub enum PlotType {
    Plot,
    Stem,
    Quiver,
}

#[derive(Debug)]
pub struct FigureWindow {
    pub window: DoubleWindow,
    pub rows: usize,
    pub cols: usize,
    pub plots: Vec<Option<Box<dyn MyWidget>>>,
}

#[allow(dead_code)]
impl FigureWindow {
    const CAP_SPC: i32 = 25;
    const XLBL_SPC: i32 = 20;
    const YLBL_SPC: i32 = 20;

    pub fn new(w: i32, h: i32, lb: &str, rows: usize, cols: usize) -> FigureWindow {
        let mut v = Vec::with_capacity(rows * cols);
        v.resize_with(rows * cols, || None::<Box<dyn MyWidget>>);

        let mut win = DoubleWindow::default()
            .with_size(w, h)
            .with_label(lb)
            .center_screen();
        win.make_resizable(true);

        FigureWindow {
            window: win,
            plots: v,
            rows,
            cols,
        }
    }

    fn is_subplot_index_valid(&self, subplot: usize) -> bool {
        subplot < (self.rows * self.cols)
    }

    fn does_subplot_exist(&self, subplot: usize) -> bool {
        self.is_subplot_index_valid(subplot) && self.plots.get(subplot).unwrap().is_some()
    }

    fn create_subplot(
        &mut self,
        x: &[f64],
        y: &[f64],
        style: LineStyle,
        width: i32,
        color: Color,
        subplot: usize,
        plot_type: PlotType,
    ) {
        if !self.does_subplot_exist(subplot) {
            let row = subplot / self.cols;
            let column = subplot % self.cols;
            let dx = (self.window.width() - 20) / self.cols as i32;
            let dy = (self.window.height() - 20) / self.rows as i32;
            let posx = 10 + dx * column as i32;
            let posy = 10 + dy * row as i32;

            self.window.begin();

            match plot_type {
                PlotType::Plot => {
                    let mut plot_widget: Box<dyn MyWidget> = Box::new(Plot2DWidget::new(
                        posx + FigureWindow::YLBL_SPC,
                        posy + FigureWindow::CAP_SPC,
                        dx - FigureWindow::YLBL_SPC,
                        dy - (FigureWindow::CAP_SPC + FigureWindow::XLBL_SPC),
                        "Subfig",
                    ));
                    plot_widget.put_data(x, y, style, width, color);
                    self.plots[subplot] = Some(plot_widget);
                }
                PlotType::Stem => {
                    let mut stem_widget: Box<dyn MyWidget> = Box::new(Stem2DWidget::new(
                        posx + FigureWindow::YLBL_SPC,
                        posy + FigureWindow::CAP_SPC,
                        dx - FigureWindow::YLBL_SPC,
                        dy - (FigureWindow::CAP_SPC + FigureWindow::XLBL_SPC),
                        "Subfig",
                    ));
                    stem_widget.put_data(x, y, style, width, color);
                    self.plots[subplot] = Some(stem_widget);
                }
                PlotType::Quiver => {}
            }

            self.window.end();
        }
    }

    fn update_subplot(
        &mut self,
        x: &[f64],
        y: &[f64],
        style: LineStyle,
        width: i32,
        color: Color,
        subplot: usize,
    ) {
        if self.does_subplot_exist(subplot) {
            self.plots[subplot]
                .as_mut()
                .unwrap()
                .as_mut()
                .put_data(x, y, style, width, color);
        }
    }

    pub fn plot(
        &mut self,
        x: &[f64],
        y: &[f64],
        style: LineStyle,
        width: i32,
        color: Color,
        subplot: usize,
    ) {
        if !self.does_subplot_exist(subplot) {
            self.create_subplot(x, y, style, width, color, subplot, PlotType::Plot);
        } else {
            self.update_subplot(x, y, style, width, color, subplot);
        }
    }

    pub fn stem(
        &mut self,
        x: &[f64],
        y: &[f64],
        style: LineStyle,
        width: i32,
        color: Color,
        subplot: usize,
    ) {
        if !self.does_subplot_exist(subplot) {
            self.create_subplot(x, y, style, width, color, subplot, PlotType::Stem);
        } else {
            self.update_subplot(x, y, style, width, color, subplot);
        }
    }

    pub fn set_grid(&mut self, on: bool, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_grid(on);
        }
    }

    pub fn set_x_label(&mut self, label: &str, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_x_label(label);
        }
    }

    pub fn set_y_label(&mut self, label: &str, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_y_label(label);
        }
    }

    pub fn set_caption(&mut self, caption: &str, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_caption(caption);
        }
    }

    pub fn set_hold(&mut self, on: bool, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_hold(on);
        }
    }
}

impl Deref for FigureWindow {
    type Target = DoubleWindow;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for FigureWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}
