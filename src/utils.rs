use wasm_bindgen::{JsCast, JsError, JsValue};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader,
};
use yew::NodeRef;

fn noop() {}

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

pub fn use_2d_canvas_render<
    C: FnOnce(CanvasRenderingContext2d) -> Result<(), JsError> + 'static,
>(
    node: NodeRef,
    render: C,
) {
    use_canvas_render(node, |canvas| {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        render(ctx)?;
        Ok(())
    })
}

pub fn use_webgl2_canvas_render<
    C: FnOnce(WebGl2RenderingContext) -> Result<(), JsError> + 'static,
>(
    node: NodeRef,
    render: C,
) {
    use_canvas_render(node, |canvas| {
        let ctx = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        render(ctx)?;
        Ok(())
    })
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsError> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| JsError::new("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context.get_shader_info_log(&shader).map_or_else(
            || JsError::new("Unknown error creating shader"),
            |info_log| JsError::new(&info_log),
        ))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, JsError> {
    let program = context
        .create_program()
        .ok_or_else(|| JsError::new("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context.get_program_info_log(&program).map_or_else(
            || JsError::new("Unknown error creating program object"),
            |info_log| JsError::new(&info_log),
        ))
    }
}
