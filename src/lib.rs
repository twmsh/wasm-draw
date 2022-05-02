mod component;
mod render;
mod utils;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use component::*;
use render::*;
use std::rc::Rc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type ComponentVec = HashMap<u32, Box<dyn Component>>;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Copy, Clone)]
pub struct BgImgInfo {
    scale: f64,
    dx: f64,
    dy: f64,
    width: f64,
    height: f64,
    origin_width: f64,
    origin_height: f64,
}

impl BgImgInfo {
    fn new(c_width: u32, c_height: u32, img_width: u32, img_height: u32) -> BgImgInfo {
        let c_width = c_width as f64;
        let c_height = c_height as f64;
        let img_width = img_width as f64;
        let img_height = img_height as f64;

        let c_ratio = c_width / c_height;
        let i_ratio = img_width / img_height;
        let scale = if i_ratio > c_ratio {
            c_width / img_width
        } else {
            c_height / img_height
        };

        let dx = (c_width - img_width * scale) / 2 as f64;
        let dy = (c_height - img_height * scale) / 2 as f64;
        BgImgInfo {
            dx,
            dy,
            scale,
            width: img_width * scale,
            height: img_height * scale,
            origin_width: img_width,
            origin_height: img_height,
        }
    }
}

#[wasm_bindgen]
pub struct FyCanvas {
    id: String,
    height: u32,
    width: u32,

    canvas: web_sys::HtmlCanvasElement,

    render: Rc<RefCell<FyRender>>,
    bg_img: Rc<Cell<Option<BgImgInfo>>>,
    childs: Rc<RefCell<ComponentVec>>,
}

