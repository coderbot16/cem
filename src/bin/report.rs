extern crate cem;

use cem::{ModelHeader, v1, v2, v5};
use std::io::BufReader;

const PATH: &str = "/home/coderbot/Programming/Java/EmpireEarthReverse/extract/data/models";
// const PATH: &str = "/home/coderbot/Empire Earth/DOTMW Data Files/models";

fn main() {
	for entry in ::std::fs::read_dir(PATH).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		let name = entry.file_name().to_string_lossy().to_string();

		let mut file = BufReader::new(::std::fs::File::open(path).unwrap());

		let header = ModelHeader::read(&mut file).unwrap();

		if header == v2::EXPECTED_MODEL_HEADER {

			let model = v2::V2::read(&mut file).unwrap();

			for frame in &model.frames {
				use cem::collider::ColliderBuilder;

				let mut builder = ColliderBuilder::begin(model.node.center);
				for vertex in &frame.vertices {
					builder.update(vertex.position);
				}

				let collider = builder.build();

				// Account for tiny differences. Titan uses f80 for computations, but we use f32.
				// This can cause small but insignificant deviations.
				let radius_eq = (collider.radius - frame.collider.radius).abs() < 0.0000005;

				if !radius_eq && collider.aabb == frame.collider.aabb {
					println!("  {:32} Radius mismatch: Expected {}, got {}", name, frame.collider.radius, collider.radius);
				} else if radius_eq && collider.aabb != frame.collider.aabb {
					println!("  {:32} Aabb mismatch: Expected {:?}, {:?}", name, frame.collider.aabb, collider.aabb);
				} else if !radius_eq && collider.aabb != frame.collider.aabb {
					println!("  {:32} Collider mismatch: Expected (radius = {}, {:?}), got (radius = {}, {:?})", name, frame.collider.radius, frame.collider.aabb, collider.radius, collider.aabb);
				}
			}

		} else if header == v1::EXPECTED_MODEL_HEADER {
			print!("V1.3 | {:32} ", name);

			let model = v1::V1::read(&mut file).unwrap();

			println!("{:?}", model.quantities);
			println!("  {:?}", model.materials);
			println!("  {:?}", model.tag_points);

			//println!("{:?}", model);

			//println!("{:?}", legacy::Quantities::read(&mut file).unwrap());

		} else if header == v5::EXPECTED_MODEL_HEADER {
			// TODO
		} else {
			println!("unexpected header for file {}: {:?}", name, header);
		}
	}
}