mod utils;

use std::cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::rc::Rc;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
            c_height / c_width
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

struct FyRender {
    canvas_ctx: web_sys::CanvasRenderingContext2d,

    cache_canvas: web_sys::HtmlCanvasElement,
    cache_ctx: web_sys::CanvasRenderingContext2d,
}

impl FyRender {
    pub fn new(
        canvas_ctx: web_sys::CanvasRenderingContext2d,
        cache_canvas: web_sys::HtmlCanvasElement,
        cache_ctx: web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            canvas_ctx,
            cache_canvas,
            cache_ctx,
        }
    }

    pub fn update_bg(&self, image: &web_sys::HtmlImageElement, bg_info: &BgImgInfo) {
        let img_width = image.width();
        let img_height = image.height();

        let width = self.cache_canvas.width() as f64;
        let height = self.cache_canvas.height() as f64;

        self.cache_ctx.clear_rect(0.0, 0.0, width, height);
        self.cache_ctx
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                0.0,
                0.0,
                img_width as f64,
                img_height as f64,
                bg_info.dx,
                bg_info.dy,
                bg_info.width,
                bg_info.height,
            )
            .unwrap();
    }

    pub fn update_model(&self) {}

    pub fn paint(&self) {
        self.canvas_ctx.draw_image_with_html_canvas_element(&self.cache_canvas, 0.0, 0.0).unwrap();
    }


}


#[wasm_bindgen]
pub struct FyCanvas {
    id: String,
    height: u32,
    width: u32,

    render: Rc<FyRender>,
    bg_img: Rc<Cell<Option<BgImgInfo>>>,
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


        Ok(FyCanvas {
            id: id.to_string(),
            height: canvas_height,
            width: canvas_width,
            render: Rc::new(render),
            bg_img: Rc::new(Cell::new(None)),
        })
    }

    pub fn bind_bg_input(&self, input_id: &str) -> Result<(), JsValue> {
        let document = document();
        let input = document
            .get_element_by_id(input_id)
            .ok_or(JsError::new("body not find"))?
            .dyn_into::<web_sys::HtmlInputElement>()?;

        let file_reader = web_sys::FileReader::new()?;
        let img = web_sys::HtmlImageElement::new()?;

        let width = self.width;
        let height = self.height;

        let bg = self.bg_img.clone();
        let render = self.render.clone();


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
            render.update_bg(&ele_image, &bg_info);

            FyCanvas::repaint(&render);

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

        Ok(())
    }

}

impl FyCanvas {
    fn repaint(render: &Rc<FyRender>) {

        let render_cl = render.clone();

        let closure = Closure::wrap(Box::new(move||{
            render_cl.paint();
        }) as Box<dyn FnMut()>);
        request_animation_frame(&closure);
    }

}

//-----------------------------------------------------------------
//-----------------------------------------------------------------

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