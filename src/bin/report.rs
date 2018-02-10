extern crate cem;

use cem::ModelHeader;
use cem::modern::{Model, EXPECTED_MODEL_HEADER};
use std::io::BufReader;

fn main() {
	for path in ::std::fs::read_dir("/home/coderbot/Programming/Java/EmpireEarthReverse/extract/data/models").unwrap() {
		let path = path.unwrap().path();

		let name = path.to_string_lossy().to_string();
		let mut file = BufReader::new(::std::fs::File::open(path).unwrap());

		let header = ModelHeader::read(&mut file).unwrap();

		if header.magic != EXPECTED_MODEL_HEADER.magic {
			println!("{} is not a CEM model: unexpected magic value {}", name, header.magic);
			continue;
		}

		if header.version != EXPECTED_MODEL_HEADER.version {
			println!("unexpected version for file {}: {}", name, header.version);
			continue;
		}

		let _model = Model::read(&mut file).unwrap();
	}
}