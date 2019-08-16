//applying transformation to triangel inside vertex shader by introducing 4x4 matrices as uniform variables into it. 2D translation and rotation are shown.
#[macro_use]
extern crate glium;

const RED: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);

//main called once per vertex

//oh wow: the uniform called matrix has to be called marix inside the uniform! macro to, otherwise the uniform matrix would be seen as its default value a zero matrix, resulting in no triangel being visible on the screen.
const VERTEXSHADER: &str = r#"
#version 140
in vec2 data;
out vec2 myattr;
void main(){
    myattr = data;
    gl_Position = vec4(data, 0.0, 1.0);
}
"#;

const FRAGMENTSHADER: &str = r#"
#version 140
in vec2 myattr;
out vec4 colour;
void main(){
    colour = vec4(myattr, 1.0, 1.0);
}
"#;

//if you call the field not data,... but data, but in the vertex shader it is data, than,... ERROR
#[derive(Copy, Clone)]
struct Vertex {
    data: [f32; 2],
}
//the field name: in this case data has to be the same as in the Vertex shader.
implement_vertex!(Vertex, data);

fn main() {
    //create the window
    use glium::glutin;
    let mut eventsloop = glutin::EventsLoop::new();
    let display = glium::Display::new(
        glutin::WindowBuilder::new(),
        glutin::ContextBuilder::new(),
        &eventsloop,
    )
    .unwrap();
    //creating triangel
    let shape = vec![
        Vertex { data: [0.5, -0.25] },
        Vertex { data: [0.0, 0.5] },
        Vertex { data: [-0.5, -0.5] },
    ];
    //load shape onto gpu
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    //indices for drawing:
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    //load Shaders into gpu and compile it (slow)
    let program =
        glium::Program::from_source(&display, VERTEXSHADER, FRAGMENTSHADER, None).unwrap();
    let mut run = true;
    while run {
        let mut target = display.draw();
        use glium::Surface;
        target.clear_color(RED.0, RED.1, RED.2, RED.3);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
        eventsloop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => {
                if event == glutin::WindowEvent::CloseRequested {
                    run = false;
                }
            }
            _ => {}
        });
        std::thread::sleep(std::time::Duration::from_millis(34));
    }
}
