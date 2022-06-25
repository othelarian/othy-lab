use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn render_clock(ctx: CanvasRenderingContext2d) {
  //
  // TODO: c'est ici que l'horloge est dessinée
  //
  ctx.set_fill_style(&JsValue::from("red"));
  ctx.fill_rect(10.0, 10.0, 50.0, 50.0);
  //
  //
}

#[wasm_bindgen]
pub fn main_js(canvas: HtmlCanvasElement) {
  if let Ok(some_ctx) = canvas.get_context("2d") {
    if let Some(ctx) = some_ctx {
      render_clock(ctx.unchecked_into::<CanvasRenderingContext2d>());
    }
  }
}

