use std::io::{self, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {ModelHeader, Model, MAGIC, v2, Encode};
use cgmath::Point3;
use scene::NodeData;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Quantities {
	unknown0: u32, // Probably total vertices / dynamic vertices
	unknown1: u32, // Probably common vertices
	tag_points: u32,
	materials: u32,
	frames: u32,
	additional_models: u32,
	lod_levels: u32,
	points: u32
}

impl Quantities {
	fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Quantities {
			unknown0:          r.read_u32::<LittleEndian>()?,
			unknown1:          r.read_u32::<LittleEndian>()?,
			tag_points:        r.read_u32::<LittleEndian>()?,
			materials:         r.read_u32::<LittleEndian>()?,
			frames:            r.read_u32::<LittleEndian>()?,
			additional_models: r.read_u32::<LittleEndian>()?,
			lod_levels:        r.read_u32::<LittleEndian>()?,
			points:          r.read_u32::<LittleEndian>()?
		})
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_u32::<LittleEndian>(self.unknown0)?;
		w.write_u32::<LittleEndian>(self.unknown1)?;
		w.write_u32::<LittleEndian>(self.tag_points)?;
		w.write_u32::<LittleEndian>(self.materials)?;
		w.write_u32::<LittleEndian>(self.frames)?;
		w.write_u32::<LittleEndian>(self.additional_models)?;
		w.write_u32::<LittleEndian>(self.lod_levels)?;
		w.write_u32::<LittleEndian>(self.points)
	}
}

#[derive(Debug)]
pub struct V5 {
	pub quantities: Quantities,
	pub center: Point3<f32>,
	pub common_vertices: Vec<CommonVertex>,
	pub lod_levels: Vec<Vec<(u16, u16, u16)>>,
	pub materials: Vec<v2::Material>,
	pub tag_points: Vec<String>,
	pub frames: Vec<Frame>,
	pub points: Vec<Point3<f32>>,
	pub shadow: Vec<ShadowEdge>
}

impl V5 {
	fn quantities(&self, additional_models: u32) -> Result<Quantities, &'static str> {
		let mut quantities = self.quantities;

		quantities.additional_models = additional_models;

		Ok(quantities)
	}
}

impl Model for V5 {
	const HEADER: ModelHeader = ModelHeader { magic: MAGIC, major: 5, minor: 0 };

	fn read<R>(r: &mut R) -> io::Result<(Self, NodeData)> where R: Read {
		let quantities = Quantities::read(r)?;
		let lod_levels = quantities.lod_levels as usize;

		let node = NodeData {
			additional_models: quantities.additional_models,
			name: Cow::Owned(String::read(r)?)
		};

		Ok((V5 {
			center: Point3::read(r)?,
			common_vertices: {
				let len = r.read_u32::<LittleEndian>()?;
				let mut common_vertices = Vec::with_capacity(len as usize);

				for _ in 0..len {
					common_vertices.push(CommonVertex::read(r)?);
				}

				common_vertices
			},
			lod_levels: {
				let mut lod_levels = Vec::with_capacity(quantities.lod_levels as usize);
				for _ in 0..lod_levels.capacity() {
					let count = r.read_u32::<LittleEndian>()?;

					let mut triangles = Vec::with_capacity(count as usize);
					for _ in 0..count {
						triangles.push((
							r.read_u16::<LittleEndian>()?,
							r.read_u16::<LittleEndian>()?,
							r.read_u16::<LittleEndian>()?
						));
					}

					lod_levels.push(triangles);
				}

				lod_levels
			},
			materials: {
				let mut materials = Vec::with_capacity(quantities.materials as usize);

				for _ in 0..quantities.materials {
					materials.push(v2::Material::read(r, lod_levels)?);
				}

				materials
			},
			tag_points: {
				let mut tag_points = Vec::with_capacity(quantities.tag_points as usize);

				for _ in 0..quantities.tag_points {
					tag_points.push(String::read(r)?);
				}

				tag_points
			},
			frames: {
				let mut frames = Vec::with_capacity(quantities.frames as usize);

				for _ in 0..quantities.frames {
					frames.push(Frame::read(r)?);
				}

				frames
			},
			points: {
				let mut points = Vec::with_capacity(quantities.points as usize);

				for _ in 0..quantities.points {
					points.push(Point3::read(r)?);
				}

				points
			},
			shadow: {
				let len = r.read_u32::<LittleEndian>()?;
				let mut edges = Vec::with_capacity(len as usize);

				for _ in 0..len {
					edges.push(ShadowEdge::read(r)?);
				}

				edges
			},
			quantities
		}, node))
	}

	fn write<W>(&self, w: &mut W, node: NodeData) -> io::Result<()> where W: Write {
		let quantities = self.quantities(node.additional_models).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		quantities.write(w)?;

		node.name.write(w)?;
		self.center.write(w)?;

		// Then common_vertices and the rest

		unimplemented!()
	}
}

#[derive(Debug)]
pub struct CommonVertex {
	pub unknown0: [f32; 16],
	pub unknown1: i32
}

impl CommonVertex {
	fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(CommonVertex {
			unknown0: [
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?
			],
			unknown1: r.read_i32::<LittleEndian>()?
		})
	}
}

#[derive(Debug)]
pub struct Frame {
	radius: f32
	// TODO: Vertices, TagPoints, Mat4, Aabb, BumpMap
}

impl Frame {
	fn read<R>(_r: &mut R) -> io::Result<Self> where R: Read {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct ShadowEdge {
	unknown0: u32,
	unknown1: [u16; 4]
}

impl ShadowEdge {
	fn read<R>(_r: &mut R) -> io::Result<Self> where R: Read {
		unimplemented!()
	}
}