use js_sys::Float32Array;
use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram};
use yew::NodeRef;

use crate::utils::WebGl2RenderingContextExt;

const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
attribute vec4 a_Color;
varying vec4 v_Color;
void main() {
    gl_Position = a_Position;
    gl_PointSize = 10.0;
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

// The number of vertices
const N: i32 = 3;

// Vertex coordinates and color
const VERTICES_COLORS: &[f32] = &[
    0.0, 0.5, 1.0, 0.0, 0.0, // the 1st point
    -0.5, -0.5, 0.0, 1.0, 0.0, // the 2nd point
    0.5, -0.5, 0.0, 0.0, 1.0, // the 3rd point
];

const FSIZE: i32 = std::mem::size_of::<f32>() as i32;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Points = GL::POINTS,
    Triangles = GL::TRIANGLES,
}

pub enum Message {
    ChangeMode(Mode),
}

pub struct Page {
    gl: Option<GL>,
    canvas: NodeRef,
    mode: Mode,

    onclick_points: yew::Callback<web_sys::MouseEvent>,
    onclick_triangles: yew::Callback<web_sys::MouseEvent>,
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

        // Unbind the buffer object
        gl.bind_buffer(GL::ARRAY_BUFFER, None);

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        self.rerender_triangle(&gl);

        self.gl = Some(gl);
        Ok(())
    }

    fn rerender_triangle(&self, gl: &GL) {
        // Clear <canvas>
        gl.clear(GL::COLOR_BUFFER_BIT);

        // Draw
        gl.draw_arrays(self.mode as u32, 0, N);
    }
}

impl yew::Component for Page {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link();
        let onclick_points = link.callback(|_| Message::ChangeMode(Mode::Points));
        let onclick_triangles = link.callback(|_| Message::ChangeMode(Mode::Triangles));
        Self {
            gl: None,
            canvas: NodeRef::default(),
            mode: Mode::Points,

            onclick_points,
            onclick_triangles,
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
                        onclick={self.onclick_points.clone()}
                        disabled={self.mode == Mode::Points}
                    >{ "POINTS" }</button>
                    <button
                        onclick={self.onclick_triangles.clone()}
                        disabled={self.mode == Mode::Triangles}
                    >{ "TRIANGLES" }</button>
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
    // let vertices_colors = Float32Array::from(VERTICES_COLORS);
    // use view() instead of from() to avoid additional memory allocation
    let vertices_colors = unsafe { Float32Array::view(VERTICES_COLORS) };
    let vertex_buffer = gl.create_buffer();
    if vertex_buffer.is_none() {
        return Err(JsError::new("Failed to create the buffer object"));
    }
    gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref());
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_colors, GL::STATIC_DRAW);

    let a_position = gl.get_attrib_location(program, "a_Position");
    if a_position < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_Position",
        ));
    }

    gl.vertex_attrib_pointer_with_i32(a_position as u32, 2, GL::FLOAT, false, FSIZE * 5, 0);
    gl.enable_vertex_attrib_array(a_position as u32);

    let a_color = gl.get_attrib_location(program, "a_Color");
    if a_color < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_Color",
        ));
    }

    gl.vertex_attrib_pointer_with_i32(a_color as u32, 3, GL::FLOAT, false, FSIZE * 5, FSIZE * 2);
    gl.enable_vertex_attrib_array(a_color as u32);

    Ok(())
}
