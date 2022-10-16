use anyhow::{anyhow, Result};
use std::future::Future;
use wasm_bindgen::closure::{Closure, WasmClosureFnOnce};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, Window};

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!( $( $t )*).into());
    };
}

macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )*).into());
    };
}

macro_rules! panic {
    ( $( $t:tt )* ) => {
        web_sys::console::exception_1(&format!( $( $t )*).into());
    };
}

// pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

// pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
//     closure_wrap(Box::new(f))
// }

// pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
//     Closure::wrap(data)
// }

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub fn new_image() -> Result<HtmlImageElement> {
    HtmlImageElement::new().map_err(|e| anyhow!("error creating image: {:#?}", e))
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("no window found :S"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("no document found :S"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("error getting canvas element"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|element| anyhow!("error converting {:#?} to HtmlCanvasElement", element))
}

pub fn context() -> Result<CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|js_value| anyhow!("error getting 2d context {:#?}", js_value))?
        .ok_or_else(|| anyhow!("no 2d context found"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|element| anyhow!("error converting {:#?} to HtmlCanvasElement", element))
}
