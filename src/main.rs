
/*
https://github.com/glium/glium/tree/master/book
*/

#[macro_use]
extern crate glium;
extern crate ordered_float;
extern crate rand;

use std::env;
use std::path::Path;

mod file;
mod geometry;
mod graphics;
mod voroni;

fn main() {
	let args : Vec<String> = env::args().collect();
	let sites;
	if args.len() > 1 {
		let path = Path::new(&args[1]);
		sites = file::io::read_site_file(path);
	} else {
		sites = vec![(0,0)];
	}
	
	let mut voroni_process = voroni::voroni_process::VoroniProcess::new(sites);
	voroni_process.execute();
	println!("{:?}", voroni_process.get_dcel());
	let gl_vertices = voroni_process.get_dcel().get_opengl_vertices();

	/*
	let delaunay_dcel = delaunay::execute(voroni_dcel);
	io::outputFile(voroni_dcel, "VoroniOutput.txt");
	io::outputFile(delaunay_dcel, "DelaunayOutput.txt");
	graphics::draw(voroni_dcel, bounds);
	graphics::draw(delaunay_dcel, bounds);
	*/

	graphics::display::opengl_window(gl_vertices);
	let string = format!("{:?}", voroni_process.get_dcel());
	file::io::write_output_file(string);
}

