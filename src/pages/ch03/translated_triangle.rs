use js_sys::Float32Array;
use wasm_bindgen::JsError;
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram};

#[yew::function_component(TranslatedTriangle)]
pub fn translated_triangle() -> yew::Html {
    let canvas = yew::use_node_ref();
    crate::utils::use_webgl2_canvas_render(canvas.clone(), render);

    yew::html! {
        <canvas ref={canvas} width="400" height="400" />
    }
}

const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
uniform vec4 u_Translation;
void main() {
    gl_Position = a_Position + u_Translation;
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

// The translation distance for x, y, and z direction
const TX: f32 = 0.5;
const TY: f32 = 0.5;
const TZ: f32 = 0.0;

fn render(gl: GL) -> Result<(), JsError> {
    let vert_shader = crate::utils::compile_shader(&gl, GL::VERTEX_SHADER, VSHADER_SOURCE)?;
    let frag_shader = crate::utils::compile_shader(&gl, GL::FRAGMENT_SHADER, FSHADER_SOURCE)?;
    let program = crate::utils::link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    // Write the positions of vertices to a vertex shader
    init_vertex_buffers(&gl, &program)?;

    // Pass the translation distance to the vertex shader
    let u_translation = gl.get_uniform_location(&program, "u_Translation");
    if u_translation.is_none() {
        return Err(JsError::new(
            "Failed to get the storage location of u_Translation",
        ));
    }
    gl.uniform4f(u_translation.as_ref(), TX, TY, TZ, 0.0);

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
