#[macro_use]
extern crate glium;

extern crate image;

use std::io::Cursor;

use glium::Surface;
use glium::index::PrimitiveType;
#[allow(unused_imports)]
use glium::glutin::{self, event::{ElementState, VirtualKeyCode, Event, WindowEvent}};

mod support;

fn main() {
    // building the display, ie. the main object
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_fullscreen(Some(event_loop.primary_monitor()));
    let cb = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::GlThenGles { opengl_version: (3, 0), opengles_version: (2, 0) });
    let gl_window = cb.build_windowed(wb, &event_loop).unwrap();
    let display = glium::Display::with_debug(gl_window, glium::debug::DebugCallbackBehavior::PrintAll).unwrap();

    // building a texture with "OpenGL" drawn on it
    let image = image::load(Cursor::new(&include_bytes!("../tests/fixture/opengl.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let opengl_texture = glium::texture::Texture2d::new(&display, image).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
            ]
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip,
                                               &[1 as u16, 2, 0, 3]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;

                in vec2 position;
                in vec2 tex_coords;

                out vec2 v_tex_coords;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 color;

                void main() {
                    color = texture(tex, v_tex_coords);
                }
            "
        },

        100 => {
            vertex: "
                #version 100

                precision mediump float;
                uniform mat4 matrix;

                attribute vec2 position;
                attribute vec2 tex_coords;

                varying vec3 vColor;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = vec3(tex_coords, 0.5);
                }
            ",

            fragment: "
                #version 100
                precision mediump float;
                varying vec3 vColor;

                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    ).unwrap();

    let mut fullscreen = false;

    println!("Press Enter to switch fullscreen mode");

    support::start_loop(&display, || {
        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniform! {
                matrix: [
                    [0.5, 0.0, 0.0, 0.0],
                    [0.0, 0.5, 0.0, 0.0],
                    [0.0, 0.0, 0.5, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ],
                tex: &opengl_texture
            }, &Default::default()).unwrap();
        target.finish().unwrap();

        let mut action = support::Action::Continue;

    //     // polling and handling the events received by the window
    //     let mut enter_pressed = false;
    //     event_loop.poll_events(|event| match event {
    //         Event::WindowEvent { event, window_id } =>
    //             if window_id == display.gl_window().window().id() {
    //                 match event {
    //                     WindowEvent::CloseRequested => action = support::Action::Stop,
    //                     WindowEvent::KeyboardInput { input, .. } => {
    //                         if let ElementState::Pressed = input.state {
    //                             if let Some(VirtualKeyCode::Return) = input.virtual_keycode {
    //                                 enter_pressed = true;
    //                             }
    //                         }
    //                     },
    //                     _ => ()
    //                 }
    //             },
    //         _ => (),
    //     });

    //     // If enter was pressed toggle fullscreen.
    //     if enter_pressed {
    //         if fullscreen {
    //             let wb = glutin::window::WindowBuilder::new();
    //             let cb = glutin::ContextBuilder::new();
    //             display.rebuild(wb, cb, &event_loop).unwrap();
    //             fullscreen = false;
    //         } else {
    //             let wb = glutin::window::WindowBuilder::new()
    //                 .with_fullscreen(Some(event_loop.get_primary_monitor()));
    //             let cb = glutin::ContextBuilder::new();
    //             display.rebuild(wb, cb, &event_loop).unwrap();
    //             fullscreen = true;
    //         }
    //     }

        action
    });
}
