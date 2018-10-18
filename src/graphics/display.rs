
use glium;

#[derive(Copy, Clone)]
pub struct OpenglVertex {
	pub position: [f32; 2],
	pub color: [f32; 3],
}

pub fn opengl_window(input_shapes : Vec<OpenglVertex>) {
	use glium::Surface;

	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
		.with_dimensions(1024, 768)
		.with_title("Hello world");
	let context = glium::glutin::ContextBuilder::new();
	let display = glium::Display::new(window, context, &events_loop).unwrap();
	
	implement_vertex!(OpenglVertex, position, color);
	let vertex_buffer = glium::VertexBuffer::new(&display, &input_shapes).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
	//let index_buffer = glium::IndexBuffer::new(vec![0]);

	let vertex_shader_src = r#"
		#version 140
		
		in vec2 position;
		in vec3 color;

        out vec3 o_color;

		void main() {
            o_color = color;
			gl_Position = vec4(position, 0.0, 1.0);
		}
	"#;

	let fragment_shader_src = r#"
		#version 140

        in vec3 o_color;
		out vec4 out_color;

		void main() {
			out_color = vec4(o_color,0.0);
		}
	"#;
	
	let program = glium::Program::from_source(&display, vertex_shader_src, 
			fragment_shader_src, None).unwrap();
	
	let mut closed = false;
	while !closed {
	
		let mut target = display.draw();
		target.clear_color(0.05, 0.05, 0.05, 1.0);
		target.draw(&vertex_buffer, &indices, &program, &uniform! {},
				&Default::default()).unwrap();
		target.finish().unwrap();
	
		events_loop.poll_events(|ev| {
			match ev {
				glium::glutin::Event::WindowEvent { event, .. } => match event {
					glium::glutin::WindowEvent::Closed => closed = true,
					_ => (),
				},
				_ => (),
			}
		})
	}
}
