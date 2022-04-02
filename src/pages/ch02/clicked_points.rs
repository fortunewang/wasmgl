use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{Event, HtmlCanvasElement, MouseEvent, WebGl2RenderingContext as GL};
use yew::NodeRef;

use crate::utils::WebGl2RenderingContextExt;

const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
void main() {
    gl_Position = a_Position;
    gl_PointSize = 10.0;
}
";

const FSHADER_SOURCE: &str = "
void main() {
  gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
";

pub enum Message {
    Click(f32, f32),
}

fn on_click(ev: MouseEvent) -> Message {
    let x = ev.client_x() as f32;
    let y = ev.client_y() as f32;
    let rect = AsRef::<Event>::as_ref(&ev)
        .target()
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap()
        .get_bounding_client_rect();

    Message::Click(x - rect.left() as f32, y - rect.top() as f32)
}

pub struct Page {
    gl: Option<GL>,
    canvas: NodeRef,
    a_position: i32,
    points: Vec<(f32, f32)>,
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

        // Get the storage location of a_Position
        let a_position = gl.get_attrib_location(&program, "a_Position");
        if a_position < 0 {
            return Err(JsError::new("Failed to get the storage location of a_Position").into());
        }

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        // Clear <canvas>
        gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl = Some(gl);
        self.a_position = a_position;
        Ok(())
    }

    fn on_click(&mut self, x: f32, y: f32) -> bool {
        if let Some(gl) = self.gl.as_ref() {
            let canvas = self.get_canvas().unwrap();

            let half_width = (canvas.width() as f32) / 2.0;
            let half_height = (canvas.height() as f32) / 2.0;

            let x = (x - half_width) / half_width;
            let y = (half_height - y) / half_height;
            self.points.push((x, y));

            // Clear <canvas>
            gl.clear(GL::COLOR_BUFFER_BIT);

            for (x, y) in self.points.iter() {
                // Pass the position of a point to a_Position variable
                gl.vertex_attrib3f(self.a_position as u32, *x, *y, 0.0);

                // Draw
                gl.draw_arrays(GL::POINTS, 0, 1);
            }
        }
        false
    }
}

impl yew::Component for Page {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            gl: None,
            canvas: NodeRef::default(),
            a_position: -1,
            points: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Click(x, y) => self.on_click(x, y),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let onclick = ctx.link().callback(on_click);
        yew::html! {
            <canvas
              ref={self.canvas.clone()}
              {onclick}
              width="400"
              height="400"
            />
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            self.setup_gl().unwrap_throw();
        }
    }
}