#[wasm_bindgen]
impl FyCanvas {
    pub fn new(id: &str) -> Result<FyCanvas, JsValue> {
        let document = document();

        let canvas = document
            .get_element_by_id(id)
            .ok_or(JsError::new("body not find"))?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        let canvas_height = canvas.height();
        let canvas_width = canvas.width();

        let canvas_context = canvas
            .get_context("2d")?
            .ok_or(JsError::new("document not find"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let cache_canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        cache_canvas.set_width(canvas_height);
        cache_canvas.set_height(canvas_width);

        let cache_context = cache_canvas
            .get_context("2d")?
            .ok_or(JsError::new("document not find"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let render = FyRender::new(canvas_context, cache_canvas, cache_context);

        let childs = Rc::new(RefCell::new(HashMap::new()));

        // 加测试数据
        let component = test_create_rect_component(1, 100, 100);
        childs.borrow_mut().insert(component.id(), component);

        // let component = test_create_rect_component(1, 150, 300);
        // childs.borrow_mut().insert(component.id(), component);

        // let component = test_create_line_component(3, 150, 300, 300, 200);
        // childs.borrow_mut().insert(component.id(), component);

        let component = test_create_line_component(4, 210, 130, 110, 240);
        childs.borrow_mut().insert(component.id(), component);

        // childs
        //     .borrow_mut()
        //     .push(test_create_circle_component(5,210, 130,100));
        //
        // childs
        //     .borrow_mut()
        //     .push(test_create_circle_component(6,230, 180, 50));

        Ok(FyCanvas {
            id: id.to_string(),
            height: canvas_height,
            width: canvas_width,
            canvas,
            render: Rc::new(RefCell::new(render)),
            bg_img: Rc::new(Cell::new(None)),
            childs,
        })
    }

    pub fn bind_bg_input(&self, input_id: &str) -> Result<(), JsValue> {
        let document = document();
        let input = document
            .get_element_by_id(input_id)
            .ok_or(JsError::new("input not find"))?
            .dyn_into::<web_sys::HtmlInputElement>()?;

        let file_reader = web_sys::FileReader::new()?;
        let img = web_sys::HtmlImageElement::new()?;

        let width = self.width;
        let height = self.height;

        let bg = self.bg_img.clone();
        let render = self.render.clone();
        let childs = self.childs.clone();

        // img onload 回调
        let closure_image = Closure::wrap(Box::new(move |event: web_sys::Event| {
            log(&format!("--> closure_image, type: {:?}", event.type_()));
            let ele_image = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlImageElement>()
                .unwrap();
            let img_width = ele_image.width();
            let img_height = ele_image.height();
            log(&format!(
                "--> closure_image, img, width: {:?}, height: {:?}",
                img_width, img_height
            ));

            let bg_info = BgImgInfo::new(width, height, img_width, img_height);
            log(&format!("--> closure_image, bg_info: {:?}", bg_info));

            bg.set(Some(bg_info));
            log("--> closure_image, draw bg on cache");
            render.borrow().update_bg(&ele_image, &bg_info);
            FyCanvas::repaint(render.clone(), childs.clone());
        }) as Box<dyn FnMut(_)>);
        img.set_onload(Some(closure_image.as_ref().unchecked_ref()));
        closure_image.forget();

        // filereader onload 回调
        let closure_reader = Closure::wrap(Box::new(move |event: web_sys::Event| {
            log(&format!("--> closure_reader, type: {:?}", event.type_()));
            let ele_reader = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::FileReader>()
                .unwrap();
            img.set_src(ele_reader.result().unwrap().as_string().unwrap().as_str());
        }) as Box<dyn FnMut(_)>);
        file_reader.set_onload(Some(closure_reader.as_ref().unchecked_ref()));
        closure_reader.forget();

        // input change 回调
        let closure_input = Closure::wrap(Box::new(move |event: web_sys::Event| {
            log(&format!("--> closure_input, type: {:?}", event.type_()));
            let ele_input = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            let file = ele_input.files().unwrap().get(0).unwrap();
            file_reader.read_as_data_url(&file).unwrap();
        }) as Box<dyn FnMut(_)>);

        input.add_event_listener_with_callback("change", closure_input.as_ref().unchecked_ref())?;
        closure_input.forget();

        self.bind_mouse_event();

        Ok(())
    }
}

impl FyCanvas {
    pub fn bind_mouse_event(&self) {
        let render = self.render.clone();
        let render2 = self.render.clone();
        let render3 = self.render.clone();

        let childs = self.childs.clone();
        let childs2 = self.childs.clone();
        let childs3 = self.childs.clone();

        // 鼠标down
        let closure_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            log(&format!("--> mouse down, type: {:?}", event));
            log(&format!(
                "--> screen:({},{}), client:({},{}), offset:({},{})",
                event.screen_x(),
                event.screen_y(),
                event.client_x(),
                event.client_y(),
                event.offset_x(),
                event.offset_y(),
            ));

            //
            render.borrow_mut().mouse_down(childs.clone(),event.offset_x(),
                                           event.offset_y(),);

            // 刷新ui
            FyCanvas::repaint(render.clone(), childs.clone());

        }) as Box<dyn FnMut(_)>);

        self.canvas
            .add_event_listener_with_callback("mousedown", closure_down.as_ref().unchecked_ref())
            .unwrap();
        closure_down.forget();


        // 鼠标移动
        let closure_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            // log(&format!("--> mouse move, button:{}, type: {:?}", event.buttons(),event));
            log(&format!(
                "--> screen:({},{}), client:({},{}), offset:({},{})",
                event.screen_x(),
                event.screen_y(),
                event.client_x(),
                event.client_y(),
                event.offset_x(),
                event.offset_y(),
            ));

            if event.buttons() ==1 {
                //
                render2.borrow_mut().mouse_move(childs2.clone(),event.offset_x(),
                                                event.offset_y(),);

                // 刷新ui
                FyCanvas::repaint(render2.clone(), childs2.clone());
                log("mouse move, repaint");
            }


        }) as Box<dyn FnMut(_)>);

        self.canvas
            .add_event_listener_with_callback("mousemove", closure_move.as_ref().unchecked_ref())
            .unwrap();
        closure_move.forget();

        // 鼠标up
        let closure_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            log(&format!("--> mouse up, type: {:?}", event));
            log(&format!(
                "--> screen:({},{}), client:({},{}), offset:({},{})",
                event.screen_x(),
                event.screen_y(),
                event.client_x(),
                event.client_y(),
                event.offset_x(),
                event.offset_y(),
            ));

            //
            render3.borrow_mut().mouse_up(childs3.clone(),event.offset_x(),
                                           event.offset_y(),);

            // 刷新ui
            FyCanvas::repaint(render3.clone(), childs3.clone());

        }) as Box<dyn FnMut(_)>);

        self.canvas
            .add_event_listener_with_callback("mouseup", closure_up.as_ref().unchecked_ref())
            .unwrap();
        closure_up.forget();
    }

    fn repaint(render: Rc<RefCell<FyRender>>, childs: Rc<RefCell<ComponentVec>>) {
        let closure = Closure::wrap(Box::new(move || {
            render.borrow().paint(childs.clone());
        }) as Box<dyn FnMut()>);

        request_animation_frame(&closure);
        closure.forget();
    }
}

