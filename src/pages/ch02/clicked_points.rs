use wasm_bindgen::{JsCast, JsError, JsValue};
use web_sys::{Event, HtmlCanvasElement, MouseEvent, WebGl2RenderingContext as GL};
use yew::NodeRef;

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

pub enum ClickedPointsMessage {
    SetupGL,
    Click(f32, f32),
}

fn on_click(ev: MouseEvent) -> ClickedPointsMessage {
    let x = ev.client_x() as f32;
    let y = ev.client_y() as f32;
    let rect = AsRef::<Event>::as_ref(&ev)
        .target()
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap()
        .get_bounding_client_rect();

    ClickedPointsMessage::Click(x - rect.left() as f32, y - rect.top() as f32)
}

pub struct ClickedPoints {
    gl: Option<GL>,
    canvas: NodeRef,
    a_position: i32,
    points: Vec<(f32, f32)>,
}

impl ClickedPoints {
    fn get_canvas(&self) -> Option<HtmlCanvasElement> {
        self.canvas.cast::<HtmlCanvasElement>()
    }

    fn setup_gl(&mut self) -> Result<bool, JsValue> {
        let canvas = self.get_canvas().unwrap();

        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap();

        let vert_shader = crate::utils::compile_shader(&gl, GL::VERTEX_SHADER, VSHADER_SOURCE)?;
        let frag_shader = crate::utils::compile_shader(&gl, GL::FRAGMENT_SHADER, FSHADER_SOURCE)?;
        let program = crate::utils::link_program(&gl, &vert_shader, &frag_shader)?;
        gl.use_program(Some(&program));

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
        Ok(true)
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

impl yew::Component for ClickedPoints {
    type Message = ClickedPointsMessage;
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
            ClickedPointsMessage::SetupGL => self.setup_gl().unwrap(),
            ClickedPointsMessage::Click(x, y) => self.on_click(x, y),
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

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(ClickedPointsMessage::SetupGL);
        }
    }
}
