use js_sys::Float32Array;
use reqwasm::http::Request;
use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{
    HtmlCanvasElement, ImageBitmap, WebGl2RenderingContext as GL, WebGlProgram, WebGlTexture,
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
uniform sampler2D u_Sampler;
varying vec2 v_TexCoord;
void main() {
  gl_FragColor = texture2D(u_Sampler, v_TexCoord);
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
    TextureLoaded(ImageBitmap),
}

pub struct TexturedQuad {
    gl: Option<GL>,
    canvas: NodeRef,
    texture: Option<WebGlTexture>,
    u_sampler: Option<WebGlUniformLocation>,
}

impl TexturedQuad {
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

        let texture = gl.create_texture();
        if texture.is_none() {
            return Err(JsError::new("Failed to create the texture object").into());
        }

        let u_sampler = gl.get_uniform_location(&program, "u_Sampler");
        if u_sampler.is_none() {
            return Err(JsError::new("Failed to get the storage location of u_Sampler").into());
        }

        // Specify the color for clearing <canvas>
        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        // Clear <canvas>
        gl.clear(GL::COLOR_BUFFER_BIT);

        self.gl = Some(gl);
        self.texture = texture;
        self.u_sampler = u_sampler;
        Ok(())
    }

    fn load_texture(&self, image: &ImageBitmap) -> Result<(), JsValue> {
        if let Some(gl) = self.gl.as_ref() {
            gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);
            // Enable texture unit0
            gl.active_texture(GL::TEXTURE0);
            // Bind the texture object to the target
            gl.bind_texture(GL::TEXTURE_2D, self.texture.as_ref());

            // Set the texture parameters
            gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
            // Set the texture image
            gl.tex_image_2d_with_u32_and_u32_and_image_bitmap(
                GL::TEXTURE_2D,
                0,
                GL::RGB as i32,
                GL::RGB,
                GL::UNSIGNED_BYTE,
                image,
            )?;

            // Set the texture unit 0 to the sampler
            gl.uniform1i(self.u_sampler.as_ref(), 0);

            // Clear <canvas>
            gl.clear(GL::COLOR_BUFFER_BIT);

            // Draw the rectangle
            gl.draw_arrays(GL::TRIANGLE_STRIP, 0, N);
        }
        Ok(())
    }
}

impl yew::Component for TexturedQuad {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            gl: None,
            canvas: NodeRef::default(),
            texture: None,
            u_sampler: None,
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::TextureLoaded(image) => {
                self.load_texture(&image).unwrap();
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
            self.setup_gl().unwrap_throw();
            ctx.link().send_future(request_texture());
        }
    }
}

fn init_vertex_buffers(gl: &GL, program: &WebGlProgram) -> Result<(), JsError> {
    let vertices_tex_coords = Float32Array::from(VERTICES_COLORS);
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

async fn request_texture() -> Message {
    let bytes = Request::new("/resources/sky.jpg")
        .send()
        .await
        .unwrap()
        .binary()
        .await
        .unwrap();
    let blob = gloo::file::Blob::new(bytes.as_slice());
    let promise = gloo::utils::window()
        .create_image_bitmap_with_blob(blob.as_ref())
        .unwrap();
    let bitmap = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .unwrap()
        .dyn_into::<ImageBitmap>()
        .unwrap();
    Message::TextureLoaded(bitmap)
}
