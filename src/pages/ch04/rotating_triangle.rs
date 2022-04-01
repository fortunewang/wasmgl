use gloo::render::{request_animation_frame, AnimationFrame};
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

const VERTICES: &[f32] = &[0.0, 0.5, -0.5, -0.5, 0.5, -0.5];

// Rotation angle (degrees/second)
const ANGLE_STEP: f32 = 45.0;
// Convert to radians
const RADIAN_STEP: f32 = std::f32::consts::PI * ANGLE_STEP / 180.0;

pub enum Message {
    Animate(f64),
}

pub struct RotatingTriangle {
    gl: Option<GL>,
    canvas: NodeRef,
    tick: Option<AnimationFrame>,
    u_model_matrix: Option<WebGlUniformLocation>,
    last_radian: f32,
    last_render: Option<f64>,
}

impl RotatingTriangle {
    fn get_canvas(&self) -> Option<HtmlCanvasElement> {
        self.canvas.cast::<HtmlCanvasElement>()
    }

    fn setup_gl(&mut self, link: yew::html::Scope<Self>) -> Result<(), JsValue> {
        let canvas = self.get_canvas().unwrap();

        let gl = canvas
            .get_context("webgl2")
            .unwrap_throw()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap();

        let program = gl.init_shaders(VSHADER_SOURCE, FSHADER_SOURCE)?;

        // Write the positions of vertices to a vertex shader
        init_vertex_buffers(&gl, &program)?;

        // Pass the rotation matrix to the vertex shader
        let u_model_matrix = gl.get_uniform_location(&program, "u_ModelMatrix");
        if u_model_matrix.is_none() {
            return Err(JsError::new("Failed to get the storage location of u_ModelMatrix").into());
        }

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        self.u_model_matrix = u_model_matrix;
        self.gl = Some(gl);
        self.reset_tick(link);

        Ok(())
    }

    fn reset_tick(&mut self, link: yew::html::Scope<Self>) {
        // A reference to the new handle must be retained for the next render to run.
        self.tick = Some(request_animation_frame(move |now| {
            link.send_message(Message::Animate(now))
        }));
    }

    fn animate(&mut self, link: yew::html::Scope<Self>, now: f64) {
        if let Some(gl) = self.gl.as_ref() {
            let elapsed = self
                .last_render
                .map(|last_render| now - last_render)
                .unwrap_or(0.0);
            self.last_render = Some(now);

            let radian = self.last_radian + (RADIAN_STEP * elapsed as f32) / 1000.0;
            self.last_radian = radian % std::f32::consts::TAU;

            let model_matrix =
                na::Matrix4::new_rotation(na::Vector3::new(0.0, 0.0, self.last_radian));
            gl.uniform_matrix4fv_with_f32_array(
                self.u_model_matrix.as_ref(),
                false,
                model_matrix.as_slice(),
            );

            // Clear <canvas>
            gl.clear(GL::COLOR_BUFFER_BIT);

            // Draw
            gl.draw_arrays(GL::TRIANGLES, 0, N);

            self.reset_tick(link);
        } else {
            self.tick = None;
        }
    }
}

impl yew::Component for RotatingTriangle {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            gl: None,
            canvas: NodeRef::default(),
            tick: None,
            u_model_matrix: None,
            last_radian: 0.0,
            last_render: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Animate(now) => {
                self.animate(ctx.link().clone(), now);
                false
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
            </div>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            self.setup_gl(ctx.link().clone()).unwrap_throw();
        }
    }
}

fn init_vertex_buffers(gl: &GL, program: &WebGlProgram) -> Result<(), JsError> {
    let vertices = Float32Array::from(VERTICES);
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
