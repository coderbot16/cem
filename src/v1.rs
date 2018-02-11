use types::{Mat4, Pos3, Aabb};
use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};
use ::{string, ModelHeader, MAGIC};

// 1.1
// 	Adds the TagPoints chunk
//  Adds the tag_points count in Root.
// 1.2
//  Adds the sub_models count in Root.
// 1.3
//  Adds the array to MiscChunk.

/// Expected: SSMF v1.3
pub const EXPECTED_MODEL_HEADER: ModelHeader = ModelHeader { magic: MAGIC, major: 1, minor: 3 };

#[derive(Debug)]
pub struct Model {
	pub quantities: Quantities,
	pub root: Root,
	pub triangles: Vec<(Vertex, Vertex, Vertex)>,
	pub triangle_groups: Vec<TriangleGroup>,
	pub materials: Vec<Material>,
	pub vertices: Vec<(u32, f32)>,
	pub tag_points: Vec<String>,
	pub frames: Vec<Frame>
}

impl Model {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		let quantities = Quantities::read(r)?;

		Ok(Model {
			root: Root::read(r, quantities.vertex_points)?,
			triangles: {
				let mut triangles = Vec::with_capacity(quantities.triangles as usize);

				for _ in 0..quantities.triangles {
					triangles.push((
						Vertex::read(r)?,
						Vertex::read(r)?,
						Vertex::read(r)?
					));
				}

				triangles
			},
			triangle_groups: {
				let mut triangle_groups = Vec::with_capacity(quantities.triangle_groups as usize);

				for _ in 0..quantities.triangle_groups {
					triangle_groups.push(TriangleGroup::read(r)?);
				}

				triangle_groups
			},
			materials: {
				let mut materials = Vec::with_capacity(quantities.materials as usize);

				for _ in 0..quantities.materials {
					materials.push(Material::read(r)?);
				}

				materials
			},
			vertices: {
				let mut vertices = Vec::with_capacity(quantities.vertices as usize);

				for _ in 0..quantities.vertices {
					vertices.push((
						r.read_u32::<LittleEndian>()?,
						r.read_f32::<LittleEndian>()?
					));
				}

				vertices
			},
			tag_points: {
				let mut tag_points = Vec::with_capacity(quantities.tag_points as usize);

				for _ in 0..quantities.tag_points {
					tag_points.push(string::read_string_iso(r)?);
				}

				tag_points
			},
			frames: {
				let mut frames = Vec::with_capacity(quantities.frames as usize);

				for _ in 0..quantities.frames {
					frames.push(Frame::read(r, &quantities)?);
				}

				frames
			},
			quantities
		})
	}
}

/// Contains metadata about the quantities of certain things in this file.
/// Not useful on its own, but necessary to parse the rest of the file.
#[derive(Debug)]
pub struct Quantities {
	pub frames:  u32,
	pub materials:  u32,
	/// Number of individual vertex points. This only represents the count of unique position components.
	pub vertex_points:  u32,
	pub triangles: u32,
	pub triangle_groups:  u32,
	/// Count of unique vertices, including vertex normals and texture positions. Should always be >= vertex_points.
	pub vertices:  u32,
	pub tag_points:  u32,
	pub additional_models:  u32
}

impl Quantities {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Quantities {
			frames:  r.read_u32::<LittleEndian>()?,
			materials:  r.read_u32::<LittleEndian>()?,
			vertex_points:  r.read_u32::<LittleEndian>()?,
			triangles: r.read_u32::<LittleEndian>()?,
			triangle_groups:  r.read_u32::<LittleEndian>()?,
			vertices:  r.read_u32::<LittleEndian>()?,
			tag_points:  r.read_u32::<LittleEndian>()?,
			additional_models:  r.read_u32::<LittleEndian>()?
		})
	}
}

