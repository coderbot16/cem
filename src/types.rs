use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Pos2(pub f32, pub f32);

impl Pos2 {
	pub fn read<R>(data: &mut R) -> io::Result<Self> where R: Read {
		Ok(Pos2(
			data.read_f32::<LittleEndian>()?,
			data.read_f32::<LittleEndian>()?
		))
	}
}

#[derive(Debug)]
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