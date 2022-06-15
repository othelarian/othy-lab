use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main_js(canvas: HtmlCanvasElement) {
  //
  // TODO
  //
  if let Ok(some_ctx) = canvas.get_context("2d") {
    if let Some(ctx) = some_ctx {
      //
      // TODO: faire joujou avec le contexte ici
      //
      let ctx = ctx.unchecked_into::<CanvasRenderingContext2d>();
      //
      //
      ctx.set_fill_style(&JsValue::from("red"));
      ctx.fill_rect(10.0, 10.0, 50.0, 50.0);
      //
    }
  }
}

