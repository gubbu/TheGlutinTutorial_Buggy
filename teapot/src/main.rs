//applying transformation to triangel inside vertex shader by introducing 4x4 matrices as uniform variables into it. 2D translation and rotation are shown.
#[macro_use]
extern crate glium;
//loading teapot
mod teapot;

const RED: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);

//main called once per vertex

//oh wow: the uniform called matrix has to be called marix inside the uniform! macro to, otherwise the uniform matrix would be seen as its default value a zero matrix, resulting in no triangel being visible on the screen.
const VERTEXSHADER: &str = r#"
#version 140
in vec3 position;
out vec3 color;
uniform mat4 matrix;
void main(){
    color = position*0.005;
    gl_Position = matrix*vec4(position, 1.0);
}
"#;

const FRAGMENTSHADER: &str = r#"
#version 140
out vec4 endcolour;
in vec3 color;
void main(){
    endcolour = vec4(color, 1.0);
}
"#;

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
    //load shape onto gpu
    let vertex_buffer = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    //normals
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    //indices for drawing:
    let indices = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();
    //load Shaders into gpu and compile it (slow)
    let program =
        glium::Program::from_source(&display, VERTEXSHADER, FRAGMENTSHADER, None).unwrap();
    let mut run = true;
    let mut t = -0.5f32;
    while run {
        //maipulating t for uniform use
        t += 0.02;
        //translation reset condition
        /*
        if t > 0.5 {
            t = -0.5;
        }
        */
        //rotation condition ... is not actually need the angel could go up to infin...
        if t > std::f32::consts::PI*2.0{
            t = 0.0;
        }
        let mut target = display.draw();
        use glium::Surface;
        target.clear_color(RED.0, RED.1, RED.2, RED.3);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform!(matrix: [
                    [0.01, 0.0,  0.0, 0.0],
                    [0.0, 0.01,  0.0, 0.0],
                    [0.0,  0.0, 0.01, 0.0],
                    [0.0,  0.0,  0.0, 1.0f32]
                ]),
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
