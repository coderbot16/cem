extern crate byteorder;

pub mod cem;
pub mod types;

mod string {
	use byteorder::{LittleEndian, ReadBytesExt};
	use std::io::{Read, Error};

	pub fn read_string_iso<T: Read>(data: &mut T) -> Result<String, Error> {
		let len = data.read_u32::<LittleEndian>()? as usize;

		if len == 0 {
			return Ok(String::new());
		}

		let mut string = String::with_capacity(len);

		for _ in 0..len-1 {
			let byte = data.read_u8()?;

			if byte==0 {
				continue;
			}

			string.push(byte as char);
		}

		assert_eq!(data.read_u8()?, 0);

		Ok(string)
	}
}