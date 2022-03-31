use js_sys::Float32Array;
use wasm_bindgen::JsError;
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram};

#[yew::function_component(RotatedTriangle)]
pub fn rotated_triangle() -> yew::Html {
    let canvas = yew::use_node_ref();
    crate::utils::use_webgl2_canvas_render(canvas.clone(), render);

    yew::html! {
        <canvas ref={canvas} width="400" height="400" />
    }
}

// x' = x cosβ - y sinβ
// y' = x sinβ + y cosβ　Equation 3.3
// z' = z
const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
uniform float u_CosB, u_SinB;
void main() {
    gl_Position.x = a_Position.x * u_CosB - a_Position.y * u_SinB;
    gl_Position.y = a_Position.x * u_SinB + a_Position.y * u_CosB;
    gl_Position.z = a_Position.z;
    gl_Position.w = 1.0;
}
";

const FSHADER_SOURCE: &str = "
void main() {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
";

// The number of vertices
const N: i32 = 3;

const VERTICES: &[f32] = &[0.0, 0.5, -0.5, -0.5, 0.5, -0.5];

// The rotation angle
const ANGLE: f32 = 90.0;
// Convert to radians
const RADIAN: f32 = std::f32::consts::PI * ANGLE / 180.0;

fn render(gl: GL) -> Result<(), JsError> {
    let vert_shader = crate::utils::compile_shader(&gl, GL::VERTEX_SHADER, VSHADER_SOURCE)?;
    let frag_shader = crate::utils::compile_shader(&gl, GL::FRAGMENT_SHADER, FSHADER_SOURCE)?;
    let program = crate::utils::link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    // Write the positions of vertices to a vertex shader
    init_vertex_buffers(&gl, &program)?;

    // Pass the data required to rotate the shape to the vertex shader

    let cos_b = RADIAN.cos();
    let sin_b = RADIAN.sin();

    let u_cos_b = gl.get_uniform_location(&program, "u_CosB");
    let u_sin_b = gl.get_uniform_location(&program, "u_SinB");
    if u_cos_b.is_none() || u_sin_b.is_none() {
        return Err(JsError::new("Failed to get the storage location of u_CosB or u_SinB").into());
    }
    gl.uniform1f(u_cos_b.as_ref(), cos_b);
    gl.uniform1f(u_sin_b.as_ref(), sin_b);

    // Specify the color for clearing <canvas>
    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    // Clear <canvas>
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Draw
    gl.draw_arrays(GL::TRIANGLES, 0, N);
    Ok(())
}

fn init_vertex_buffers(gl: &GL, program: &WebGlProgram) -> Result<(), JsError> {
    let vertices = Float32Array::from(VERTICES);
    let vertex_buffer = gl.create_buffer();
    if vertex_buffer.is_none() {
        return Err(JsError::new("Failed to create the buffer object"));
    }
    gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref());
    gl.buffer_data_with_opt_array_buffer(
        GL::ARRAY_BUFFER,
        Some(&vertices.buffer()),
        GL::STATIC_DRAW,
    );

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