//-----------------------------------------------------------------
//-----------------------------------------------------------------

fn test_create_rect_component(id: u32, x: i32, y: i32) -> Box<dyn Component> {
    let width = 200;
    let height = 100;
    let control_width = 8;

    let style = ComponentStyle {
        font: "16px serif".to_string(),
        line_width: 2,
        line_color: "blue".to_string(),
        line_focus_color: "red".to_string(),
        control_line_width: 2,
        control_width,
        control_line_color: "blue".to_string(),
        control_fill_color: "red".to_string(),
    };

    let comp = RectComponent {
        id,
        style,

        width,
        height,

        start_control: ControlPoint {
            point: Point { x, y },
            width: control_width,
            selected: false
        },
        end_control: ControlPoint {
            point: Point {
                x: x + width as i32,
                y: y + height as i32,
            },
            width: control_width,
            selected: false
        },

        selected: false,
        title: "抓拍区域".to_string(),
    };
    Box::new(comp)
}

fn test_create_line_component(id: u32, x1: i32, y1: i32, x2: i32, y2: i32) -> Box<dyn Component> {
    let control_width = 8;

    let style = ComponentStyle {
        font: "16px serif".to_string(),
        line_width: 2,
        line_color: "blue".to_string(),
        line_focus_color: "red".to_string(),
        control_line_width: 2,
        control_width: 8,
        control_line_color: "blue".to_string(),
        control_fill_color: "red".to_string(),
    };

    let comp = LineComponent {
        id,
        style,
        title: "边界线".to_string(),

        start_control: ControlPoint {
            point: Point { x: x1, y: y1 },
            width: control_width,
            selected: false
        },
        end_control: ControlPoint {
            point: Point { x: x2, y: y2 },
            width: control_width,
            selected: false
        },

        selected: false,
    };
    Box::new(comp)
}

fn test_create_circle_component(id: u32, x: i32, y: i32, radius: u32) -> Box<dyn Component> {
    let control_width = 8;

    let style = ComponentStyle {
        font: "16px serif".to_string(),
        line_width: 2,
        line_color: "blue".to_string(),
        line_focus_color: "red".to_string(),
        control_line_width: 2,
        control_width: 8,
        control_line_color: "blue".to_string(),
        control_fill_color: "red".to_string(),
    };

    let comp = CircleComponent {
        id,
        style,
        title: "圆形".to_string(),

        start_control: ControlPoint {
            point: Point { x, y },
            width: control_width,
            selected: false
        },
        end_control: ControlPoint {
            point: Point {
                x: x + radius as i32,
                y,
            },
            width: control_width,
            selected: false
        },

        radius,
        selected: false,
    };
    Box::new(comp)
}

//---------------------------------------------------------
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}
