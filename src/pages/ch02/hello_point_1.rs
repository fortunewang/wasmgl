use wasm_bindgen::JsError;
use web_sys::WebGl2RenderingContext as GL;

#[yew::function_component(HelloPoint1)]
pub fn hello_point_1() -> yew::Html {
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
    let vert_shader = crate::utils::compile_shader(&gl, GL::VERTEX_SHADER, VSHADER_SOURCE)?;
    let frag_shader = crate::utils::compile_shader(&gl, GL::FRAGMENT_SHADER, FSHADER_SOURCE)?;
    let program = crate::utils::link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    // Specify the color for clearing <canvas>
    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    // Clear <canvas>
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Draw a point
    gl.draw_arrays(GL::POINTS, 0, 1);
    Ok(())
}
