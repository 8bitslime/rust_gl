
extern crate gl;
extern crate glam;
extern crate glutin;
extern crate image;
extern crate mol;

mod shader;
mod mesh;
mod texture;
mod gl2d;

use {
    gl::types::*,

    glutin::event::{Event, WindowEvent},
    glutin::event_loop::{ControlFlow, EventLoop},
    glutin::window::WindowBuilder,
    glutin::ContextBuilder,
    glutin::dpi::{LogicalSize},
    glutin::{GlProfile, GlRequest, Api},
    
    glam::*,
    
    shader::*,
    mesh::*,
    texture::*,
    
    std::path::Path,
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
        .with_inner_size(LogicalSize::new(640.0, 480.0))
        .with_min_inner_size(LogicalSize::new(320.0, 200.0));
    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 5)))
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &el)
        .unwrap();
    
    let context = unsafe { context.make_current().unwrap() };
    gl::load_with(|ptr| context.get_proc_address(ptr) as *const _);
    
    println!("OpenGL Renderer:\t{}", gl_get_string(gl::RENDERER));
    println!("OpenGL Version:\t{}", gl_get_string(gl::VERSION));
    
    //Set up OpenGL
    unsafe {
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
    }
    
    let shader_source = r#"
    #ifdef VERTEX
    
    layout (location = 0) in vec3 pos;
    layout (location = 1) in vec3 norm;
    layout (location = 2) in vec2 uv;
    uniform mat4 world;
    uniform mat4 model;
    
    out vec3 normal;
    out vec2 coord;
    
    void main() {
        gl_Position = world * model * vec4(pos, 1);
        normal = (model * vec4(norm, 0)).xyz;
        coord = uv;
    }
    #endif
    
    #ifdef FRAGMENT
    
    uniform sampler2D diffuse;
    
    in vec3 normal;
    in vec2 coord;
    
    out vec4 glColor;
    void main() {
        glColor = texture(diffuse, coord) * max(vec4(1, 1, 1, 1) * dot(vec3(0, 0, 1), normal), vec4(0.2));
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
    
    let start_time = std::time::Instant::now();
    
    let obj = mol::obj::OBJ::from_path(Path::new("res/box.obj")).unwrap();
    let mut verts = Vec::<Vec3>::new();
    let mut norms = Vec::<Vec3>::new();
    let mut uvs   = Vec::<Vec2>::new();
    
    for (vert, uv, norm) in obj.flat_iter() {
        let pos = Vec3::new(vert.x, vert.y, vert.z);
        let norm = Vec3::new(norm.x, norm.y, norm.z);
        let uv = Vec2::new(uv.x, uv.y);
        verts.push(pos);
        norms.push(norm);
        uvs.push(uv);
    }
    
    let duration = start_time.elapsed().as_micros() as f64;
    println!("Model load time: {}ms", duration / 1000.);
    
    let mesh_size = verts.len();
    
    let mesh = Mesh::new(3);
    mesh.buffer_data_3f(0, verts.as_ref());
    mesh.buffer_data_3f(1, norms.as_ref());
    mesh.buffer_data_2f(2, uvs.as_ref());
    
    let texture = Texture::from_path(Path::new("res/brick.png")).unwrap();
    texture.bind();
    
    unsafe {
        let world_mat = Mat4::perspective_rh_gl(1.7, 640./480., 0.001, 1000.0);
        gl::UniformMatrix4fv(world_loc, 1, gl::FALSE, world_mat.as_ref().as_ptr());
    }
    
    let start_time = std::time::Instant::now();
    let render = move |context: &Context| {
        let elapsed = start_time.elapsed().as_secs_f32();
        
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        };
        
        let distance = elapsed.sin() * 1.5 - 4.;
        
        let model_mat =
            Mat4::from_translation(Vec3::new(0., 0., distance)) *
            Mat4::from_axis_angle(Vec3::new(1., 1., 0.).normalize(), elapsed * std::f32::consts::PI * 0.5);
        program.set_uniform(model_loc, &model_mat);
        mesh.draw(mesh_size, 0);
        context.swap_buffers().unwrap();
    };
    
    el.run(move |event, _, control_flow| {
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
                _ => {}
            },
            _ => {}
        }
        render(&context);
    });
}
