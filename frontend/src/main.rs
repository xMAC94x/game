
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;
use gfx::traits::FactoryExt;
use glutin::GlContext;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "coord3d",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const SCALE: f32 = 0.3;
const THE_VOXEL: [Vertex; 12] = [
    //front
    Vertex { pos: [ -SCALE , -SCALE ,  SCALE ] },
    Vertex { pos: [ -SCALE ,  SCALE ,  SCALE ] },
    Vertex { pos: [  SCALE ,  SCALE ,  SCALE ] },
    Vertex { pos: [  SCALE ,  SCALE ,  SCALE ] },
    Vertex { pos: [  SCALE , -SCALE ,  SCALE ] },
    Vertex { pos: [ -SCALE , -SCALE ,  SCALE ] },

    //top
    Vertex { pos: [ -SCALE ,  SCALE ,  SCALE ] },
    Vertex { pos: [ -SCALE ,  SCALE ,  SCALE ] },
    Vertex { pos: [  SCALE ,  SCALE ,  SCALE ] },
    Vertex { pos: [  SCALE ,  SCALE ,  SCALE ] },
    Vertex { pos: [  SCALE ,  SCALE ,  -SCALE ] },
    Vertex { pos: [ -SCALE ,  SCALE ,  -SCALE ] },


];

fn main() {
    let window_builder = glutin::WindowBuilder::new().with_title("Example".to_string()).with_dimensions(640, 480);
    let context_builder = glutin::ContextBuilder::new().with_vsync(true);

    let mut events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!("voxel.glslv"),
            include_bytes!("voxel.glslf"),
            pipe::new(),
        )
        .unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&THE_VOXEL, ());

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out:  main_color,
    };

    let mut running = true;
    while running {
        // fetch events
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => return,
                    _ => (),
                }
            }
        });

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
