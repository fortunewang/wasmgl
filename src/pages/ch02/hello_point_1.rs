use wasm_bindgen::JsError;
use web_sys::WebGl2RenderingContext as GL;

use crate::utils::WebGl2RenderingContextExt;

#[yew::function_component(Page)]
pub fn page() -> yew::Html {
    let canvas = yew::use_node_ref();
    crate::utils::use_webgl2_canvas_render(canvas.clone(), render);

    yew::html! {
        <canvas ref={canvas} width="400" height="400" />
    }
}

const VSHADER_SOURCE: &str = "
void main() {
    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
    gl_PointSize = 10.0;
}
";

const FSHADER_SOURCE: &str = "
void main() {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
";

fn render(gl: GL) -> Result<(), JsError> {
    let _program = gl.init_shaders(VSHADER_SOURCE, FSHADER_SOURCE)?;

    // Specify the color for clearing <canvas>
    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    // Clear <canvas>
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Draw a point
    gl.draw_arrays(GL::POINTS, 0, 1);
    Ok(())
}
