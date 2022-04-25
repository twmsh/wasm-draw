mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-draw!");
}


pub struct FyCanvasCtx {
    pub canvas_id: String,
    pub canvas: web_sys::HtmlCanvasElement,
    pub canvas_context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
pub struct FyCanvas {
    pub width: u32,
    pub height: u32,
    ctx: FyCanvasCtx,
}

impl FyCanvasCtx {
    pub fn new(id: &str, width: u32, height: u32) -> Result<FyCanvasCtx, JsValue> {
        let document = web_sys::window()
            .ok_or(JsError::new("windows not find"))?.document()
            .ok_or(JsError::new("document not find"))?;
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
            canvas_context,
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
        let document = web_sys::window()
            .ok_or(JsError::new("windows not find"))?.document()
            .ok_or(JsError::new("document not find"))?;

        document.body()
            .ok_or(JsError::new("body not find"))?
            .append_child(&self.ctx.canvas)?;
        Ok(())
    }


    pub fn paint(&self) -> Result<(), JsValue> {
        self.ctx.canvas_context.stroke_rect(10.0, 20.0, 100.0, 100.0);
        Ok(())
    }
}


pub fn draw_rect() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(400);
    canvas.set_height(400);
    canvas.style().set_property("border", "solid")?;

    document.body().unwrap().append_child(&canvas)?;


    Ok(())
}