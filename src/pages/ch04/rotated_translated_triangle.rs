use js_sys::Float32Array;
use nalgebra as na;
use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlUniformLocation,
};
use yew::NodeRef;

use crate::utils::WebGl2RenderingContextExt;

const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
uniform mat4 u_ModelMatrix;
void main() {
    gl_Position = u_ModelMatrix * a_Position;
}
";

const FSHADER_SOURCE: &str = "
void main() {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
";

// The number of vertices
const N: i32 = 3;

const VERTICES: &[f32] = &[0.0, 0.3, -0.3, -0.3, 0.3, -0.3];

// The rotation angle
const ANGLE: f32 = 60.0;
// Convert to radians
const RADIAN: f32 = std::f32::consts::PI * ANGLE / 180.0;
// Translation distance
const TX: f32 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    RotatedTranslated,
    TranslatedRotated,
}

pub enum Message {
    ChangeMode(Mode),
}

pub struct Page {
    gl: Option<GL>,
    canvas: NodeRef,
    mode: Mode,
    u_model_matrix: Option<WebGlUniformLocation>,

    onclick_rotated_translated: yew::Callback<web_sys::MouseEvent>,
    onclick_translated_rotated: yew::Callback<web_sys::MouseEvent>,
}

impl Page {
    fn get_canvas(&self) -> Option<HtmlCanvasElement> {
        self.canvas.cast::<HtmlCanvasElement>()
    }

    fn setup_gl(&mut self) -> Result<(), JsValue> {
        let canvas = self.get_canvas().unwrap();

        let gl = canvas
            .get_context("webgl2")
            .unwrap_throw()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap();

        let program = gl.init_shaders(VSHADER_SOURCE, FSHADER_SOURCE)?;

        init_vertex_buffers(&gl, &program)?;

        let u_model_matrix = gl.get_uniform_location(&program, "u_ModelMatrix");
        if u_model_matrix.is_none() {
            return Err(JsError::new("Failed to get the storage location of u_ModelMatrix").into());
        }

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        self.u_model_matrix = u_model_matrix;
        self.rerender_triangle(&gl);
        self.gl = Some(gl);
        Ok(())
    }

    fn rerender_triangle(&self, gl: &GL) {
        let rotation = na::Matrix4::new_rotation(na::Vector3::new(0.0, 0.0, RADIAN));
        let translation = na::Matrix4::new_translation(&na::Vector3::new(TX, 0.0, 0.0));
        let model_matrix = match self.mode {
            Mode::RotatedTranslated => rotation * translation,
            Mode::TranslatedRotated => translation * rotation,
        };
        // Pass the rotation matrix to the vertex shader
        gl.uniform_matrix4fv_with_f32_array(
            self.u_model_matrix.as_ref(),
            false,
            model_matrix.as_slice(),
        );

        // Clear <canvas>
        gl.clear(GL::COLOR_BUFFER_BIT);

        // Draw
        gl.draw_arrays(GL::TRIANGLES, 0, N);
    }
}

impl yew::Component for Page {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link();
        let onclick_rotated_translated =
            link.callback(|_| Message::ChangeMode(Mode::RotatedTranslated));
        let onclick_translated_rotated =
            link.callback(|_| Message::ChangeMode(Mode::TranslatedRotated));
        Self {
            gl: None,
            canvas: NodeRef::default(),
            mode: Mode::RotatedTranslated,
            u_model_matrix: None,

            onclick_rotated_translated,
            onclick_translated_rotated,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ChangeMode(mode) => {
                self.mode = mode;
                if let Some(gl) = self.gl.as_ref() {
                    self.rerender_triangle(gl);
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <div>
                <canvas
                    ref={self.canvas.clone()}
                    width="400"
                    height="400"
                />
                <p>
                    <button
                        onclick={self.onclick_rotated_translated.clone()}
                        disabled={self.mode == Mode::RotatedTranslated}
                    >{ "rotation -> translation" }</button>
                    <button
                        onclick={self.onclick_translated_rotated.clone()}
                        disabled={self.mode == Mode::TranslatedRotated}
                    >{ "translation -> rotation" }</button>
                </p>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            self.setup_gl().unwrap_throw();
        }
    }
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
