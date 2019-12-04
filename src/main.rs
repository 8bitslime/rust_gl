
extern crate gl;
extern crate glam;
extern crate glutin;

mod shader;
mod mesh;

use {
    gl::types::*,

    glutin::event::{Event, WindowEvent},
    glutin::event_loop::{ControlFlow, EventLoop},
    glutin::window::WindowBuilder,
    glutin::ContextBuilder,
    glutin::{GlProfile, GlRequest, Api},
    
    glam::*,
    
    shader::*,
    mesh::*,
};

type Context = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

fn gl_get_string(name: GLenum) -> String {
    use std::ffi::CStr;
    unsafe {
        CStr::from_ptr(gl::GetString(name) as *const _).to_str().unwrap().to_owned()
    }
}

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("New window, who this?")
        .with_inner_size(glutin::dpi::LogicalSize::new(640.0, 480.0));
    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 5)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    
    let context = unsafe { context.make_current().unwrap() };
    gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);
    
    println!("OpenGL Renderer:\t{}", gl_get_string(gl::RENDERER));
    println!("OpenGL Version:\t{}", gl_get_string(gl::VERSION));
    
    let shader_source = r#"
    #ifdef VERTEX
    
    layout (location = 0) in vec3 pos;
    uniform mat4 world;
    uniform mat4 model;
    
    void main() {
        gl_Position = world * model * vec4(pos, 1);
    }
    #endif
    
    #ifdef FRAGMENT
    out vec4 glColor;
    void main() {
        glColor = vec4(1, 1, 1, 1);
    }
    #endif
    "#;
    
    let program = match Program::from_source(shader_source) {
        Ok(prog) => prog,
        Err(msg) => panic!("Shader failed to compile: {}", msg),
    };
    program.bind();
    
    let model_loc = program.get_uniform("model").unwrap();
    let world_loc = program.get_uniform("world").unwrap();
    
    let triangle = &[ vec3(0., 1., 0.), vec3(1., -1., 0.), vec3(-1., -1., 0.) ];
    let mesh = Mesh::new(1);
    mesh.buffer_data_3f(0, triangle);
    
    unsafe {
        let world_mat = Mat4::perspective_rh_gl(1.57, 640./480., 0.001, 1000.0);
        gl::UniformMatrix4fv(world_loc, 1, gl::FALSE, world_mat.as_ref().as_ptr());
    }
    
    let start_time = std::time::Instant::now();
    let render = move |context: &Context| {
        let model_mat = 
            Mat4::from_translation(Vec3::new(0., 0., -3.)) *
            Mat4::from_axis_angle(Vec3::new(0., 1., 0.), start_time.elapsed().as_secs_f32());
        unsafe {
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model_mat.as_ref().as_ptr());
            gl::ClearColor(1.0, 0.4, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            mesh.draw(3, 0);
        }
        context.swap_buffers().unwrap();
    };
    
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = context.window().hidpi_factor();
                    let physical_size = logical_size.to_physical(dpi_factor);
                    context.resize(physical_size);
                    let aspect_ratio = (physical_size.width / physical_size.height) as f32;
                    let (w, h): (u32, u32) = physical_size.into();
                    unsafe {
                        gl::Viewport(0, 0, w as i32, h as i32);
                        let world_mat = Mat4::perspective_rh_gl(1.57, aspect_ratio, 0.001, 1000.0);
                        gl::UniformMatrix4fv(world_loc, 1, gl::FALSE, world_mat.as_ref().as_ptr());
                        render(&context);
                    }
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                _ => ()
            },
            _ => ()
        }
        render(&context);
    });
}
