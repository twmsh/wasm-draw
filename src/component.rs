use wasm_bindgen::JsValue;
use crate::log;
use web_sys::CanvasRenderingContext2d;


pub trait Component {
    fn id(&self) -> u32;
    fn type_id(&self) -> u32;
    fn style(&self) -> ComponentStyle;

    fn update_mouse(&mut self, x: i32, y: i32);

    fn paint(&self, context: &web_sys::CanvasRenderingContext2d);


    fn try_select(&mut self, x: i32, y: i32) -> bool;

    fn selected(&self) -> bool;
    fn set_select(&mut self, s:bool);
}

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    pub font: String,

    pub line_width: u32,
    pub line_color: String,
    pub line_focus_color: String,

    pub control_line_width: u32,
    pub control_width: u32,
    pub control_line_color: String,
    pub control_fill_color: String,

}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct ControlPoint {
    pub point: Point,
    pub width: u32,
    pub selected: bool,
}

impl ControlPoint {
    pub fn new(x: i32, y: i32, width: u32) -> Self {
        Self {
            point: Point { x, y },
            width,
            selected: false,
        }
    }

    pub fn can_select(&self, x: i32, y: i32) -> bool {
        let left = self.point.x - (self.width / 2) as i32;
        let right = self.point.x + (self.width / 2) as i32;
        let top = self.point.y - (self.width / 2) as i32;
        let bottom = self.point.y + (self.width / 2) as i32;

        let ret = x >= left && x <= right && y >= top && y <= bottom;
        log(&format!("{} {} {} {}, {} {}, {}",left,right,top,bottom,x,y,ret));

        ret

    }


    fn paint(&self, context: &CanvasRenderingContext2d, style: &ComponentStyle) {
        let left = self.point.x - (self.width / 2) as i32;
        let top = self.point.y - (self.width / 2) as i32;

        context.set_line_width(style.control_line_width as f64);
        context.set_fill_style(&JsValue::from_str(style.control_fill_color.as_str()));
        context.fill_rect(left as f64,
                          top as f64,
                          self.width as f64,
                          self.width as f64, );

        context.set_stroke_style(&JsValue::from_str(style.control_line_color.as_str()));
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
    pub id: u32,
    pub style: ComponentStyle,

    pub width: u32,
    pub height: u32,

    pub title: String,

    pub start_control: ControlPoint,
    pub end_control: ControlPoint,

    pub selected: bool,

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

        (x + (self.width / 2) as f64, y)
    }
}

impl Component for RectComponent {
    fn id(&self) -> u32 {
        self.id
    }

    fn type_id(&self) -> u32 {
        4
    }

    fn style(&self) -> ComponentStyle {
        self.style.clone()
    }

    fn update_mouse(&mut self, x: i32, y: i32) {
        if self.start_control.selected {
            self.start_control.point.x = x;
            self.start_control.point.y = y;
        } else if self.end_control.selected {
            self.end_control.point.x = x;
            self.end_control.point.y = y;
        }
    }

    fn paint(&self, context: &CanvasRenderingContext2d) {

        // 设置线颜色和宽带
        context.set_stroke_style(&JsValue::from_str(self.style.line_color.as_str()));
        context.set_line_width(self.style.line_width as f64);

        // 画矩形框
        let (lt_x, lt_y) = self.lt_point();
        context.stroke_rect(lt_x, lt_y, self.width as f64, self.height as f64);

        // 画控制点
        self.start_control.paint(context, &self.style);
        self.end_control.paint(context, &self.style);

        // 设置字体相关信息
        context.set_fill_style(&JsValue::from_str(self.style.line_color.as_str()));
        context.set_text_baseline("middle");
        context.set_text_align("cetner");
        context.set_font(self.style.font.as_str());

        // 画 title
        let (title_x, title_y) = self.title_position();
        let title_offset = 16.00;
        context.fill_text(&self.title, title_x - title_offset, title_y + title_offset).unwrap();
    }


    fn try_select(&mut self, x: i32, y: i32) -> bool {
        if self.start_control.can_select(x,y) {
            self.start_control.selected = true;
            return true;
        }

        if self.end_control.can_select(x,y) {
            self.end_control.selected = true;
            return true;
        }

        false
    }


    fn selected(&self) -> bool {
        self.selected
    }

    fn set_select(&mut self,s: bool) {
        self.selected = s;
        if !self.selected {
            self.start_control.selected = false;
            self.end_control.selected = false;
        }

    }
}

//-----------------------------------------------------
pub struct LineComponent {
    pub id: u32,
    pub style: ComponentStyle,

    pub title: String,

