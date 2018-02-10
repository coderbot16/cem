use types::{Mat4, Pos3, Pos2};
use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};
use string;

/// The expected magic number for all CEM models. If this does not match, then
/// this file is almost certainly not a CEM file.
pub const SSMF: u32 = 0x464D5353;

/// The header, contains the magic number and revision. The current revision is 2.
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

/// A single vertex. Contains the position, a relevant vertex normal, and the position on the material's texture.
#[derive(Debug)]
pub struct Vertex {
	pub position: Pos3,
	pub normal:   Pos3,
	pub texture:  Pos2
}

impl Vertex {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Vertex {
			position: Pos3::read(r)?,
			normal: Pos3::read(r)?,
			texture: Pos2::read(r)?
		})
	}
}

/// An indexed triangle. The 3 indices represent the 3 vertices that make up this triangle.
#[derive(Debug)]
pub struct Triangle(pub u32, pub u32, pub u32);

/// Selects a range of vertices or trinagles.
#[derive(Debug)]
pub struct Selection {
	pub offset: u32,
	pub len: u32
}

pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
	Ok(Selection {
		offset: r.read_u32::<LittleEndian>()?,
		len: r.read_u32::<LittleEndian>()?
	})
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

/// Contains metadata about the quantities of certain things in this file.
/// Not useful on its own, but necessary to parse the rest of the file.
#[derive(Debug)]
pub struct Quantities {
	pub tris: u32,
	pub vertices: u32,
	pub tags: u32,
	pub materials: u32,
	pub frames: u32,
	pub additional_models: u32,
	pub lod_levels: u32
}

impl Quantities {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Quantities {
			tris: r.read_u32::<LittleEndian>()?,
			vertices: r.read_u32::<LittleEndian>()?,
			tags: r.read_u32::<LittleEndian>()?,
			materials: r.read_u32::<LittleEndian>()?,
			frames: r.read_u32::<LittleEndian>()?,
			additional_models: r.read_u32::<LittleEndian>()?,
			lod_levels: r.read_u32::<LittleEndian>()?
		})
	}
}

/// A model. This contains all of the relevant sub structures.
#[derive(Debug)]
pub struct Model {
	pub quantities: Quantities,
	pub root:       Root,
	pub lod_levels: Vec<Lod>,
	pub materials:  Vec<Material>,
	pub tag_points: Vec<String>,
	pub frames:     Vec<Frame>
}

impl Model {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		let quantities = Quantities::read(r)?;
		let root = Root::read(r)?;

		let mut lod_levels = Vec::with_capacity(quantities.lod_levels as usize);
		for _ in 0..quantities.lod_levels {
			lod_levels.push(Lod::read(r)?);
		}

		let mut materials = Vec::with_capacity(quantities.materials as usize);
		for _ in 0..quantities.materials {
			materials.push(Material::read(r, quantities.lod_levels as usize)?);
		}

		let mut tag_points = Vec::with_capacity(quantities.tags as usize);
		for _ in 0..quantities.tags {
			tag_points.push(string::read_string_iso(r)?);
		}

		let mut frames = Vec::with_capacity(quantities.frames as usize);
		for _ in 0..quantities.frames {
			frames.push(Frame::read(r, quantities.vertices as usize, quantities.tags as usize)?);
		}

		Ok(Model {
			quantities,
			root,
			lod_levels,
			materials,
			tag_points,
			frames
		})
	}
}

/// A single level of detail. Contains all of the triangles for this level.
#[derive(Debug)]
pub struct Lod(pub Vec<Triangle>);

impl Lod {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		let count = r.read_u32::<LittleEndian>()?;

		let mut triangles = Vec::with_capacity(count as usize);
		for _ in 0..count {
			triangles.push(Triangle(
				r.read_u32::<LittleEndian>()?,
				r.read_u32::<LittleEndian>()?,
				r.read_u32::<LittleEndian>()?
			));
		}

		Ok(Lod(triangles))
	}
}

/// Contains the information about the root. Appears to simply define the center of the model.
#[derive(Debug)]
pub struct Root {
	/// Always "Scene Root"
	pub name: String,
	/// The point that represents the center of the model.
	pub center: Pos3
}

impl Root {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Root {
			name: string::read_string_iso(r)?,
			center: Pos3::read(r)?
		})
	}
}

/// A material to be applied to vertices. Contains special names, the texture, and target vertices / triangles.
/// The name of the material may give it special meaning depending on the context. For example, the "player color" material
/// is used to render the player color.
#[derive(Debug)]
pub struct Material {
	pub name: String,
	/// The bound texture used by this material. The texture bindings are managed externally,
	/// in most cases by a file like dbgraphics.
	pub texture: u32,
	/// The range of triangles for each LOD level.
	pub triangles: Vec<Selection>,
	/// The range of vertices used by the triangles at all LOD levels.
	pub vertices: Selection,
	pub name2: String
}

impl Material {
	pub fn read<R>(r: &mut R, lod_levels: usize) -> io::Result<Self> where R: Read {
		Ok(Material {
			name: string::read_string_iso(r)?,
			texture: r.read_u32::<LittleEndian>()?,
			triangles: {
				let mut ranges = Vec::with_capacity(lod_levels);
				for _ in 0..lod_levels {
					ranges.push(Selection::read(r)?);
				}

				ranges
			},
			vertices: Selection::read(r)?,
			name2: string::read_string_iso(r)?
		})
	}
}

/// A single frame of this model's animations. This contains the raw geometry data for the model.
/// Includes the AABB and radius for physics, the vertices, tag point positions, and a relative transform to be applied before rendering.
/// This is made up entirely of 32-bit floating point data.
#[derive(Debug)]
pub struct Frame {
	pub radius:     f32,
	pub vertices:   Vec<Vertex>,
	pub tag_points: Vec<Pos3>,
	pub transform:  Mat4,
	pub bound:      Aabb
}

impl Frame {
	pub fn read<R>(r: &mut R, vertex_count: usize, tag_point_count: usize) -> io::Result<Self> where R: Read {
		Ok(Frame{
			radius: r.read_f32::<LittleEndian>()?,
			vertices: {
				let mut vertices = Vec::with_capacity(vertex_count);
				for _ in 0..vertex_count {
					vertices.push(Vertex::read(r)?);
				}

				vertices
			},
			tag_points: {
				let mut tag_points = Vec::with_capacity(tag_point_count);
				for _ in 0..tag_point_count {
					tag_points.push(Pos3::read(r)?);
				}

				tag_points
			},
			transform: Mat4::read(r)?,
			bound: Aabb::read(r)?
		})
	}
}