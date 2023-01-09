use wasm_bindgen::{JsCast, JsError, JsValue, UnwrapThrowExt};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader,
};
use yew::NodeRef;

fn noop() {}

#[yew::hook]
pub fn use_canvas_render<C: FnOnce(HtmlCanvasElement) -> Result<(), JsValue> + 'static>(
    node: NodeRef,
    render: C,
) {
    yew::use_effect_with_deps(
        move |_| {
            let canvas = node.cast::<HtmlCanvasElement>().unwrap();
            render(canvas).unwrap();
            noop
        },
        (),
    )
}

#[yew::hook]
pub fn use_2d_canvas_render<
    C: FnOnce(CanvasRenderingContext2d) -> Result<(), JsError> + 'static,
>(
    node: NodeRef,
    render: C,
) {
    use_canvas_render(node, |canvas| {
        let ctx = canvas
            .get_context("2d")
            .unwrap_throw()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        render(ctx)?;
        Ok(())
    })
}

#[yew::hook]
pub fn use_webgl2_canvas_render<
    C: FnOnce(WebGl2RenderingContext) -> Result<(), JsError> + 'static,
>(
    node: NodeRef,
    render: C,
) {
    use_canvas_render(node, |canvas| {
        let ctx = canvas
            .get_context("webgl2")
            .unwrap_throw()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        render(ctx)?;
        Ok(())
    })
}

pub trait WebGl2RenderingContextExt {
    fn compile_shader_from_source(
        &self,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, JsError>;
    fn link_program_with_shaders(
        &self,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, JsError>;
    fn init_shaders(&self, vert_shader: &str, frag_shader: &str) -> Result<WebGlProgram, JsError>;

    fn compile_vertex_shader_from_source(&self, source: &str) -> Result<WebGlShader, JsError> {
        self.compile_shader_from_source(WebGl2RenderingContext::VERTEX_SHADER, source)
    }

    fn compile_fragment_shader_from_source(&self, source: &str) -> Result<WebGlShader, JsError> {
        self.compile_shader_from_source(WebGl2RenderingContext::FRAGMENT_SHADER, source)
    }
}

impl WebGl2RenderingContextExt for WebGl2RenderingContext {
    fn compile_shader_from_source(
        &self,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, JsError> {
        let shader = self
            .create_shader(shader_type)
            .ok_or_else(|| JsError::new("Unable to create shader object"))?;
        self.shader_source(&shader, source);
        self.compile_shader(&shader);

        if self
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self.get_shader_info_log(&shader).map_or_else(
                || JsError::new("Unknown error creating shader"),
                |info_log| JsError::new(&info_log),
            ))
        }
    }

    fn link_program_with_shaders(
        &self,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, JsError> {
        let program = self
            .create_program()
            .ok_or_else(|| JsError::new("Unable to create shader object"))?;

        self.attach_shader(&program, vert_shader);
        self.attach_shader(&program, frag_shader);
        self.link_program(&program);

        if self
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(self.get_program_info_log(&program).map_or_else(
                || JsError::new("Unknown error creating program object"),
                |info_log| JsError::new(&info_log),
            ))
        }
    }

    fn init_shaders(&self, vert_shader: &str, frag_shader: &str) -> Result<WebGlProgram, JsError> {
        let vert_shader = self.compile_vertex_shader_from_source(vert_shader)?;
        let frag_shader = self.compile_fragment_shader_from_source(frag_shader)?;
        let program = self.link_program_with_shaders(&vert_shader, &frag_shader)?;
        self.use_program(Some(&program));
        Ok(program)
    }
}
