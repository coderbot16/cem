extern crate byteorder;

pub mod modern;
pub mod types;

use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};

/// The expected magic number for all CEM models. If this does not match, then
/// this file is almost certainly not a CEM file.
/// FCC version of "SSMF"
pub const MAGIC: u32 = 0x464D5353;

/// The header, contains the magic number and revision. The current revision is 2.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ModelHeader {
	pub magic: u32,
	pub version: u32
}

impl ModelHeader {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(ModelHeader {
			magic: r.read_u32::<LittleEndian>()?,
			version: r.read_u32::<LittleEndian>()?
		})
	}
}


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