#[macro_use]
extern crate gfx;
pub use gfx::format as fm;

gfx_vertex_struct!(Vertex {
    _x: i8 = "x",
    _y: f32 = "y",
});

gfx_vertex_struct!(Instance {
    pos: [f32; 2] = "pos",
    color: [f32; 3] = "color",
});

gfx_constant_struct!(Local {
    pos: [u32; 4] = "pos",
});

#[derive(Clone, Debug)]
pub struct Rg16;
gfx_format!(Rg16: R16_G16 = Vec2<Float>);

gfx_pipeline!( testpipe {
    _vertex: gfx::VertexBuffer<Vertex> = (),
    _instance: gfx::InstanceBuffer<Instance> = (),
    _const_locals: gfx::ConstantBuffer<Local> = "Locals",
    _global: gfx::Global<[f32; 4]> = "Global",
    tex_diffuse: gfx::ShaderResource<[f32; 4]> = "Diffuse",
    sampler_linear: gfx::Sampler = "Linear",
    buf_frequency: gfx::UnorderedAccess<[f32; 4]> = "Frequency",
    pixel_color: gfx::RenderTarget<fm::Rgba8> = "Color",
    blend_target: gfx::BlendTarget<Rg16> =
        ("o_Color1", gfx::state::MASK_ALL, gfx::preset::blend::ADD),
    raw_target: gfx::RawRenderTarget = ("o_Color2",
        fm::Format(fm::SurfaceType::R8_G8_B8_A8, fm::ChannelType::Unorm),
        gfx::state::MASK_ALL, None),
    depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_TEST,
    blend_ref: gfx::BlendRef = (),
    scissor: gfx::Scissor = (),
});

fn _test_pso<R, F>(factory: &mut F)
             -> gfx::PipelineState<R, testpipe::Meta>  where
    R: gfx::Resources,
    F: gfx::traits::FactoryExt<R>,
{
    factory.create_pipeline_simple(&[], &[],
        gfx::state::CullFace::Nothing, testpipe::new()
        ).unwrap()
}
