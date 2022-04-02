use js_sys::Float32Array;
use nalgebra as na;
use wasm_bindgen::{prelude::Closure, JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlUniformLocation,
};
use yew::NodeRef;

use crate::utils::{Draging, WebGl2RenderingContextExt};

const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
attribute vec4 a_Color;
uniform mat4 u_ViewMatrix;
varying vec4 v_Color;
void main() {
    gl_Position = u_ViewMatrix * a_Position;
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

// Vertex coordinates and color(RGBA)
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

pub enum Message {
    KeyLeft,
    KeyRight,
    KeyUp,
    KeyDown,
    MouseDown(i32, i32),
    MouseMove(i32, i32),
    MouseUp(i32, i32),
}

pub struct Page {
    gl: Option<GL>,
    canvas: NodeRef,
    u_view_matrix: Option<WebGlUniformLocation>,
    eye_x: f32,
    eye_y: f32,
    draging: Draging,

    original_onkeydown: Option<Option<js_sys::Function>>,
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

        let u_view_matrix = gl.get_uniform_location(&program, "u_ViewMatrix");
        if u_view_matrix.is_none() {
            return Err(JsError::new("Failed to get the storage location of u_ViewMatrix").into());
        }

        // Unbind the buffer object
        gl.bind_buffer(GL::ARRAY_BUFFER, None);

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        self.u_view_matrix = u_view_matrix;
        self.rerender_triangle(&gl);

        self.gl = Some(gl);
        Ok(())
    }

    fn rerender_triangle(&self, gl: &GL) {
        // Set the matrix to be used for to set the camera view
        let view_matrix = na::Matrix4::look_at_rh(
            &na::Point3::new(self.eye_x, self.eye_y, 0.25),
            &na::Point3::new(0.0, 0.0, 0.0),
            &na::Vector3::new(0.0, 1.0, 0.0),
        );

        // Pass the view projection matrix
        gl.uniform_matrix4fv_with_f32_array(
            self.u_view_matrix.as_ref(),
            false,
            view_matrix.as_slice(),
        );

        // Clear <canvas>
        gl.clear(GL::COLOR_BUFFER_BIT);
        // Draw the rectangle
        gl.draw_arrays(GL::TRIANGLES, 0, N);
    }
}

impl yew::Component for Page {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            gl: None,
            canvas: NodeRef::default(),
            u_view_matrix: None,
            eye_x: 0.0,
            eye_y: 0.0,
            draging: Draging::default(),
            original_onkeydown: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::KeyLeft => {
                if let Some(gl) = self.gl.as_ref() {
                    if !self.draging.is_tracking() {
                        self.eye_x -= 0.01;
                        self.rerender_triangle(gl);
                    }
                }
            }
            Message::KeyRight => {
                if let Some(gl) = self.gl.as_ref() {
                    if !self.draging.is_tracking() {
                        self.eye_x += 0.01;
                        self.rerender_triangle(gl);
                    }
                }
            }
            Message::KeyUp => {
                if let Some(gl) = self.gl.as_ref() {
                    if !self.draging.is_tracking() {
                        self.eye_y += 0.01;
                        self.rerender_triangle(gl);
                    }
                }
            }
            Message::KeyDown => {
                if let Some(gl) = self.gl.as_ref() {
                    if !self.draging.is_tracking() {
                        self.eye_y -= 0.01;
                        self.rerender_triangle(gl);
                    }
                }
            }
            Message::MouseDown(x, y) => {
                if self.draging.onmousedown(x, y) {
                    let document = gloo::utils::document();

                    let link = ctx.link().clone();
                    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                        link.send_message(Message::MouseMove(event.client_x(), event.client_y()));
                    }) as Box<dyn FnMut(_)>);
                    document.set_onmousemove(Some(closure.as_ref().unchecked_ref()));
                    closure.forget();

                    let link = ctx.link().clone();
                    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                        link.send_message(Message::MouseUp(event.client_x(), event.client_y()));
                    }) as Box<dyn FnMut(_)>);
                    document.set_onmouseup(Some(closure.as_ref().unchecked_ref()));
                    closure.forget();
                }
            }
            Message::MouseMove(x, y) => {
                if let Some((diff_x, diff_y)) = self.draging.onmousemove(x, y) {
                    let old_eye_x = self.eye_x;
                    let old_eye_y = self.eye_y;
                    self.eye_x += diff_x as f32 / 1000.0;
                    self.eye_y -= diff_y as f32 / 1000.0;
                    if let Some(gl) = self.gl.as_ref() {
                        self.rerender_triangle(gl);
                    }
                    self.eye_x = old_eye_x;
                    self.eye_y = old_eye_y;
                }
            }
            Message::MouseUp(x, y) => {
                if let Some((diff_x, diff_y)) = self.draging.onmouseup(x, y) {
                    self.eye_x += diff_x as f32 / 1000.0;
                    self.eye_y -= diff_y as f32 / 1000.0;
                }
            }
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let onmousedown = ctx.link().callback(|event: web_sys::MouseEvent| {
            Message::MouseDown(event.client_x(), event.client_y())
        });
        yew::html! {
            <canvas
                ref={self.canvas.clone()}
                {onmousedown}
                width="400"
                height="400"
            />
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            self.setup_gl().unwrap_throw();

            let link = ctx.link().clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                match event.key_code() {
                    37 => {
                        link.send_message(Message::KeyLeft);
                    }
                    38 => {
                        link.send_message(Message::KeyUp);
                    }
                    39 => {
                        link.send_message(Message::KeyRight);
                    }
                    40 => {
                        link.send_message(Message::KeyDown);
                    }
                    _ => {}
                }
            }) as Box<dyn FnMut(_)>);
            let document = gloo::utils::document();
            self.original_onkeydown = Some(document.onkeydown());
            document.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
    }

    fn destroy(&mut self, _ctx: &yew::Context<Self>) {
        if let Some(original_onkeydown) = self.original_onkeydown.take() {
            gloo::utils::document().set_onkeydown(original_onkeydown.as_ref());
        }
        self.draging.stop_tracking();
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

    let a_color = gl.get_attrib_location(program, "a_Color");
    if a_color < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_Color",
        ));
    }

    // Assign the buffer object to a_Position and enable the assignment
    gl.vertex_attrib_pointer_with_i32(a_position as u32, 3, GL::FLOAT, false, FSIZE * 6, 0);
    gl.enable_vertex_attrib_array(a_position as u32);
    // Assign the buffer object to a_Color and enable the assignment
    gl.vertex_attrib_pointer_with_i32(a_color as u32, 3, GL::FLOAT, false, FSIZE * 6, FSIZE * 3);
    gl.enable_vertex_attrib_array(a_color as u32);

    Ok(())
}
