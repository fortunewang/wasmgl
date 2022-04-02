use js_sys::Float32Array;
use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram};
use yew::NodeRef;

use crate::utils::WebGl2RenderingContextExt;

const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
void main() {
    gl_Position = a_Position;
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

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Triangles = GL::TRIANGLES,
    Lines = GL::LINES,
    LineStrip = GL::LINE_STRIP,
    LineLoop = GL::LINE_LOOP,
}

pub enum Message {
    ChangeMode(Mode),
}

pub struct Page {
    gl: Option<GL>,
    canvas: NodeRef,
    mode: Mode,

    onclick_triangles: yew::Callback<web_sys::MouseEvent>,
    onclick_lines: yew::Callback<web_sys::MouseEvent>,
    onclick_line_strip: yew::Callback<web_sys::MouseEvent>,
    onclick_line_loop: yew::Callback<web_sys::MouseEvent>,
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

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        self.rerender_triangle(&gl);

        self.gl = Some(gl);
        Ok(())
    }

    fn rerender_triangle(&self, gl: &GL) {
        // Clear <canvas>
        gl.clear(GL::COLOR_BUFFER_BIT);

        // Draw the rectangle
        gl.draw_arrays(self.mode as u32, 0, N);
    }
}

impl yew::Component for Page {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link();
        let onclick_triangles = link.callback(|_| Message::ChangeMode(Mode::Triangles));
        let onclick_lines = link.callback(|_| Message::ChangeMode(Mode::Lines));
        let onclick_line_strip = link.callback(|_| Message::ChangeMode(Mode::LineStrip));
        let onclick_line_loop = link.callback(|_| Message::ChangeMode(Mode::LineLoop));
        Self {
            gl: None,
            canvas: NodeRef::default(),
            mode: Mode::Triangles,

            onclick_triangles,
            onclick_lines,
            onclick_line_strip,
            onclick_line_loop,
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
                        onclick={self.onclick_triangles.clone()}
                        disabled={self.mode == Mode::Triangles}
                    >{ "TRIANGLES" }</button>
                    <button
                        onclick={self.onclick_lines.clone()}
                        disabled={self.mode == Mode::Lines}
                    >{ "LINES" }</button>
                    <button
                        onclick={self.onclick_line_strip.clone()}
                        disabled={self.mode == Mode::LineStrip}
                    >{ "LINE_STRIP" }</button>
                    <button
                        onclick={self.onclick_line_loop.clone()}
                        disabled={self.mode == Mode::LineLoop}
                    >{ "LINE_LOOP" }</button>
                </p>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            self.setup_gl().unwrap_throw()
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
