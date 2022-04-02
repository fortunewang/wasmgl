use gloo::render::AnimationFrame;
use js_sys::Float32Array;
use nalgebra as na;
use wasm_bindgen::{prelude::Closure, JsCast, JsError, JsValue, UnwrapThrowExt};
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
const ANGLE_STEP_STEP: f32 = 10.0;
// Convert to radians
const RADIAN_STEP: f32 = std::f32::consts::PI * ANGLE_STEP / 180.0;
const RADIAN_STEP_STEP: f32 = std::f32::consts::PI * ANGLE_STEP_STEP / 180.0;

pub enum Message {
    Animate(f64),
    SpeedUp,
    SpeedDown,
}

pub struct Page {
    gl: Option<GL>,
    canvas: NodeRef,
    tick: Option<AnimationFrame>,
    u_model_matrix: Option<WebGlUniformLocation>,
    step: f32,
    last_radian: f32,
    last_render: Option<f64>,

    onclick_speed_up: yew::Callback<web_sys::MouseEvent>,
    onclick_speed_down: yew::Callback<web_sys::MouseEvent>,
}

impl Page {
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
        let on_animate = move |now: f64| {
            link.send_message(Message::Animate(now));
        };

        let closure = Closure::wrap(Box::new(on_animate) as Box<dyn FnMut(_)>);
        gloo::utils::window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        // A reference to the new handle must be retained for the next render to run.
        // self.tick = Some(gloo::render::request_animation_frame(on_animate));
    }

    fn animate(&mut self, link: yew::html::Scope<Self>, now: f64) {
        if let Some(gl) = self.gl.as_ref() {
            let elapsed = self
                .last_render
                .map(|last_render| now - last_render)
                .unwrap_or(0.0);
            self.last_render = Some(now);

            let radian = self.last_radian + (self.step * elapsed as f32) / 1000.0;
            self.last_radian = radian % std::f32::consts::TAU;

            let model_matrix =
                na::Matrix4::new_rotation(na::Vector3::new(0.0, 0.0, self.last_radian))
                    * na::Matrix4::new_translation(&na::Vector3::new(0.35f32, 0.0, 0.0));
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

impl yew::Component for Page {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link();
        let onclick_speed_up = link.callback(|_| Message::SpeedUp);
        let onclick_speed_down = link.callback(|_| Message::SpeedDown);
        Self {
            gl: None,
            canvas: NodeRef::default(),
            tick: None,
            step: RADIAN_STEP,
            u_model_matrix: None,
            last_radian: 0.0,
            last_render: None,

            onclick_speed_up,
            onclick_speed_down,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Animate(now) => {
                self.animate(ctx.link().clone(), now);
                false
            }
            Message::SpeedUp => {
                if self.step < std::f32::consts::TAU {
                    self.step += RADIAN_STEP_STEP;
                }
                false
            }
            Message::SpeedDown => {
                if self.step > RADIAN_STEP_STEP {
                    self.step -= RADIAN_STEP_STEP;
                }
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
                <p>
                    <button
                        onclick={self.onclick_speed_up.clone()}
                    >{ "UP" }</button>
                    <button
                        onclick={self.onclick_speed_down.clone()}
                    >{ "DOWN" }</button>
                </p>
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
