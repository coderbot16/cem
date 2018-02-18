use std::io::{self, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4(pub [f32; 16]);

impl Mat4 {
	pub fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Mat4([
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?
		]))
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.0[0])?;
		w.write_f32::<LittleEndian>(self.0[1])?;
		w.write_f32::<LittleEndian>(self.0[2])?;
		w.write_f32::<LittleEndian>(self.0[3])?;

		w.write_f32::<LittleEndian>(self.0[4])?;
		w.write_f32::<LittleEndian>(self.0[5])?;
		w.write_f32::<LittleEndian>(self.0[6])?;
		w.write_f32::<LittleEndian>(self.0[7])?;

		w.write_f32::<LittleEndian>(self.0[8])?;
		w.write_f32::<LittleEndian>(self.0[9])?;
		w.write_f32::<LittleEndian>(self.0[10])?;
		w.write_f32::<LittleEndian>(self.0[11])?;

		w.write_f32::<LittleEndian>(self.0[12])?;
		w.write_f32::<LittleEndian>(self.0[13])?;
		w.write_f32::<LittleEndian>(self.0[14])?;
		w.write_f32::<LittleEndian>(self.0[15])
	}

	pub fn is_identity(&self) -> bool {
		*self == Self::default()
	}
}

impl Default for Mat4 {
	fn default() -> Self {
		Mat4([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Pos2(pub f32, pub f32);

impl Pos2 {
	pub fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Pos2(
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?
		))
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.0)?;
		w.write_f32::<LittleEndian>(self.1)
	}
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Pos3(pub f32, pub f32, pub f32);

impl Pos3 {
	pub fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Pos3(
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?
		))
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.0)?;
		w.write_f32::<LittleEndian>(self.1)?;
		w.write_f32::<LittleEndian>(self.2)
	}
}