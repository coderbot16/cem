use std::io::{self, Read, Write};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::borrow::Cow;
use cgmath::{Point2, Point3, Vector3, Matrix4};

pub trait Encode: Sized {
	fn read<R>(r: &mut R) -> io::Result<Self> where R: Read;
	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write;
}

impl Encode for String {
	fn read<T: Read>(data: &mut T) -> io::Result<Self> {
		Cow::read(data).map(Cow::into_owned)
	}

	fn write<W: Write>(&self, w: &mut W) -> io::Result<()> where W: Write {
		write_str(w, &self)
	}
}

impl<'a> Encode for Cow<'a, str> {
	fn read<T: Read>(data: &mut T) -> io::Result<Self> {
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

		Ok(Cow::Owned(string))
	}

	fn write<W: Write>(&self, w: &mut W) -> io::Result<()> where W: Write {
		write_str(w, &self)
	}
}

/// Writes a string encoded with ISO-8859-1, replacing unknown characters with a question mark ('?').
fn write_str<W: Write>(w: &mut W, s: &str) -> io::Result<()> where W: Write {
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

impl Encode for Point2<f32> {
	fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Point2 {
			x: data.read_f32::<LittleEndian>()?,
			y: data.read_f32::<LittleEndian>()?
		})
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.x)?;
		w.write_f32::<LittleEndian>(self.y)
	}
}

impl Encode for Point3<f32> {
	fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Point3 {
			x: data.read_f32::<LittleEndian>()?,
			y: data.read_f32::<LittleEndian>()?,
			z: data.read_f32::<LittleEndian>()?
		})
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.x)?;
		w.write_f32::<LittleEndian>(self.y)?;
		w.write_f32::<LittleEndian>(self.z)
	}
}

impl Encode for Vector3<f32> {
	fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Vector3 {
			x: data.read_f32::<LittleEndian>()?,
			y: data.read_f32::<LittleEndian>()?,
			z: data.read_f32::<LittleEndian>()?
		})
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.x)?;
		w.write_f32::<LittleEndian>(self.y)?;
		w.write_f32::<LittleEndian>(self.z)
	}
}

impl Encode for Matrix4<f32> {
	fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		let rows = [
			[ data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()? ],
			[ data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()? ],
			[ data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()? ],
			[ data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()?, data.read_f32::<LittleEndian>()? ]
		];

		Ok(Matrix4::new (
			rows[0][0], rows[1][0], rows[2][0], rows[3][0],
			rows[0][1], rows[1][1], rows[2][1], rows[3][1],
			rows[0][2], rows[1][2], rows[2][2], rows[3][2],
			rows[0][3], rows[1][3], rows[2][3], rows[3][3]
		))
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.x.x)?;
		w.write_f32::<LittleEndian>(self.y.x)?;
		w.write_f32::<LittleEndian>(self.z.x)?;
		w.write_f32::<LittleEndian>(self.w.x)?;

		w.write_f32::<LittleEndian>(self.x.y)?;
		w.write_f32::<LittleEndian>(self.y.y)?;
		w.write_f32::<LittleEndian>(self.z.y)?;
		w.write_f32::<LittleEndian>(self.w.y)?;

		w.write_f32::<LittleEndian>(self.x.z)?;
		w.write_f32::<LittleEndian>(self.y.z)?;
		w.write_f32::<LittleEndian>(self.z.z)?;
		w.write_f32::<LittleEndian>(self.w.z)?;

		w.write_f32::<LittleEndian>(self.x.w)?;
		w.write_f32::<LittleEndian>(self.y.w)?;
		w.write_f32::<LittleEndian>(self.z.w)?;
		w.write_f32::<LittleEndian>(self.w.w)
	}
}