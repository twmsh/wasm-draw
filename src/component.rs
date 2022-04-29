use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub trait Component {
    fn type_id(&self) -> u32;
    fn update_mouse(&self, x: i32, y: i32);

    fn paint(&self, context: &web_sys::CanvasRenderingContext2d);
    fn can_move_on(&self, x: i32, y: i32) -> bool;
    fn can_select(&self, x: i32, y: i32) -> bool;
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

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

    pub lt_control: ControlPoint,
    pub rb_control: ControlPoint,

    pub is_move_on: bool,
    pub selected: bool,
    pub title: String,
}

impl Component for RectComponent {
    fn type_id(&self) -> u32 {
        1
    }

    fn update_mouse(&self, x: i32, y: i32) {}

    fn paint(&self, context: &CanvasRenderingContext2d) {
        context.set_stroke_style(&JsValue::from_str(self.line_color.as_str()));
        // 画矩形框
        context.stroke_rect(
            self.lt_point.x as f64,
            self.lt_point.y as f64,
            self.width as f64,
            self.height as f64,
        );

        // 画控制点
        self.lt_control.paint(context);
        self.rb_control.paint(context);

        // 画 title
        context.set_fill_style(&JsValue::from_str(self.line_color.as_str()));
        context.set_text_baseline("middle");
        context.set_text_align("cetner");
        context.set_font("16px serif");
        context.fill_text(
            &self.title,
            (self.lt_point.x + self.width as i32 / 2) as f64,
            (self.lt_point.y + 16) as f64,
        ).unwrap();
    }

    fn can_move_on(&self, x: i32, y: i32) -> bool {
        true
    }

    fn can_select(&self, x: i32, y: i32) -> bool {
        true
    }
}
