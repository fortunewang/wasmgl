use wasm_bindgen::{JsError, JsValue};
use web_sys::CanvasRenderingContext2d;

#[yew::function_component(Page)]
pub fn page() -> yew::Html {
    let canvas = yew::use_node_ref();
    crate::utils::use_2d_canvas_render(canvas.clone(), render);

    yew::html! {
        <canvas ref={canvas} width="400" height="400" />
    }
}

fn render(ctx: CanvasRenderingContext2d) -> Result<(), JsError> {
    ctx.set_fill_style(&JsValue::from_str("rgba(0, 0, 255, 1.0)"));
    ctx.fill_rect(120.0, 10.0, 150.0, 150.0);
    Ok(())
}
