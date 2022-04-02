use js_sys::Float32Array;
use nalgebra as na;
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
attribute vec4 a_Color;
uniform mat4 u_ViewMatrix;
uniform mat4 u_ModelMatrix;
varying vec4 v_Color;
void main() {
    gl_Position = u_ViewMatrix * u_ModelMatrix * a_Position;
    v_Color = a_Color;
}
";

const FSHADER_SOURCE: &str = "
precision mediump float;
varying vec4 v_Color;
void main() {
    gl_FragColor = v_Color;
}
";

const N: i32 = 9;

// Vertex coordinates and color
const VERTICES: &[f32] = &[
    0.0, 0.5, -0.4, 0.4, 1.0, 0.4, // The back green one
    -0.5, -0.5, -0.4, 0.4, 1.0, 0.4, //
    0.5, -0.5, -0.4, 1.0, 0.4, 0.4, //
    0.5, 0.4, -0.2, 1.0, 0.4, 0.4, // The middle yellow one
    -0.5, 0.4, -0.2, 1.0, 1.0, 0.4, //
    0.0, -0.6, -0.2, 1.0, 1.0, 0.4, //
    0.0, 0.5, 0.0, 0.4, 0.4, 1.0, // The front blue one
    -0.5, -0.5, 0.0, 0.4, 0.4, 1.0, //
    0.5, -0.5, 0.0, 1.0, 0.4, 0.4, //
];

const FSIZE: i32 = std::mem::size_of::<f32>() as i32;

fn render(gl: GL) -> Result<(), JsError> {
    let program = gl.init_shaders(VSHADER_SOURCE, FSHADER_SOURCE)?;

    init_vertex_buffers(&gl, &program)?;

    // Unbind the buffer object
    gl.bind_buffer(GL::ARRAY_BUFFER, None);

    // Specify the color for clearing <canvas>
    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    // Clear <canvas>
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Draw
    gl.draw_arrays(GL::TRIANGLES, 0, N);
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

    let a_color = gl.get_attrib_location(program, "a_Color");
    if a_color < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_Color",
        ));
    }

    let u_view_matrix = gl.get_uniform_location(program, "u_ViewMatrix");
    let u_model_matrix = gl.get_uniform_location(program, "u_ModelMatrix");
    if u_view_matrix.is_none() {
        return Err(JsError::new(
            "Failed to get the storage location of u_viewMatrix or u_ModelMatrix",
        ));
    }

    let view_matrix = na::Matrix4::look_at_rh(
        &na::Point3::new(0.20f32, 0.25, 0.25),
        &na::Point3::new(0.0, 0.0, 0.0),
        &na::Vector3::new(0.0, 1.0, 0.0),
    );
    let model_matrix = na::Matrix4::new_rotation(na::Vector3::new(
        0.0,
        0.0,
        -10.0 * std::f32::consts::PI / 180.0,
    ));

    // Assign the buffer object to a_Position and enable the assignment
    gl.vertex_attrib_pointer_with_i32(a_position as u32, 3, GL::FLOAT, false, FSIZE * 6, 0);
    gl.enable_vertex_attrib_array(a_position as u32);
    // Assign the buffer object to a_Color and enable the assignment
    gl.vertex_attrib_pointer_with_i32(a_color as u32, 3, GL::FLOAT, false, FSIZE * 6, FSIZE * 3);
    gl.enable_vertex_attrib_array(a_color as u32);
    // Pass the view projection matrix and model matrix
    gl.uniform_matrix4fv_with_f32_array(u_view_matrix.as_ref(), false, view_matrix.as_slice());
    gl.uniform_matrix4fv_with_f32_array(u_model_matrix.as_ref(), false, model_matrix.as_slice());

    Ok(())
}
