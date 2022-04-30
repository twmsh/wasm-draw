use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub trait Component {
    fn type_id(&self) -> u32;
    fn update_mouse(&self, x: i32, y: i32);

    fn paint(&self, context: &web_sys::CanvasRenderingContext2d);
    fn can_move_on(&self, x: i32, y: i32) -> bool;
    fn can_select(&self, x: i32, y: i32) -> bool;
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct ControlPoint {
    pub point: Point,
    pub width: u32,
}

impl ControlPoint {
    fn paint(&self, context: &CanvasRenderingContext2d) {
        let left = self.point.x - (self.width / 2) as i32;
        let top = self.point.y - (self.width / 2) as i32;

        // 画矩形框
        context.stroke_rect(
            left as f64,
            top as f64,
            self.width as f64,
            self.width as f64,
        );
    }
}

pub struct RectComponent {
    pub lt_point: Point,
    pub width: u32,
    pub height: u32,

    pub line_width: u32,
    pub line_color: String,
    pub focus_color: String,

    pub start_control: ControlPoint,
    pub end_control: ControlPoint,

    pub is_move_on: bool,
    pub selected: bool,
    pub title: String,
}

impl RectComponent {
    fn lt_point(&self) -> (f64, f64) {
        let start = self.start_control.point;
        let end = self.start_control.point;
        match (start.x >= end.x, start.y >= end.y) {
            (false, false) => (start.x as f64, start.y as f64),
            (true, true) => (end.x as f64, end.y as f64),
            (true, false) => (end.x as f64, start.y as f64),
            (false, true) => (start.x as f64, end.y as f64),
        }
    }

    pub fn title_position(&self) -> (f64, f64) {
        let (x, y) = self.lt_point();

        (x + (self.width  / 2) as f64, y)
    }
}

impl Component for RectComponent {
    fn type_id(&self) -> u32 {
        1
    }

    fn update_mouse(&self, x: i32, y: i32) {}

    fn paint(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str(self.line_color.as_str()));
        context.set_line_width(self.line_width as f64);

        let (lt_x, lt_y) = self.lt_point();
        // 画矩形框
        context.stroke_rect(lt_x, lt_y, self.width as f64, self.height as f64);

        // 画控制点
        self.start_control.paint(context);
        self.end_control.paint(context);

        // 画 title
        context.set_fill_style(&JsValue::from_str(self.line_color.as_str()));
        context.set_text_baseline("middle");
        context.set_text_align("cetner");
        context.set_font("16px serif");

        let (title_x, title_y) = self.title_position();
        let title_offset = 16.00;
        context.fill_text(&self.title, title_x - title_offset, title_y + title_offset).unwrap();
    }

    fn can_move_on(&self, x: i32, y: i32) -> bool {
        true
    }

    fn can_select(&self, x: i32, y: i32) -> bool {
        true
    }
}

//-----------------------------------------------------
pub struct LineComponent {
    pub start_point: ControlPoint,
    pub end_point: ControlPoint,

    pub line_width: u32,
    pub line_color: String,
    pub focus_color: String,

    pub is_move_on: bool,
    pub selected: bool,
    pub title: String,
}

impl LineComponent {
    pub fn title_position(&self) -> (f64, f64) {
        let x = self.start_point.point.x + (self.end_point.point.x - self.start_point.point.x) / 2;
        let y = self.start_point.point.y + (self.end_point.point.y - self.start_point.point.y) / 2;
        (x as f64, y as f64)
    }
}

impl Component for LineComponent {
    fn type_id(&self) -> u32 {
        2
    }

    fn update_mouse(&self, x: i32, y: i32) {}

    fn paint(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str(self.line_color.as_str()));
        context.set_line_width(self.line_width as f64);

        // 画直线
        context.begin_path();
        context.move_to(
            self.start_point.point.x as f64,
            self.start_point.point.y as f64,
        );
        context.line_to(self.end_point.point.x as f64, self.end_point.point.y as f64);
        context.stroke();

        // 画控制点
        self.start_point.paint(context);
        self.end_point.paint(context);

        // 画 title
        context.set_fill_style(&JsValue::from_str(self.line_color.as_str()));
        context.set_text_baseline("middle");
        context.set_text_align("cetner");
        context.set_font("16px serif");

        let (title_x, title_y) = self.title_position();
        let title_offset = 16.00;
        context.fill_text(&self.title, title_x - title_offset, title_y).unwrap();
    }

    fn can_move_on(&self, x: i32, y: i32) -> bool {
        true
    }

    fn can_select(&self, x: i32, y: i32) -> bool {
        true
    }
}
