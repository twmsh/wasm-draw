mod utils;

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

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-draw!");
}


pub struct FyCanvasCtx {
    pub canvas_id: String,
    pub canvas: web_sys::HtmlCanvasElement,
    pub canvas_context: Rc<web_sys::CanvasRenderingContext2d>,

    pub cache_canvas: web_sys::HtmlCanvasElement,
    pub cache_context: Rc<web_sys::CanvasRenderingContext2d>,
}

#[wasm_bindgen]
#[derive(Debug,Copy, Clone)]
pub struct ImgScaleRatio {
    pub dx: f64,
    pub dy: f64,
    pub scale: f64,
}

impl Default for ImgScaleRatio {
    fn default() -> Self {
        ImgScaleRatio {
            dx: 0.0,
            dy: 0.0,
            scale: 1.0,
        }
    }
}

impl ImgScaleRatio {
    fn calculate(c_width: u32, c_height: u32, img_width: u32, img_height: u32) -> ImgScaleRatio {
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
        ImgScaleRatio {
            dx,
            dy,
            scale,
        }
    }
}


#[wasm_bindgen]
pub struct FyCanvas {
    pub width: u32,
    pub height: u32,
    pub img_scale: ImgScaleRatio,
    ctx: FyCanvasCtx,
}

impl FyCanvasCtx {
    pub fn new(id: &str, width: u32, height: u32) -> Result<FyCanvasCtx, JsValue> {
        let document = get_document()?;
        let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;

        canvas.set_width(width);
        canvas.set_height(height);
        canvas.style().set_property("border", "solid")?;

        let canvas_context = canvas.get_context("2d")?
            .ok_or(JsError::new("document not find"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;


        let cache_canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
        cache_canvas.set_width(width);
        cache_canvas.set_height(height);

        let cache_context = cache_canvas.get_context("2d")?
            .ok_or(JsError::new("document not find"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;


        Ok(FyCanvasCtx {
            canvas_id: id.to_string(),
            canvas,
            canvas_context: Rc::new(canvas_context),
            cache_canvas,
            cache_context: Rc::new(cache_context),
        })
    }
}


#[wasm_bindgen]
impl FyCanvas {
    pub fn new(id: &str, width: u32, height: u32) -> Result<FyCanvas, JsValue> {
        let ctx = FyCanvasCtx::new(id, width, height)?;

        Ok(FyCanvas {
            width,
            height,
            ctx,
            img_scale: Default::default(),
        })
    }

    pub fn mount_ui(&self) -> Result<(), JsValue> {
        let document = get_document()?;

        document.body()
            .ok_or(JsError::new("body not find"))?
            .append_child(&self.ctx.canvas)?;
        Ok(())
    }

    pub fn bind_file_input(&self, input_id: &str) -> Result<(), JsValue> {
        let document = get_document()?;
        let input = document.get_element_by_id(input_id)
            .ok_or(JsError::new("body not find"))?
            .dyn_into::<web_sys::HtmlInputElement>()?;

        let file_reader = web_sys::FileReader::new()?;
        let img = web_sys::HtmlImageElement::new()?;
        let canvas_ctx = self.ctx.canvas_context.clone();
        let cache_ctx = self.ctx.cache_context.clone();
        let width = self.width ;
        let height = self.height ;

        let closure_image = Closure::wrap(Box::new(move |event: web_sys::Event| {
            log(&format!("--> closure_image, event: {:?}", event));
            log(&format!("--> closure_image, type: {:?}", event.type_()));
            log(&format!("--> closure_image, target: {:?}", event.target()));

            let ele_image = event.target().unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();
            log(&format!("--> closure_image, target2: {:?}", ele_image));

            let img_width = ele_image.width();
            let img_height = ele_image.height();

            log(&format!("--> closure_image, img width: {:?}", img_width));
            log(&format!("--> closure_image, img height: {:?}", img_height));

            let ratio = ImgScaleRatio::calculate(width,height,img_width,img_height);

            log(&format!("--> closure_image, ratio: {:?}", ratio));


            canvas_ctx. draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&ele_image,
                            0.0,0.0, img_width as f64, img_height as f64,
                           ratio.dx, ratio.dy, img_width as f64 * ratio.scale, img_height as f64 * ratio.scale)
                .unwrap();
        }) as Box<dyn FnMut(_)>);
        img.set_onload(Some(closure_image.as_ref().unchecked_ref()));
        closure_image.forget();


        let closure_reader = Closure::wrap(Box::new(move |event: web_sys::Event| {
            log(&format!("--> closure_reader, event: {:?}", event));
            log(&format!("--> closure_reader, type: {:?}", event.type_()));
            log(&format!("--> closure_reader, target: {:?}", event.target()));

            let ele_reader = event.target().unwrap().dyn_into::<web_sys::FileReader>().unwrap();
            log(&format!("--> closure_reader, target2: {:?}", ele_reader));

            img.set_src(ele_reader.result().unwrap().as_string().unwrap().as_str());
        }) as Box<dyn FnMut(_)>);
        file_reader.set_onload(Some(closure_reader.as_ref().unchecked_ref()));
        closure_reader.forget();


        log(&format!("input: {:?}", input));

        let closure_input = Closure::wrap(Box::new(move |event: web_sys::Event| {
            log(&format!("--> closure_input, event: {:?}", event));
            log(&format!("--> closure_input, type: {:?}", event.type_()));
            log(&format!("--> closure_input, target: {:?}", event.target()));

            let ele_input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            log(&format!("--> closure_input, target2: {:?}", ele_input));

            let file = ele_input.files().unwrap().get(0).unwrap();
            file_reader.read_as_data_url(&file).unwrap();
        }) as Box<dyn FnMut(_)>);

        input.add_event_listener_with_callback("change", closure_input.as_ref().unchecked_ref())?;

        closure_input.forget();

        Ok(())
    }


    pub fn paint(&self) -> Result<(), JsValue> {
        self.ctx.canvas_context.stroke_rect(10.0, 20.0, 100.0, 100.0);
        Ok(())
    }
}


pub fn draw_rect() -> Result<(), JsValue> {
    let document = get_document()?;
    let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(400);
    canvas.set_height(400);
    canvas.style().set_property("border", "solid")?;

    document.body().unwrap().append_child(&canvas)?;


    Ok(())
}

pub fn get_document() -> Result<web_sys::Document, JsValue> {
    let document = web_sys::window()
        .ok_or(JsError::new("windows not find"))?.document()
        .ok_or(JsError::new("document not find"))?;

    Ok(document)
}