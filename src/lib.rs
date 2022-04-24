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


#[wasm_bindgen]
pub fn draw_rect() -> Result<(),JsValue> {

    let document  = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(400);
    canvas.set_height(400);
    canvas.style().set_property("border","solid")?;

    document.body().unwrap().append_child(&canvas)?;


    Ok(())
}