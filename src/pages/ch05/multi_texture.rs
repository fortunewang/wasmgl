use js_sys::Float32Array;
use wasm_bindgen::{closure::Closure, JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{
    HtmlCanvasElement, HtmlImageElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlTexture,
    WebGlUniformLocation,
};
use yew::NodeRef;

use crate::utils::WebGl2RenderingContextExt;

const VSHADER_SOURCE: &str = "
attribute vec4 a_Position;
attribute vec2 a_TexCoord;
varying vec2 v_TexCoord;
void main() {
    gl_Position = a_Position;
    v_TexCoord = a_TexCoord;
}
";

const FSHADER_SOURCE: &str = "
precision mediump float;
uniform sampler2D u_Sampler0;
uniform sampler2D u_Sampler1;
varying vec2 v_TexCoord;
void main() {
  vec4 color0 = texture2D(u_Sampler0, v_TexCoord);
  vec4 color1 = texture2D(u_Sampler1, v_TexCoord);
  gl_FragColor = color0 * color1;
}
";

// The number of vertices
const N: i32 = 4;

// Vertex coordinates, texture coordinate
const VERTICES_COLORS: &[f32] = &[
    -0.5, 0.5, 0.0, 1.0, //
    -0.5, -0.5, 0.0, 0.0, //
    0.5, 0.5, 1.0, 1.0, //
    0.5, -0.5, 1.0, 0.0, //
];

const FSIZE: i32 = std::mem::size_of::<f32>() as i32;

pub enum Message {
    Texture0Loaded(HtmlImageElement),
    Texture1Loaded(HtmlImageElement),
}

pub struct Page {
    gl: Option<GL>,
    canvas: NodeRef,
    texture0: Option<WebGlTexture>,
    texture1: Option<WebGlTexture>,
    texture0_actived: bool,
    texture1_actived: bool,
    u_sampler0: Option<WebGlUniformLocation>,
    u_sampler1: Option<WebGlUniformLocation>,
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

        let texture0 = gl.create_texture();
        let texture1 = gl.create_texture();
        if texture0.is_none() || texture1.is_none() {
            return Err(JsError::new("Failed to create the texture object").into());
        }

        let u_sampler0 = gl.get_uniform_location(&program, "u_Sampler0");
        let u_sampler1 = gl.get_uniform_location(&program, "u_Sampler0");
        if u_sampler0.is_none() || u_sampler1.is_none() {
            return Err(JsError::new("Failed to get the storage location of u_Sampler").into());
        }

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        // Clear <canvas>
        gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl = Some(gl);
        self.texture0 = texture0;
        self.texture1 = texture1;
        self.u_sampler0 = u_sampler0;
        self.u_sampler1 = u_sampler1;
        Ok(())
    }

    fn request_texture0(&mut self, link: yew::html::Scope<Self>) {
        self.request_texture(
            "/resources/sky.jpg",
            Box::new(move |event| {
                let image = event.target().unwrap().dyn_into().unwrap();
                link.send_message(Message::Texture0Loaded(image));
            }),
        );
    }

    fn request_texture1(&mut self, link: yew::html::Scope<Self>) {
        self.request_texture(
            "/resources/circle.gif",
            Box::new(move |event| {
                let image = event.target().unwrap().dyn_into().unwrap();
                link.send_message(Message::Texture1Loaded(image));
            }),
        );
    }

    fn request_texture(&mut self, src: &str, callback: Box<dyn FnMut(web_sys::Event)>) {
        let image = gloo::utils::document()
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        image.set_cross_origin(Some("anonymous"));
        image.set_src(src);
        let closure = Closure::wrap(callback);
        image.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            // log::error!("failed loading texture");
            gloo::console::error!("failed loading texture");
        }) as Box<dyn FnMut(_)>);
        image.set_onerror(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    fn load_texture0(&mut self, image: HtmlImageElement) -> Result<(), JsValue> {
        self.texture0_actived = true;
        self.load_texture(
            self.u_sampler0.as_ref(),
            self.texture0.as_ref(),
            GL::TEXTURE0,
            0,
            image,
        )
    }

    fn load_texture1(&mut self, image: HtmlImageElement) -> Result<(), JsValue> {
        self.texture1_actived = true;
        self.load_texture(
            self.u_sampler1.as_ref(),
            self.texture1.as_ref(),
            GL::TEXTURE1,
            1,
            image,
        )
    }

    fn load_texture(
        &self,
        location: Option<&WebGlUniformLocation>,
        texture: Option<&WebGlTexture>,
        gl_unit: u32,
        unit: i32,
        image: HtmlImageElement,
    ) -> Result<(), JsValue> {
        if let Some(gl) = self.gl.as_ref() {
            gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);
            // Make the texture unit active
            gl.active_texture(gl_unit);
            // Bind the texture object to the target
            gl.bind_texture(GL::TEXTURE_2D, texture);

            // Set the texture parameters
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
            // Set the texture image
            gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                GL::TEXTURE_2D,
                0,
                GL::RGBA as i32,
                GL::RGBA,
                GL::UNSIGNED_BYTE,
                &image,
            )?;

            // Set the texture unit 0 to the sampler
            gl.uniform1i(location, unit);

            if self.texture0_actived && self.texture1_actived {
                // Clear <canvas>
                gl.clear(GL::COLOR_BUFFER_BIT);

                // Draw the rectangle
                gl.draw_arrays(GL::TRIANGLE_STRIP, 0, N);
            }
        }
        Ok(())
    }
}

impl yew::Component for Page {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            gl: None,
            canvas: NodeRef::default(),
            texture0: None,
            texture1: None,
            texture0_actived: false,
            texture1_actived: false,
            u_sampler0: None,
            u_sampler1: None,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Texture0Loaded(image) => {
                self.load_texture0(image).unwrap();
            }
            Message::Texture1Loaded(image) => {
                self.load_texture1(image).unwrap();
            }
        }
        false
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <canvas
                ref={self.canvas.clone()}
                width="400"
                height="400"
            />
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            self.setup_gl().unwrap_throw();
            self.request_texture0(ctx.link().clone());
            self.request_texture1(ctx.link().clone());
        }
    }
}

fn init_vertex_buffers(gl: &GL, program: &WebGlProgram) -> Result<(), JsError> {
    // let vertices_tex_coords = Float32Array::from(VERTICES_COLORS);
    // use view() instead of from() to avoid additional memory allocation
    let vertices_tex_coords = unsafe { Float32Array::view(VERTICES_COLORS) };
    let vertex_buffer = gl.create_buffer();
    if vertex_buffer.is_none() {
        return Err(JsError::new("Failed to create the buffer object"));
    }
    gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref());
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_tex_coords, GL::STATIC_DRAW);

    let a_position = gl.get_attrib_location(program, "a_Position");
    if a_position < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_Position",
        ));
    }

    gl.vertex_attrib_pointer_with_i32(a_position as u32, 2, GL::FLOAT, false, FSIZE * 4, 0);
    gl.enable_vertex_attrib_array(a_position as u32);

    let a_tex_coord = gl.get_attrib_location(program, "a_TexCoord");
    if a_tex_coord < 0 {
        return Err(JsError::new(
            "Failed to get the storage location of a_TexCoord",
        ));
    }

    gl.vertex_attrib_pointer_with_i32(
        a_tex_coord as u32,
        2,
        GL::FLOAT,
        false,
        FSIZE * 4,
        FSIZE * 2,
    );
    gl.enable_vertex_attrib_array(a_tex_coord as u32);

    Ok(())
}
