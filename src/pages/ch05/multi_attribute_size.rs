use js_sys::Float32Array;
use wasm_bindgen::JsError;
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram};

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
attribute float a_PointSize;
void main() {
    gl_Position = a_Position;
    gl_PointSize = a_PointSize;
}
";

const FSHADER_SOURCE: &str = "
void main() {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
";

const N: i32 = 3;

const VERTICES: &[f32] = &[0.0, 0.5, -0.5, -0.5, 0.5, -0.5];

const SIZES: &[f32] = &[10.0, 20.0, 30.0];

fn render(gl: GL) -> Result<(), JsError> {
    let program = gl.init_shaders(VSHADER_SOURCE, FSHADER_SOURCE)?;

    init_vertex_buffers(&gl, &program)?;
    init_size_buffers(&gl, &program)?;

    // Unbind the buffer object
    gl.bind_buffer(GL::ARRAY_BUFFER, None);

    // Specify the color for clearing <canvas>
    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    // Clear <canvas>
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Draw
    gl.draw_arrays(GL::POINTS, 0, N);
    Ok(())
}

fn init_vertex_buffers(gl: &GL, program: &WebGlProgram) -> Result<(), JsError> {
    // let vertices = Float32Array::from(VERTICES);
    // use view() instead of from() to avoid additional memory allocation
    let vertices = unsafe { Float32Array::view(VERTICES) };
    let vertex_buffer = gl.create_buffer();
    if vertex_buffer.is_none() {
        return Err(JsError::new("Failed to create the buffer object"));
    }
    gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref());
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices, GL::STATIC_DRAW);

    let a_position = gl.get_attrib_location(program, "a_Position");
    if a_position < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_Position",
        ));
    }

    gl.vertex_attrib_pointer_with_i32(a_position as u32, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(a_position as u32);

    Ok(())
}

fn init_size_buffers(gl: &GL, program: &WebGlProgram) -> Result<(), JsError> {
    // let sizes = Float32Array::from(SIZES);
    // use view() instead of from() to avoid additional memory allocation
    let sizes = unsafe { Float32Array::view(SIZES) };
    let size_buffer = gl.create_buffer();
    if size_buffer.is_none() {
        return Err(JsError::new("Failed to create the buffer object"));
    }
    gl.bind_buffer(GL::ARRAY_BUFFER, size_buffer.as_ref());
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &sizes, GL::STATIC_DRAW);

    let a_point_size = gl.get_attrib_location(program, "a_PointSize");
    if a_point_size < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_PointSize",
        ));
    }

    gl.vertex_attrib_pointer_with_i32(a_point_size as u32, 1, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(a_point_size as u32);

    Ok(())
}
