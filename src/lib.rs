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
}

#[wasm_bindgen]
pub struct FyCanvas {
    pub width: u32,
    pub height: u32,
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

        Ok(FyCanvasCtx {
            canvas_id: id.to_string(),
            canvas,
            canvas_context: Rc::new(canvas_context),
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
        })
    }

    pub fn mount_ui(&self) -> Result<(), JsValue> {
        let document = get_document()?;

        document.body()
            .ok_or(JsError::new("body not find"))?
            .append_child(&self.ctx.canvas)?;
        Ok(())
    }

    pub fn bind_file_input(&self, input_id: &str) -> Result<(),JsValue> {
        let document = get_document()?;
        let input = document.get_element_by_id(input_id)
            .ok_or(JsError::new("body not find"))?
            .dyn_into::<web_sys::HtmlInputElement>()?;

        let file_reader = web_sys::FileReader::new()?;
        let img = web_sys::HtmlImageElement::new()?;
        let canvas_ctx_cl = self.ctx.canvas_context.clone();
        let width = self.width as f64;
        let height = self.height as f64;

        let closure_image = Closure::wrap(Box::new(move |event: web_sys::Event| {

            log(&format!("--> closure_image, event: {:?}",event));
            log(&format!("--> closure_image, type: {:?}",event.type_()));
            log(&format!("--> closure_image, target: {:?}",event.target()));

            let ele_image = event.target().unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();
            log(&format!("--> closure_image, target2: {:?}", ele_image));


            let document = get_document().unwrap();

            document.body().unwrap()
                .append_child(&ele_image).unwrap();

            canvas_ctx_cl.draw_image_with_html_image_element_and_dw_and_dh(&ele_image,0.0,0.0,width,height)
                .unwrap();


        }) as Box<dyn FnMut(_)>);
        img.set_onload(Some(closure_image.as_ref().unchecked_ref()));
        closure_image.forget();


        let closure_reader = Closure::wrap(Box::new(move |event: web_sys::Event| {

            log(&format!("--> closure_reader, event: {:?}",event));
            log(&format!("--> closure_reader, type: {:?}",event.type_()));
            log(&format!("--> closure_reader, target: {:?}",event.target()));

            let ele_reader = event.target().unwrap().dyn_into::<web_sys::FileReader>().unwrap();
            log(&format!("--> closure_reader, target2: {:?}", ele_reader));

            img.set_src(ele_reader.result().unwrap().as_string().unwrap().as_str());


        }) as Box<dyn FnMut(_)>);
        file_reader.set_onload(Some(closure_reader.as_ref().unchecked_ref()));
        closure_reader.forget();



        log(&format!("input: {:?}",input));

        let closure_input = Closure::wrap(Box::new(move |event: web_sys::Event|{
            log(&format!("--> closure_input, event: {:?}",event));
            log(&format!("--> closure_input, type: {:?}",event.type_()));
            log(&format!("--> closure_input, target: {:?}",event.target()));

            let ele_input = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            log(&format!("--> closure_input, target2: {:?}", ele_input));

            let file = ele_input.files().unwrap().get(0).unwrap();
            file_reader.read_as_data_url(&file).unwrap();


        }) as Box<dyn FnMut(_)>);

        input.add_event_listener_with_callback("change",closure_input.as_ref().unchecked_ref())?;

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

pub fn get_document() -> Result<web_sys::Document,JsValue> {
    let document = web_sys::window()
        .ok_or(JsError::new("windows not find"))?.document()
        .ok_or(JsError::new("document not find"))?;

    Ok(document)
}