extern crate byteorder;
extern crate cgmath;

pub mod scene;

/// V1 model format. Found rarely in Empire Earth 1, but not the native format of any released game.
pub mod v1;

/// V2 model format. Primarily found in Empire Earth 1.
pub mod v2;

// NOTE: I have not yet found v3 and v4 models in the wild yet.
// However, they may be present in other games running on the Titan Engine.

/// V5 model format. Primarily found in Empires: Dawn of the Modern World.
pub mod v5;

pub mod collider;

mod encode;

use std::io::{self, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

/// The expected magic number for all CEM models. If this does not match, then
/// this file is almost certainly not a CEM file.
/// FCC version of "SSMF"
pub const MAGIC: u32 = 0x464D5353;

pub use v1::V1;
pub use v2::V2;
pub use v5::V5;
pub use encode::Encode;
pub use scene::{Scene, Model};

// TODO: We should implement something comparable to the Edge Collapse
// LOD generation that Titan uses.

/// The header, contains the magic number and revision. The current revision is 2.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ModelHeader {
	pub magic: u32,
	pub major: u16,
	pub minor: u16
}

impl Encode for ModelHeader {
	fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(ModelHeader {
			magic: r.read_u32::<LittleEndian>()?,
			major: r.read_u16::<LittleEndian>()?,
			minor: r.read_u16::<LittleEndian>()?
		})
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_u32::<LittleEndian>(self.magic)?;
		w.write_u16::<LittleEndian>(self.major)?;
		w.write_u16::<LittleEndian>(self.minor)
	}
}