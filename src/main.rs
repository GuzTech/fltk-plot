pub use fltk::prelude::*;
use fltk::{app::*, draw::*, widget::*, window::*};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
enum PlotType {
    PLOT,
    STEM,
    QUIVER,
}

#[derive(Debug)]
struct FigureWindow {
    window: DoubleWindow,
    rows: usize,
    cols: usize,
    plots: Vec<Option<Box<dyn MyWidget>>>,
}

#[allow(dead_code)]
impl FigureWindow {
    const CAP_SPC: i32 = 25;
    const XLBL_SPC: i32 = 20;
    const YLBL_SPC: i32 = 20;

    fn new(w: i32, h: i32, lb: &str, rows: usize, cols: usize) -> FigureWindow {
        let mut v = Vec::with_capacity(rows * cols);
        v.resize_with(rows * cols, || None::<Box<dyn MyWidget>>);

        FigureWindow {
            window: DoubleWindow::default()
                .with_size(w, h)
                .with_label(lb)
                .center_screen(),
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
                PlotType::PLOT => {
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
                PlotType::STEM => {
                    // self.plots[subplot] = Some(Stem2DWidget::new(
                    //     posx + YLBL_SPC,
                    //     posy + CAP_SPC,
                    //     dx - YLBL_SPC,
                    //     dy - (CAP_SPC + XLBL_SPC),
                    //     "Subfig",
                    // ));
                    // self.plots[subplot].put_data(x, y, style, width, column);
                }
                PlotType::QUIVER => {}
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

    fn plot(
        &mut self,
        x: &[f64],
        y: &[f64],
        style: LineStyle,
        width: i32,
        color: Color,
        subplot: usize,
    ) {
        if !self.does_subplot_exist(subplot) {
            self.create_subplot(x, y, style, width, color, subplot, PlotType::PLOT);
        } else {
            self.update_subplot(x, y, style, width, color, subplot);
        }
    }

    fn set_grid(&mut self, on: bool, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_grid(on);
        }
    }

    fn set_x_label(&mut self, label: &str, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_x_label(label);
        }
    }

    fn set_y_label(&mut self, label: &str, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_y_label(label);
        }
    }

    fn set_caption(&mut self, caption: &str, subplot: usize) {
        if self.does_subplot_exist(subplot) {
            self.plots
                .get_mut(subplot)
                .unwrap()
                .as_mut()
                .unwrap()
                .set_caption(caption);
        }
    }

