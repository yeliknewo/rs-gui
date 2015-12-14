#[macro_use] extern crate glium;
use glium::{DisplayBuild, Surface, Frame};
use glium::backend::glutin_backend::GlutinFacade;

use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Vertex{
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

struct Keyboard {
    keys : HashMap<glium::glutin::VirtualKeyCode, glium::glutin::ElementState>,
}

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard{
            keys : HashMap::new(),
        }
    }

    fn get_key_state(&self, key : glium::glutin::VirtualKeyCode) -> glium::glutin::ElementState {
        match self.keys.get(&key) {
            Some(state) => *state,
            None => glium::glutin::ElementState::Released,
        }
    }

    fn set_key_state(&mut self, key : glium::glutin::VirtualKeyCode, state : glium::glutin::ElementState) {
        self.keys.insert(key, state);
    }
}

pub struct GUI {
    keyboard : Keyboard,
    facade : GlutinFacade,
    program : glium::Program,
}

impl GUI {
    pub fn new() -> GUI {
        let facade: GlutinFacade =  match glium::glutin::WindowBuilder::new()
            .with_dimensions(640, 480)
            .with_title("GUI".to_string())
            .with_gl(glium::glutin::GlRequest::Latest)
            .with_gl_profile(glium::glutin::GlProfile::Core)
            .with_gl_robustness(glium::glutin::Robustness::NotRobust)
            .build_glium() {
                Ok(facade) => facade,             //glium::backend_::glutin_backend::GlutinFacade
                Err(error) => panic!(error),    //glium::glutin::GliumCreationError<glium::glutin::CreationError>
            };

        let program : glium::Program = match glium::Program::from_source(
            &facade,
            r#"
                #version 140

                in vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            "#,
            r#"
                #version 140

                out vec4 color;

                void main() {
                    color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "#,
            None) {
                Ok(program) => program,
                Err(error) => panic!(error),
            };

        GUI{
            keyboard : Keyboard::new(),
            facade : facade,
            program : program,
        }
    }
}

fn main() {
    let mut width : u32 = 640;
    let mut height : u32 = 480;

    let facade : GlutinFacade = match glium::glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title("GUI".to_string())
        .with_gl(glium::glutin::GlRequest::Latest)
        .with_gl_profile(glium::glutin::GlProfile::Core)
        .with_gl_robustness(glium::glutin::Robustness::NotRobust)
        .build_glium() {
            Ok(facade) => facade,             //glium::backend_::glutin_backend::GlutinFacade
            Err(error) => panic!(error),    //glium::glutin::GliumCreationError<glium::glutin::CreationError>
        };

    let vertex1 : Vertex = Vertex {position : [-1.0, -1.0] };
    let vertex2 : Vertex = Vertex {position : [1.0, -1.0] };
    let vertex3 : Vertex = Vertex {position : [1.0, 1.0] };
    let vertex4 : Vertex = Vertex {position : [-1.0, 1.0] };

    let shape: Vec<Vertex> = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer : glium::VertexBuffer<Vertex> = match glium::VertexBuffer::new(&facade, &shape) {
        Ok(buffer) => buffer,
        Err(error) => panic!(error),
    };

    let indices : Vec<u32> = vec!{
        0, 1, 2,
        2, 3, 0,
    };

    let index_buffer : glium::index::IndexBuffer<_> = match glium::index::IndexBuffer::new(
        &facade,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    ){
        Ok(buffer) => buffer,
        Err(error) => panic!(error),
    };

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program : glium::Program = match glium::Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None){
        Ok(program) => program,
        Err(error) => panic!(error),
    };

    let mut keyboard : Keyboard = Keyboard::new();

    loop{
        let mut frame: Frame = facade.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);
        match frame.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms, &Default::default()){
            Ok(()) => (),
            Err(error) => panic!(error),
        }
        match frame.finish() {
            Ok(()) => (),
            Err(error) => panic!(error),
        }

        for event in facade.poll_events() {
            match event{
                glium::glutin::Event::Closed => return,                                             //on window closed exit main
                glium::glutin::Event::Resized(new_width, new_height) => {                           //on window resized
                    width = new_width;
                    height = new_height;
                },
                glium::glutin::Event::KeyboardInput(state, _, key_code) => {
                    match key_code{
                        Some(code) => keyboard.set_key_state(code, state),
                        None => (),
                    }
                },
                _ => (),                                                                            //on other event do nothing
            }
        }

        match keyboard.get_key_state(glium::glutin::VirtualKeyCode::Escape) {
            glium::glutin::ElementState::Pressed => return,
            glium::glutin::ElementState::Released => (),
        }
    }
}
