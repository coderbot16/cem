extern crate byteorder;

pub mod v1;
pub mod v2;
pub mod types;

use std::io::{self, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

/// The expected magic number for all CEM models. If this does not match, then
/// this file is almost certainly not a CEM file.
/// FCC version of "SSMF"
pub const MAGIC: u32 = 0x464D5353;

/// The header, contains the magic number and revision. The current revision is 2.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ModelHeader {
	pub magic: u32,
	pub major: u16,
	pub minor: u16
}

impl ModelHeader {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(ModelHeader {
			magic: r.read_u32::<LittleEndian>()?,
			major: r.read_u16::<LittleEndian>()?,
			minor: r.read_u16::<LittleEndian>()?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_u32::<LittleEndian>(self.magic)?;
		w.write_u16::<LittleEndian>(self.major)?;
		w.write_u16::<LittleEndian>(self.minor)
	}
}


mod string {
	use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
	use std::io::{self, Read, Write};

	pub fn read_string_iso<T: Read>(data: &mut T) -> io::Result<String> {
		let len = data.read_u32::<LittleEndian>()? as usize;
		let mut string = String::with_capacity(len);
		let mut end = false;

		for _ in 0..len {
			let byte = data.read_u8()?;

			if byte==0 {
				end = true;
			}

			if !end {
				string.push(byte as char);
			}
		}

		Ok(string)
	}

	/// Writes a string encoded with ISO-8859-1, replacing unknown characters with a question mark ('?').
	pub fn write_string_iso<W: Write>(w: &mut W, s: &str) -> io::Result<()> where W: Write {
		let s = s.trim_right_matches('\0');
		let len = s.chars().count() + 1;

		if len > u32::max_value() as usize {
			return Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot write a string more than 4GB long"));
		}

		w.write_u32::<LittleEndian>(len as u32)?;

		for char in s.chars() {
			// Simply replace all unknown chars with a ?, as an encoding error.

			w.write_u8(if char < '\u{256}' { char as u8 } else { '?' as u8 })?;
		}

		w.write_u8(0)
	}
}