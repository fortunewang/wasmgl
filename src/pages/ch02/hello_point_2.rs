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
attribute vec4 a_Position;
void main() {
    gl_Position = a_Position;
    gl_PointSize = 10.0;
}
";

const FSHADER_SOURCE: &str = "
void main() {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
";

fn render(gl: GL) -> Result<(), JsError> {
    let program = gl.init_shaders(VSHADER_SOURCE, FSHADER_SOURCE)?;

    // Get the storage location of a_Position
    let a_position = gl.get_attrib_location(&program, "a_Position");
    if a_position < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_Position",
        ));
    }

    // Pass vertex position to attribute variable
    gl.vertex_attrib3f(a_position as u32, 0.0, 0.0, 0.0);

    // Specify the color for clearing <canvas>
    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    // Clear <canvas>
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Draw
    gl.draw_arrays(GL::POINTS, 0, 1);
    Ok(())
}