    pub start_control: ControlPoint,
    pub end_control: ControlPoint,

    pub selected: bool,

}

impl LineComponent {
    pub fn title_position(&self) -> (f64, f64) {
        let x = self.start_control.point.x + (self.end_control.point.x - self.start_control.point.x) / 2;
        let y = self.start_control.point.y + (self.end_control.point.y - self.start_control.point.y) / 2;
        (x as f64, y as f64)
    }
}

impl Component for LineComponent {
    fn id(&self) -> u32 {
        self.id
    }

    fn type_id(&self) -> u32 {
        1
    }

    fn style(&self) -> ComponentStyle {
        self.style.clone()
    }

    fn update_mouse(&mut self, x: i32, y: i32) {
        if self.start_control.selected {
            self.start_control.point.x = x;
            self.start_control.point.y = y;
        } else if self.end_control.selected {
            self.end_control.point.x = x;
            self.end_control.point.y = y;
        }
    }

    fn paint(&self, context: &CanvasRenderingContext2d) {

        // 设置线颜色和宽带
        context.set_stroke_style(&JsValue::from_str(self.style.line_color.as_str()));
        context.set_line_width(self.style.line_width as f64);

        // 画直线
        context.begin_path();
        context.move_to(
            self.start_control.point.x as f64,
            self.start_control.point.y as f64,
        );
        context.line_to(self.end_control.point.x as f64, self.end_control.point.y as f64);
        context.stroke();

        // 画控制点
        self.start_control.paint(context, &self.style);
        self.end_control.paint(context, &self.style);

        // 设置字体相关信息
        context.set_fill_style(&JsValue::from_str(self.style.line_color.as_str()));
        context.set_text_baseline("middle");
        context.set_text_align("cetner");
        context.set_font(self.style.font.as_str());

        // 画 title
        let (title_x, title_y) = self.title_position();
        let title_offset = 4.00;
        context.fill_text(&self.title, title_x + title_offset, title_y + title_offset).unwrap();
    }


    fn try_select(&mut self, x: i32, y: i32) -> bool {
        if self.start_control.can_select(x,y) {
            self.start_control.selected = true;
            return true;
        }

        if self.end_control.can_select(x,y) {
            self.end_control.selected = true;
            return true;
        }

        false
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_select(&mut self,s: bool) {
        self.selected = s;
        if !self.selected {
            self.start_control.selected = false;
            self.end_control.selected = false;
        }

    }
}

//-----------------------------------------------------
pub struct CircleComponent {
    pub id: u32,
    pub style: ComponentStyle,

    pub title: String,

    pub start_control: ControlPoint,
    pub end_control: ControlPoint,

    pub radius: u32,

    pub selected: bool,

}

impl CircleComponent {
    pub fn title_position(&self) -> (f64, f64) {
        let x = self.start_control.point.x + (self.end_control.point.x - self.start_control.point.x) / 2;
        let y = self.start_control.point.y + (self.end_control.point.y - self.start_control.point.y) / 2;
        (x as f64, y as f64)
    }
}

impl Component for CircleComponent {
    fn id(&self) -> u32 {
        self.id
    }

    fn type_id(&self) -> u32 {
        2
    }

    fn style(&self) -> ComponentStyle {
        self.style.clone()
    }

    fn update_mouse(&mut self, x: i32, y: i32) {}

    fn paint(&self, context: &CanvasRenderingContext2d) {

        // 设置线颜色和宽带
        context.set_stroke_style(&JsValue::from_str(self.style.line_color.as_str()));
        context.set_line_width(self.style.line_width as f64);

        // 画直线
        context.begin_path();

        context.arc(
            self.start_control.point.x as f64,
            self.start_control.point.y as f64,
            self.radius as f64,
            0.0,
            2 as f64 * std::f64::consts::PI,
        ).unwrap();
        context.stroke();

        // 画控制点
        self.start_control.paint(context, &self.style);
        self.end_control.paint(context, &self.style);

        // 设置字体相关信息
        context.set_fill_style(&JsValue::from_str(self.style.line_color.as_str()));
        context.set_text_baseline("middle");
        context.set_text_align("cetner");
        context.set_font(self.style.font.as_str());

        // 画 title
        let (title_x, title_y) = self.title_position();
        let title_offset = 16.00;
        context.fill_text(&self.title, title_x - title_offset, title_y + title_offset).unwrap();
    }


    fn try_select(&mut self, x: i32, y: i32) -> bool {
        true
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_select(&mut self,s: bool) {
        self.selected = s;
        if !self.selected {
            self.start_control.selected = false;
            self.end_control.selected = false;
        }

    }
}
