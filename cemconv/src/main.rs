extern crate cem;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate wavefront_obj;

use wavefront_obj::obj::{self, Object};
use std::fs::File;
use std::io::{self, Read, Write};
use cem::{ModelHeader, v2, V2, Scene, Model};

#[derive(StructOpt, Debug)]
struct Opt {
	#[structopt(short = "i", long = "input", help = "Input file to convert, default is stdout")]
	input: Option<String>,
	#[structopt(short = "g", long = "iformat", help = "Format to use for the input")]
	input_format: Option<String>,
	#[structopt(short = "f", long = "format", help = "Format to use as the output")]
	format: String,
	#[structopt(help = "Output file, default is stdout")]
	output: Option<String>
}

enum Format {
	Cem(u16, u16),
	Obj
}

fn main() {
	use structopt::StructOpt;

	let opt = Opt::from_args();
	let format = match &opt.format as &str {
		"cem1.3" => Format::Cem(1 ,3),
		"cem2" => Format::Cem(2, 0),
		"cem" => Format::Cem(2, 0),
		"ssmf" => Format::Cem(2, 0),
		"obj" => Format::Obj,
		_ => {
			eprintln!("Unrecognized output format {:?}", opt.format);
			return;
		}
	};

	let input_format = match opt.input_format.as_ref().map(|s| s as &str) {
		Some("cem1.3") => Format::Cem(1, 3),
		Some("cem2")   => Format::Cem(2, 0),
		Some("cem")    => Format::Cem(2, 0),
		Some("ssmf")   => Format::Cem(2, 0),
		Some("obj")    => Format::Obj,
		Some(_) | None => Format::Cem(2, 0)
	};

	let stdin = io::stdin();
	let stdout = io::stdout();

	match (opt.input, opt.output) {
		(None, None) => convert (
			stdin.lock(),
			stdout.lock(),
			input_format,
			format
		),
		(None, Some(path)) => convert (
			stdin.lock(),
			File::open(path).unwrap(),
			input_format,
			format
		),
		(Some(path), None) => convert (
			File::open(path).unwrap(),
			stdout.lock(),
			input_format,
			format
		),
		(Some(input), Some(output)) => convert (
			File::open(input).unwrap(),
			File::open(output).unwrap(),
			input_format,
			format
		)
	}.unwrap();
}

fn convert<I, O>(mut i: I, mut o: O, input_format: Format, format: Format) -> io::Result<()> where I: Read, O: Write {
	match (input_format, format) {
		(Format::Obj, Format::Cem(2, 0)) => {
			let mut buffer = String::new();
			i.read_to_string(&mut buffer)?;

			let obj = obj::parse(buffer).map_err(
				|parse| io::Error::new(io::ErrorKind::InvalidData, format!("Error in OBJ file on line {}: {}", parse.line_number, parse.message))
			)?;

			let model = obj_to_cem(&obj.objects[0]);

			Scene::root(model).write(&mut o)
		},
		(Format::Cem(2, 0), Format::Cem(2, 0)) => {
			let header = ModelHeader::read(&mut i)?;

			if header == V2::HEADER {
				Scene::<V2>::read_without_header(&mut i)?.write(&mut o)
			} else {
				unimplemented!("Cannon rewrite non-CEMv2 files yet.")
			}
		},
		(Format::Cem(_, _), Format::Obj) => {
			let header = ModelHeader::read(&mut i)?;

			if header == V2::HEADER {
				let scene = Scene::<V2>::read_without_header(&mut i)?;

				let buffer = cem2_to_obj(scene.model);

				o.write_all(buffer.as_bytes())
			} else {
				unimplemented!("Cannon convert non-CEMv2 files to OBJ yet.")
			}
		},
		_ => unimplemented!()
	}
}

fn obj_to_cem(_i: &Object) -> V2 {
	unimplemented!("OBJ to CEM not supported.")
}

fn cem2_to_obj(cem: V2) -> String {
	use std::fmt::Write;

	let triangle_data = &cem.lod_levels[0];
	let frame = &cem.frames[0];

	let mut string = String::new();

	for &v2::Vertex { position, normal, texture } in frame.vertices.iter() {
		// Swap Y and Z to make models look upright. However, this seems to make them appear flipped across the Y=X axis?
		// TODO: This needs to be investigated further.
		writeln!(string, "v {} {} {}", position.0, position.2, position.1).unwrap();
		writeln!(string, "vn {} {} {}", normal.0, normal.2, normal.1).unwrap();
		writeln!(string, "vt {} {}", texture.0, texture.1).unwrap();
	}

	for &v2::Material { ref name, texture, ref triangles, vertex_offset, vertex_count: _vertex_count, ref texture_name } in &cem.materials {
		let triangle_slice = triangles[0];

		writeln!(string, "# name: {}, texture: {}, texture_name: {}", name, texture, texture_name).unwrap();

		for index in 0..triangle_slice.len {
			let index = index + triangle_slice.offset;
			let triangle = &triangle_data[index as usize];

			let indices = (
				vertex_offset + triangle.0 + 1,
				vertex_offset + triangle.1 + 1,
				vertex_offset + triangle.2 + 1
			);

			writeln!(string, "f {}/{}/{} {}/{}/{} {}/{}/{}", indices.0, indices.0, indices.0, indices.1, indices.1, indices.1, indices.2, indices.2, indices.2).unwrap();
		}
	}

	string
}