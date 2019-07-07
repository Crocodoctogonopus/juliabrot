extern crate ezgl;
extern crate gl;
extern crate glutin;
extern crate nalgebra;

use ezgl::*;
use glutin::dpi::*;
use glutin::Event::WindowEvent;
use glutin::Event::*;
use glutin::*;
use nalgebra::*;
use std::path::*;

pub fn get_root() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn main() {
    // window, loop and context
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("JuliaBrot")
        .with_dimensions(LogicalSize::new(640. * 2., 640.));
    let context = ContextBuilder::new().with_vsync(true);
    let window = GlWindow::new(window_builder, context, &events_loop).unwrap();

    // build gl context
    unsafe {
        use crate::glutin::GlContext;

        window.make_current().unwrap();
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0., 0., 0., 1.);
    }

    //
    let mandel_prog = ProgramBuilder::new()
        .with(Shader::from_file(&get_root().join("shaders/mandelbrot.vert")).unwrap())
        .with(Shader::from_file(&get_root().join("shaders/mandelbrot.frag")).unwrap())
        .build()
        .unwrap();

    let julia_prog = ProgramBuilder::new()
        .with(Shader::from_file(&get_root().join("shaders/julia.vert")).unwrap())
        .with(Shader::from_file(&get_root().join("shaders/julia.frag")).unwrap())
        .build()
        .unwrap();

    //
    let mandel_verts = Buffer::<(f32, f32)>::from(
        gl::ARRAY_BUFFER,
        &[(-1., 1.), (0., 1.), (0., -1.), (-1., -1.)],
    );
    let mandel_view = Buffer::<(f32, f32)>::from(
        gl::ARRAY_BUFFER,
        &[(-1., -1.), (1., -1.), (1., 1.), (-1., 1.)],
    );

    let julia_verts = Buffer::<(f32, f32)>::from(
        gl::ARRAY_BUFFER,
        &[(0., 1.), (1., 1.), (1., -1.), (0., -1.)],
    );
    let julia_view = Buffer::<(f32, f32)>::from(
        gl::ARRAY_BUFFER,
        &[(-1., -1.), (1., -1.), (1., 1.), (-1., 1.)],
    );

    // ibo for both
    let ibo = Buffer::<u8>::from(gl::ELEMENT_ARRAY_BUFFER, &[0, 1, 2, 2, 3, 0]);

    // state
    let mut scale = 1.;
    let mut offset = Vector3::new(0.25, 0., 0.);
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    let mut plus = false;
    let mut minus = false;

    // update/draw
    loop {
        // event poll
        let mut exit = false;
        events_loop.poll_events(|event| {
            match event {
                WindowEvent {
                    event:
                        glutin::WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(code),
                                    state,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    // code: glutin::VirtualKeyCode
                    // state: glutin::ElementState

                    // set keyboard
                    let b = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };

                    match code {
                        VirtualKeyCode::W => up = b,
                        VirtualKeyCode::A => left = b,
                        VirtualKeyCode::S => down = b,
                        VirtualKeyCode::D => right = b,
                        VirtualKeyCode::Q => plus = b,
                        VirtualKeyCode::E => minus = b,
                        _ => {}
                    }
                }
                WindowEvent {
                    event: glutin::WindowEvent::CloseRequested,
                    ..
                } => exit = true,
                _ => {}
            }
        });
        if exit {
            break;
        }

        if plus {
            scale /= 2f32.powf(2. * 0.066);
        }
        if minus {
            scale *= 2f32.powf(2. * 0.066);
        }
        if left {
            offset.x -= 0.05 * scale;
        }
        if right {
            offset.x += 0.05 * scale;
        }
        if up {
            offset.y -= 0.05 * scale;
        }
        if down {
            offset.y += 0.05 * scale;
        }

        // unif
        let mandel_trans = {
            // create ortho
            let matrix = Matrix4::new_translation(&offset);
            let matrix = matrix * Matrix4::new_scaling(scale);

            // return it
            let mut t = Mat4([0.; 16]);
            t.0.clone_from_slice(matrix.as_slice());
            GLSLAny::Mat4(t)
        };

        let julia_trans = {
            // create ortho
            let matrix = Matrix4::new_translation(&offset);
            let matrix = matrix * Matrix4::new_scaling(scale);

            // return it
            let mut t = Mat4([0.; 16]);
            t.0.clone_from_slice(matrix.as_slice());
            GLSLAny::Mat4(t)
        };

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        InstantDraw::start_tri_draw(2, &mandel_prog, &ibo)
            .with_buffer(&mandel_verts, 0)
            .with_buffer(&mandel_view, 1)
            .with_uniform(mandel_trans, 0)
            //.with_uniform(GLSLAny::Mat4(mandel_view), 0)
            .draw();

        InstantDraw::start_tri_draw(2, &julia_prog, &ibo)
            .with_buffer(&julia_verts, 0)
            .with_buffer(&julia_view, 1)
            .with_uniform(GLSLAny::Vec2((offset.x, offset.y)), 1)
            .draw();

        window.swap_buffers().unwrap();
    }
}