    fn set_hold(&mut self, on: bool, subplot: usize) {
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

#[derive(Clone, Debug)]
struct Plot2DData {
    style: LineStyle,
    width: i32,
    color: Color,
    x: Vec<f64>,
    y: Vec<f64>,
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
    length: usize,
}

#[allow(dead_code)]
impl Plot2DData {
    fn new(length: usize, style: LineStyle, width: i32, color: Color) -> Plot2DData {
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

    fn new_xy(x: &[f64], y: &[f64], style: LineStyle, width: i32, color: Color) -> Plot2DData {
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

    fn get_x_limit(&self, x_min: &mut f64, x_max: &mut f64) {
        *x_min = self.x_min;
        *x_max = self.x_max;
    }

    fn get_y_limit(&self, y_min: &mut f64, y_max: &mut f64) {
        *y_min = self.y_min;
        *y_max = self.y_max;
    }

    fn set_value(&mut self, index: usize, nx: f64, ny: f64) {
        if index < self.length {
            self.x[index] = nx;
            self.y[index] = ny;

            self.x_min = if self.x_min > nx { nx } else { self.x_min };
            self.x_max = if self.x_max < nx { nx } else { self.x_max };
            self.y_min = if self.y_min > ny { ny } else { self.y_min };
            self.y_max = if self.y_max < ny { ny } else { self.y_max };
        }
    }

    fn get_value(&self, index: usize, nx: &mut f64, ny: &mut f64) {
        if index < self.length {
            *nx = self.x[index];
            *ny = self.y[index];
        }
    }
}

#[derive(Debug)]
struct DataTip {
    x: f64,
    y: f64,
    plot: usize,
    lx: i32,
    ly: i32,
}

#[derive(Debug, Copy, Clone)]
struct Limit {
    x_left: f64,
    x_right: f64,
    y_left: f64,
    y_right: f64,
}

trait MyWidget {
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

#[derive(Clone, Debug)]
struct GraphWidget {
    widget: Widget,
    x_label: String,
    y_label: String,
    caption: String,
    limit: Limit,
    limit_c: Limit,
    curr_data_tip: Rc<RefCell<usize>>,
    zooming: Rc<RefCell<bool>>,
    grid: Rc<RefCell<bool>>,
    hold: Rc<RefCell<bool>>,
    zoom_x: Rc<RefCell<i32>>,
    zoom_y: Rc<RefCell<i32>>,
    xn_grid: Rc<RefCell<i32>>,
    yn_grid: Rc<RefCell<i32>>,
}

#[allow(dead_code)]
impl GraphWidget {
    fn new(x: i32, y: i32, width: i32, height: i32, caption: &str) -> GraphWidget {
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

    fn get_closest_point(
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
                let mut px = 0.0;
                let mut py = 0.0;

                d.get_value(i, &mut px, &mut py);

                // Scale and shift
                px = ((px - self.limit_c.x_left) / width) * w + self.widget.x() as f64;
                py = h - ((py - self.limit_c.y_left) / height) * h + self.widget.y() as f64;

                let dist = f64::max(f64::abs(px - mx as f64), f64::abs(py - my as f64));
                if dist < m_dist {
                    d.get_value(i, &mut px, &mut py);
                    m_dist = dist;
                    tip.x = px;
                    tip.y = py;
                    tip.plot = j;
                }
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

#[derive(Clone, Debug)]
struct Graph2DWidget {
    widget: GraphWidget,
    data: Rc<RefCell<Vec<Option<Plot2DData>>>>,
}

#[allow(dead_code)]
impl Graph2DWidget {
    fn new(x: i32, y: i32, width: i32, height: i32, caption: &str) -> Graph2DWidget {
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

#[derive(Clone, Debug)]
struct Plot2DWidget {
    widget: Graph2DWidget,
}

#[allow(dead_code)]
impl Plot2DWidget {
    fn new(x: i32, y: i32, width: i32, height: i32, caption: &str) -> Plot2DWidget {
        let mut x = Plot2DWidget {
            widget: Graph2DWidget::new(x, y, width, height, caption),
        };
        x.draw();
        x.handle();
        x
    }

    fn draw(&mut self) {
        let limit_c = self.limit_c;
        // TODO: These two below are not cloned, maybe they should be. I need to check this!
        let xn = *self.xn_grid.borrow();
        let yn = *self.yn_grid.borrow();
        let grid = self.grid.clone();
        let data = self.data.clone();

        self.widget.draw2(move |p| {
            let width = limit_c.x_right - limit_c.x_left;
            let height = limit_c.y_right - limit_c.y_left;

            let x = p.x();
            let y = p.y();
            let wd = p.width();
            let ht = p.height();

            // Background
            draw_rect_fill(x, y, wd, ht, Color::White);
            set_line_style(LineStyle::Solid, 1); // TODO - Have to check if 1 is correct here!
            draw_rect_with_color(x, y, wd, ht, Color::Black);

            // Captions and labels
            set_font(Font::Helvetica, 12);
            set_draw_color(Color::Black);
            //
            //

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
                draw_line(x + dx * i, y + height as i32, x + dx * i, y + ht - 10);
                //
                //
            }

            let dy = ht / (yn + 1);
            for i in 1..=yn {
                if *grid.borrow() {
                    set_draw_color(Color::Light2);
                    set_line_style(LineStyle::Dash, 1);
                    draw_line(x, y + dy * i, x + width as i32, y + dy * i);
                }

                set_draw_color(Color::Black);
                set_line_style(LineStyle::Solid, 1);
                draw_line(x, y + dy * i, x + 10, y + dy * i);
                draw_line(
                    x + width as i32,
                    y + dy * i,
                    x + width as i32 - 10,
                    y + dy * i,
                );
                //
                //
            }

            // Plot the data
            for plot_data in &*data.borrow() {
                if plot_data.is_some() {
                    let plot = plot_data.as_ref().unwrap();
                    set_draw_color(plot.color);
                    set_line_style(plot.style, plot.width);

                    let mut x_min = 0.0;
                    let mut x_max = 0.0;
                    let mut y_min = 0.0;
                    let mut y_max = 0.0;
                    plot.get_x_limit(&mut x_min, &mut x_max);
                    plot.get_y_limit(&mut y_min, &mut y_max);
                    let x_scale = width / (x_max - x_min);
                    let y_scale = height / (y_max - y_min);

                    begin_line();
                    for j in 0..plot.length {
                        let mut px = 0.0;
                        let mut py = 0.0;

                        plot.get_value(j, &mut px, &mut py);

                        // Scale and shift
                        // px = ((px - limit_c.x_left) / width) * wd as f64 + x as f64;
                        // py = ht as f64 - ((py - limit_c.y_left) / height) * ht as f64 + y as f64;
                        px = (px - x_min) * x_scale + x as f64;
                        py = (py - y_min) * y_scale + y as f64;
                        vertex(px, py);
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

fn linspace(arr: &mut Vec<f64>, l: f64, r: f64, len: usize) {
    let dx = (r - l) / (len - 1) as f64;

    arr.reserve(len);

    for i in 0..len {
        arr.push(l + (i as f64 * dx));
    }
}

fn arrayfun(input: &[f64], output: &mut Vec<f64>, fun: &dyn Fn(f64) -> f64) {
    output.reserve(input.len());

    for i in input.iter() {
        output.push(fun(*i));
    }
}

fn main() {
    let app = App::default();

    const N: usize = 1000;
    let mut t: Vec<f64> = Vec::with_capacity(N);
    let mut x: Vec<f64> = Vec::with_capacity(N);
    let mut y: Vec<f64> = Vec::with_capacity(N);

    linspace(&mut t, 0.0, 2.0 * std::f64::consts::PI, N);
    arrayfun(&t, &mut x, &f64::sin);
    arrayfun(&t, &mut y, &|x: f64| 0.5 - 0.5 * f64::exp(-x));

    let mut fig = FigureWindow::new(640, 480, "Figure 1", 1, 2);
    fig.plot(&t, &x, LineStyle::Solid, 2, Color::Red, 0);
    fig.set_hold(true, 0);
    fig.set_grid(true, 0);
    fig.plot(&t, &y, LineStyle::Dash, 2, Color::Blue, 0);
    fig.set_hold(false, 0);
    fig.plot(&x, &y, LineStyle::Solid, 2, Color::Red, 1);

    fig.end();
    fig.show();

    app.run().unwrap();
}