/// Contains the information about the root. Appears to simply define the center of the model.
#[derive(Debug)]
pub struct Root {
	/// Name of this sub-model, or just "Scene Root" if this is the root model.
	pub name: String,
	/// The point that represents the center of the model.
	pub center: Pos3,
	pub unknown: u8,
	pub points: Vec<u32>
}

impl Root {
	pub fn read<R>(r: &mut R, vertex_points: u32) -> io::Result<Self> where R: Read {
		Ok(Root {
			name: string::read_string_iso(r)?,
			center: Pos3::read(r)?,
			unknown: r.read_u8()?,
			points: {
				let mut points = Vec::with_capacity(vertex_points as usize);

				for _ in 0..vertex_points {
					points.push(r.read_u32::<LittleEndian>()?);
				}

				points
			}
		})
	}
}

#[derive(Debug)]
pub struct Vertex {
	pub unknown0: u32,
	pub uv: (f32, f32),
	pub rgb: (f32, f32, f32),
	// Unknown, seems to be constant throughout the file
	pub unknown1: [f32; 4]
}

impl Vertex {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Vertex {
			unknown0: r.read_u32::<LittleEndian>()?,
			uv: (
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?
			),
			rgb: (
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?
			),
			unknown1: [
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?,
				r.read_f32::<LittleEndian>()?
			]
		})
	}
}

#[derive(Debug)]
pub struct TriangleGroup {
	name: String,
	indices: Vec<u32>
}

impl TriangleGroup {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(TriangleGroup {
			name: string::read_string_iso(r)?,
			indices: {
				let len = r.read_u32::<LittleEndian>()?;
				let mut indices = Vec::with_capacity(len as usize);

				for _ in 0..len {
					indices.push(r.read_u32::<LittleEndian>()?);
				}

				indices
			}
		})
	}
}

#[derive(Debug)]
pub struct Material {
	pub indices: Vec<u32>,
	/// Second value has an unknown meaning.
	pub texture: Option<(String, u32)>
}

impl Material {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Material {
			indices: {
				let len = r.read_u32::<LittleEndian>()?;
				let mut indices = Vec::with_capacity(len as usize);

				for _ in 0..len {
					indices.push(r.read_u32::<LittleEndian>()?);
				}

				indices
			},
			texture: match r.read_u8()? {
				0 => None,
				1 => Some((string::read_string_iso(r)?, r.read_u32::<LittleEndian>()?)),
				x => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("A boolean must be 0 or 1, got {}", x)))
			}
		})
	}
}

#[derive(Debug)]
pub struct Frame {
	pub radius:           f32,
	pub points:           Vec<Pos3>,
	pub normals:          Vec<u16>,
	// pub triangle_normals: Vec<Pos3>, // Removed in v1.3
	pub tag_points:       Vec<Pos3>,
	pub transform:        Mat4,
	pub bound:            Aabb
}

impl Frame {
	pub fn read<R>(r: &mut R, quantities: &Quantities) -> io::Result<Self> where R: Read {
		Ok(Frame {
			radius: r.read_f32::<LittleEndian>()?,
			points: {
				let mut points = Vec::with_capacity(quantities.vertex_points as usize);

				for _ in 0..quantities.vertex_points {
					points.push(Pos3::read(r)?);
				}

				points
			},
			normals: {
				let mut normals = Vec::with_capacity(quantities.vertices as usize);

				for _ in 0..quantities.vertices {
					normals.push(r.read_u16::<LittleEndian>()?);
				}

				normals
			},
			/*triangle_normals: {
				let mut triangle_normals = Vec::with_capacity(quantities.triangles as usize);

				for _ in 0..quantities.triangles {
					triangle_normals.push(Pos3::read(r)?);
				}

				triangle_normals
			},*/
			tag_points: {
				let mut tag_points = Vec::with_capacity(quantities.tag_points as usize);

				for _ in 0..quantities.tag_points {
					tag_points.push(Pos3::read(r)?);
				}

				tag_points
			},
			transform: Mat4::read(r)?,
			bound: Aabb::read(r)?
		})
	}
}