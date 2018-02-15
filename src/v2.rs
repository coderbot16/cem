use types::{Mat4, Pos3, Pos2, Aabb};
use std::io::{self, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use ::{string, ModelHeader, MAGIC};

/// Expected: SSMF v2.0
pub const EXPECTED_MODEL_HEADER: ModelHeader = ModelHeader { magic: MAGIC, major: 2, minor: 0 };

/// Contains metadata about the quantities of certain things in this file.
/// Not useful on its own, but necessary to parse the rest of the file.
#[derive(Debug)]
pub struct Quantities {
	pub triangles: u32,
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
			triangles:         r.read_u32::<LittleEndian>()?,
			vertices:          r.read_u32::<LittleEndian>()?,
			tags:              r.read_u32::<LittleEndian>()?,
			materials:         r.read_u32::<LittleEndian>()?,
			frames:            r.read_u32::<LittleEndian>()?,
			additional_models: r.read_u32::<LittleEndian>()?,
			lod_levels:        r.read_u32::<LittleEndian>()?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_u32::<LittleEndian>(self.triangles)?;
		w.write_u32::<LittleEndian>(self.vertices)?;
		w.write_u32::<LittleEndian>(self.tags)?;
		w.write_u32::<LittleEndian>(self.materials)?;
		w.write_u32::<LittleEndian>(self.frames)?;
		w.write_u32::<LittleEndian>(self.additional_models)?;
		w.write_u32::<LittleEndian>(self.lod_levels)
	}
}

pub type VertexIndex = u32;

/// An indexed triangle. The 3 indices represent the 3 vertices that make up this triangle.
#[derive(Debug)]
pub struct Triangle(pub VertexIndex, pub VertexIndex, pub VertexIndex);

impl Triangle {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Triangle(
			r.read_u32::<LittleEndian>()?,
			r.read_u32::<LittleEndian>()?,
			r.read_u32::<LittleEndian>()?
		))
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_u32::<LittleEndian>(self.0)?;
		w.write_u32::<LittleEndian>(self.1)?;
		w.write_u32::<LittleEndian>(self.2)
	}
}

/// Selects a range of triangles.
#[derive(Debug, Copy, Clone)]
pub struct TriangleSelection {
	pub offset: u32,
	pub len: u32
}

impl TriangleSelection {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(TriangleSelection {
			offset: r.read_u32::<LittleEndian>()?,
			len: r.read_u32::<LittleEndian>()?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_u32::<LittleEndian>(self.offset)?;
		w.write_u32::<LittleEndian>(self.len)
	}
}

/// A model. This contains all of the relevant sub structures.
#[derive(Debug)]
pub struct Model {
	pub triangles:         u32,
	pub additional_models: u32,
	pub root:              Root,
	pub lod_levels:        Vec<Vec<Triangle>>,
	pub materials:         Vec<Material>,
	pub tag_points:        Vec<String>,
	pub frames:            Vec<Frame>
}

impl Model {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		let quantities = Quantities::read(r)?;
		let root = Root::read(r)?;

		let mut lod_levels = Vec::with_capacity(quantities.lod_levels as usize);
		for _ in 0..lod_levels.capacity() {
			let count = r.read_u32::<LittleEndian>()?;

			let mut triangles = Vec::with_capacity(count as usize);
			for _ in 0..count {
				triangles.push(Triangle::read(r)?);
			}

			lod_levels.push(triangles);
		}

		let mut materials = Vec::with_capacity(quantities.materials as usize);
		for _ in 0..materials.capacity() {
			materials.push(Material::read(r, lod_levels.len())?);
		}

		let mut tag_points = Vec::with_capacity(quantities.tags as usize);
		for _ in 0..tag_points.capacity() {
			tag_points.push(string::read_string_iso(r)?);
		}

		let mut frames = Vec::with_capacity(quantities.frames as usize);
		for _ in 0..frames.capacity() {
			frames.push(Frame::read(r, quantities.vertices as usize, tag_points.len())?);
		}

		Ok(Model {
			triangles: quantities.triangles,
			additional_models: quantities.additional_models,
			root,
			lod_levels,
			materials,
			tag_points,
			frames
		})
	}

	pub fn quantities(&self) -> Result<Quantities, &'static str> {
		if self.materials.len() == 0 {
			return Err("A model must have at least 1 material");
		}

		if self.lod_levels.len() == 0 {
			return Err("A model must have at least 1 LOD level");
		}

		if self.frames.len() == 0 {
			return Err("A model must have at least 1 frame")
		}

		Ok(Quantities {
			triangles:         self.triangles,
			vertices:          self.frames[0].vertices.len() as u32,
			tags:              self.tag_points.len() as u32,
			materials:         self.materials.len() as u32,
			frames:            self.frames.len() as u32,
			additional_models: self.additional_models,
			lod_levels:        self.lod_levels.len() as u32
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		let quantities = self.quantities().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		quantities.write(w)?;
		self.root.write(w)?;

		for triangles in &self.lod_levels {
			w.write_u32::<LittleEndian>(triangles.len() as u32)?;

			for triangle in triangles {
				triangle.write(w)?;
			}
		}

		for material in &self.materials {
			material.write(w)?;
		}

		for tag_point in &self.tag_points {
			string::write_string_iso(w, tag_point)?;
		}

		for frame in &self.frames {
			frame.write(w)?;
		}

		Ok(())
	}
}

/// Contains the information about the root. Appears to simply define the center of the model.
#[derive(Debug)]
pub struct Root {
	/// Name of this sub-model, or just "Scene Root" if this is the root model.
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

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		string::write_string_iso(w, &self.name)?;
		self.center.write(w)
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
	pub triangles: Vec<TriangleSelection>,
	/// Value that the vertex index of
	pub vertex_offset: VertexIndex,
	pub vertex_count: u32,
	/// Name of the texture used.
	pub texture_name: String
}

impl Material {
	pub fn read<R>(r: &mut R, lod_levels: usize) -> io::Result<Self> where R: Read {
		Ok(Material {
			name: string::read_string_iso(r)?,
			texture: r.read_u32::<LittleEndian>()?,
			triangles: {
				let mut ranges = Vec::with_capacity(lod_levels);
				for _ in 0..lod_levels {
					ranges.push(TriangleSelection::read(r)?);
				}

				ranges
			},
			vertex_offset: r.read_u32::<LittleEndian>()?,
			vertex_count: r.read_u32::<LittleEndian>()?,
			texture_name: string::read_string_iso(r)?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		string::write_string_iso(w, &self.name)?;
		w.write_u32::<LittleEndian>(self.texture)?;

		for selection in &self.triangles {
			selection.write(w)?;
		}

		w.write_u32::<LittleEndian>(self.vertex_offset)?;
		w.write_u32::<LittleEndian>(self.vertex_count)?;
		string::write_string_iso(w, &self.texture_name)?;

		Ok(())
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

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.radius)?;

		for vertex in &self.vertices {
			vertex.write(w)?;
		}

		for tag_point in &self.tag_points {
			tag_point.write(w)?;
		}

		self.transform.write(w)?;
		self.bound.write(w)
	}
}

/// A single vertex. Contains the position, a relevant vertex normal, and the position on the material's texture.
#[derive(Debug, Copy, Clone)]
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

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		self.position.write(w)?;
		self.normal.write(w)?;
		self.texture.write(w)
	}
}