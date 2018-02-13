use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Debug, Copy, Clone)]
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
}

#[derive(Debug, Copy, Clone)]
pub struct Pos2(pub f32, pub f32);

impl Pos2 {
	pub fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Pos2(
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?
		))
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Pos3(pub f32, pub f32, pub f32);

impl Pos3 {
	pub fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Pos3(
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?
		))
	}
}

/// An axis-aligned bounding box containing a lower corner and upper corner.
#[derive(Debug)]
pub struct Aabb {
	pub lower: Pos3,
	pub upper: Pos3
}

impl Aabb {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Aabb {
			lower: Pos3::read(r)?,
			upper: Pos3::read(r)?
		})
	}
}