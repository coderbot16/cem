extern crate cem;

use cem::ModelHeader;
use cem::v2;
use cem::v1;
use std::io::BufReader;

fn main() {
	for entry in ::std::fs::read_dir("/home/coderbot/Programming/Java/EmpireEarthReverse/extract/data/models").unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		let name = entry.file_name().to_string_lossy().to_string();

		let mut file = BufReader::new(::std::fs::File::open(path).unwrap());

		let header = ModelHeader::read(&mut file).unwrap();

		if header == v2::EXPECTED_MODEL_HEADER {

			let _model = v2::Model::read(&mut file).unwrap();

		} else if header == v1::EXPECTED_MODEL_HEADER {
			print!("V1.3 | {:32} ", name);

			let model = v1::Model::read(&mut file).unwrap();

			println!("{:?}", model.quantities);
			println!("  {:?}", model.materials);
			println!("  {:?}", model.tag_points);

			//println!("{:?}", model);

			//println!("{:?}", legacy::Quantities::read(&mut file).unwrap());

		} else {
			println!("unexpected header for file {}: {:?}", name, header);
		}
	}
}