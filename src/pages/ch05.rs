mod colored_triangle;
mod hello_triangle_frag_coord;
mod multi_attribute_size;
mod multi_attribute_size_interleaved;
mod multi_texture;
mod textured_quad;
mod textured_quad_clamp_mirror;
mod textured_quad_repeat;

pub use self::colored_triangle::Page as ColoredTriangle;
pub use self::hello_triangle_frag_coord::Page as HelloTriangle_FragCoord;
pub use self::multi_attribute_size::Page as MultiAttributeSize;
pub use self::multi_attribute_size_interleaved::Page as MultiAttributeSize_Interleaved;
pub use self::multi_texture::Page as MultiTexture;
pub use self::textured_quad::Page as TexturedQuad;
pub use self::textured_quad_clamp_mirror::Page as TexturedQuad_Clamp_Mirror;
pub use self::textured_quad_repeat::Page as TexturedQuad_Repeat;
