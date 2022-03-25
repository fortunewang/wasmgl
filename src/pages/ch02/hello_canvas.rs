use wasm_bindgen::JsError;
use web_sys::WebGl2RenderingContext as GL;

#[yew::function_component(HelloCanvas)]
pub fn hello_canvas() -> yew::Html {
    let canvas = yew::use_node_ref();
    crate::utils::use_webgl2_canvas_render(canvas.clone(), render);

    yew::html! {
        <canvas ref={canvas} width="400" height="400" />
    }
}

fn render(gl: GL) -> Result<(), JsError> {
    // 指定清空<canvas>的颜色
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    // 清空<canvas>
    gl.clear(GL::COLOR_BUFFER_BIT);
    Ok(())
}
