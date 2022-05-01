use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::{BgImgInfo, ComponentVec};

pub struct FyRender {
    pub canvas_ctx: web_sys::CanvasRenderingContext2d,
    pub cache_canvas: web_sys::HtmlCanvasElement,
    pub cache_ctx: web_sys::CanvasRenderingContext2d,

    pub select_id: Option<u32>,
    pub mouse_pressed: bool,
}

impl FyRender {
    pub fn new(
        canvas_ctx: web_sys::CanvasRenderingContext2d,
        cache_canvas: web_sys::HtmlCanvasElement,
        cache_ctx: web_sys::CanvasRenderingContext2d,
    ) -> Self {
        Self {
            canvas_ctx,
            cache_canvas,
            cache_ctx,
            select_id: None,
            mouse_pressed: false,
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


    pub fn paint(&self, childs: Rc<RefCell<ComponentVec>>) {
        let width = self.cache_canvas.width() as f64;
        let height = self.cache_canvas.height() as f64;

        self.canvas_ctx.clear_rect(0.0, 0.0, width, height);

        self.canvas_ctx
            .draw_image_with_html_canvas_element(&self.cache_canvas, 0.0, 0.0)
            .unwrap();

        for component in childs.borrow().values() {
            component.paint(&self.canvas_ctx);
        }
    }

    pub fn mouse_down(&mut self, childs: Rc<RefCell<ComponentVec>>, x: i32, y: i32) {
        if self.mouse_pressed {
            // mouse down 之前， pressed应该是false状态
            self.mouse_pressed = false;
            self.select_id = None;
        }

        // 寻找选中的控件，设置成focus，其他控件失去focus
        // 判断 是点中控制点，还是移动区域
        let mut component_list = childs.deref().borrow_mut();
        for (id, component) in component_list.iter_mut() {
            if component.can_select(x, y) {} else {}
        }
    }
}
