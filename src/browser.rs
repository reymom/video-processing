use anyhow::{anyhow, Result};
use std::future::Future;
use wasm_bindgen::closure::{Closure, WasmClosure, WasmClosureFnOnce};
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, HtmlElement, HtmlImageElement,
    Window,
};

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

pub fn request_animation_frame(callback: &LoopClosure) -> Result<i32> {
    window()?
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Cannot request animation frame {:#?}", err))
}

pub type LoopClosure = Closure<dyn FnMut(f64)>;

pub fn closure_once<F, A, R>(fn_once: F) -> Closure<F::FnMut>
where
    F: 'static + WasmClosureFnOnce<A, R>,
{
    Closure::once(fn_once)
}

pub fn create_raf_closure(f: impl FnMut(f64) + 'static) -> LoopClosure {
    closure_wrap(Box::new(f))
}

pub fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
}

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub fn now() -> Result<f64> {
    Ok(window()?
        .performance()
        .ok_or_else(|| anyhow!("Performance object not found"))?
        .now())
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

pub fn find_html_element_by_id(id: &str) -> Result<HtmlElement> {
    document()
        .and_then(|doc| {
            doc.get_element_by_id(id)
                .ok_or_else(|| anyhow!("Element with id {} not found", id))
        })
        .and_then(|element| {
            element
                .dyn_into::<HtmlElement>()
                .map_err(|err| anyhow!("Could not cast into HtmlElement {:#?}", err))
        })
}

pub fn draw_ui(html: &str) -> Result<()> {
    find_ui()?
        .insert_adjacent_html("afterbegin", html)
        .map_err(|err| anyhow!("Could not insert html {:#?}", err))
}

fn find_ui() -> Result<Element> {
    document().and_then(|doc| {
        doc.get_element_by_id("ui")
            .ok_or_else(|| anyhow!("UI element not found"))
    })
}
