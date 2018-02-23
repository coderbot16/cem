use cgmath::{Point2, Point3, Vector3, Matrix4, SquareMatrix};
use collider::Aabb;
use std::io::{self, Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use {ModelHeader, MAGIC, Encode};
use collider::{Collider, ColliderBuilder};
use scene::{NodeData, Model};
use std::borrow::Cow;

pub type VertexIndex = u32;

/// Contains metadata about the quantities of certain things in this file.
/// Not useful on its own, but necessary to parse the rest of the file.
#[derive(Debug)]
struct Quantities {
	triangles: u32,
	vertices: u32,
	tag_points: u32,
	materials: u32,
	frames: u32,
	additional_models: u32,
	lod_levels: u32
}

impl Quantities {
	fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Quantities {
			triangles:         r.read_u32::<LittleEndian>()?,
			vertices:          r.read_u32::<LittleEndian>()?,
			tag_points:        r.read_u32::<LittleEndian>()?,
			materials:         r.read_u32::<LittleEndian>()?,
			frames:            r.read_u32::<LittleEndian>()?,
			additional_models: r.read_u32::<LittleEndian>()?,
			lod_levels:        r.read_u32::<LittleEndian>()?
		})
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_u32::<LittleEndian>(self.triangles)?;
		w.write_u32::<LittleEndian>(self.vertices)?;
		w.write_u32::<LittleEndian>(self.tag_points)?;
		w.write_u32::<LittleEndian>(self.materials)?;
		w.write_u32::<LittleEndian>(self.frames)?;
		w.write_u32::<LittleEndian>(self.additional_models)?;
		w.write_u32::<LittleEndian>(self.lod_levels)
	}
}

/// A model. This contains all of the relevant sub structures.
#[derive(Debug)]
pub struct V2 {
	pub center:            Point3<f32>,
	pub lod_levels:        Vec<Vec<(VertexIndex, VertexIndex, VertexIndex)>>,
	pub materials:         Vec<Material>,
	pub tag_points:        Vec<String>,
	pub frames:            Vec<Frame>
}

impl V2 {
	fn quantities(&self, additional_models: u32) -> Result<Quantities, &'static str> {
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
			triangles:         self.lod_levels[0].len() as u32,
			vertices:          self.frames[0].vertices.len() as u32,
			tag_points:        self.tag_points.len() as u32,
			materials:         self.materials.len() as u32,
			frames:            self.frames.len() as u32,
			additional_models,
			lod_levels:        self.lod_levels.len() as u32
		})
	}
}

impl Model for V2 {
	const HEADER: ModelHeader = ModelHeader { magic: MAGIC, major: 2, minor: 0 };

	fn read<R>(r: &mut R) -> io::Result<(Self, NodeData)> where R: Read {
		let quantities = Quantities::read(r)?;

		let node = NodeData {
			additional_models: quantities.additional_models,
			name: Cow::Owned(String::read(r)?)
		};

		let center = Point3::read(r)?;

		let mut lod_levels = Vec::with_capacity(quantities.lod_levels as usize);
		for _ in 0..lod_levels.capacity() {
			let count = r.read_u32::<LittleEndian>()?;

			let mut triangles = Vec::with_capacity(count as usize);
			for _ in 0..count {
				triangles.push((
					r.read_u32::<LittleEndian>()?,
					r.read_u32::<LittleEndian>()?,
					r.read_u32::<LittleEndian>()?
				));
			}

			lod_levels.push(triangles);
		}

		let mut materials = Vec::with_capacity(quantities.materials as usize);
		for _ in 0..materials.capacity() {
			materials.push(Material::read(r, lod_levels.len())?);
		}

		let mut tag_points = Vec::with_capacity(quantities.tag_points as usize);
		for _ in 0..tag_points.capacity() {
			tag_points.push(String::read(r)?);
		}

		let mut frames = Vec::with_capacity(quantities.frames as usize);
		for _ in 0..frames.capacity() {
			frames.push(Frame::read(r, quantities.vertices as usize, tag_points.len())?);
		}

		Ok((V2 {
			center,
			lod_levels,
			materials,
			tag_points,
			frames
		}, node ))
	}

	fn write<W>(&self, w: &mut W, node: NodeData) -> io::Result<()> where W: Write {
		let quantities = self.quantities(node.additional_models).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		quantities.write(w)?;

		node.name.write(w)?;
		self.center.write(w)?;

		for triangles in &self.lod_levels {
			w.write_u32::<LittleEndian>(triangles.len() as u32)?;

			for triangle in triangles {
				w.write_u32::<LittleEndian>(triangle.0)?;
				w.write_u32::<LittleEndian>(triangle.1)?;
				w.write_u32::<LittleEndian>(triangle.2)?;
			}
		}

		for material in &self.materials {
			material.write(w)?;
		}

		for tag_point in &self.tag_points {
			tag_point.write(w)?;
		}

		for frame in &self.frames {
			frame.write(w)?;
		}

		Ok(())
	}
}

/// A material to be applied to vertices. Contains special names, the texture, and target vertices / triangles.
/// The name of the material may give it special meaning depending on the context. For example, the "player color" material
/// is used to render the player color.
#[derive(Debug)]
pub struct Material {
	/// A name. Empire Earth does not appear to care about the value.
	pub name: String,
	/// The bound texture used by this material. The texture bindings are managed externally,
	/// in most cases by a file like dbgraphics.
	/// Note: the true meaning of this value is unknown, and 0 seems to work.
	pub texture: u32,
	/// The range of triangles for each LOD level.
	pub triangles: Vec<TriangleSelection>,
	/// Value that the vertex index of
	pub vertex_offset: VertexIndex,
	pub vertex_count: u32,
	/// Name of the texture used. Empire Earth does not appear to care about the value.
	pub texture_name: String
}

impl Material {
	pub fn read<R>(r: &mut R, lod_levels: usize) -> io::Result<Self> where R: Read {
		Ok(Material {
			name: String::read(r)?,
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
			texture_name: String::read(r)?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		self.name.write(w)?;
		w.write_u32::<LittleEndian>(self.texture)?;

		for selection in &self.triangles {
			selection.write(w)?;
		}

		w.write_u32::<LittleEndian>(self.vertex_offset)?;
		w.write_u32::<LittleEndian>(self.vertex_count)?;
		self.texture_name.write(w)?;

		Ok(())
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

/// A single frame of this model's animations. This contains the raw geometry data for the model.
/// Includes the AABB and radius for physics, the vertices, tag point positions, and a relative transform to be applied before rendering.
/// This is made up entirely of 32-bit floating point data.
#[derive(Debug)]
pub struct Frame {
	pub vertices:   Vec<Vertex>,
	pub tag_points: Vec<Point3<f32>>,
	pub transform:  Matrix4<f32>,
	pub collider:   Collider
}

impl Frame {
	pub fn from_vertices(vertices: Vec<Vertex>, tag_points: Vec<Point3<f32>>, center: Point3<f32>) -> Self {
		let mut builder = ColliderBuilder::begin(center);

		for vertex in &vertices {
			builder.update(vertex.position);
		}

		let collider = builder.build();

		Frame {
			vertices,
			tag_points,
			transform: Matrix4::identity(),
			collider
		}
	}

	pub fn read<R>(r: &mut R, vertex_count: usize, tag_point_count: usize) -> io::Result<Self> where R: Read {
		let radius = r.read_f32::<LittleEndian>()?;

		Ok(Frame {
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
					tag_points.push(Point3::read(r)?);
				}

				tag_points
			},
			transform: Matrix4::read(r)?,
			collider: Collider {
				radius,
				aabb: Aabb::read(r)?
			}
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		w.write_f32::<LittleEndian>(self.collider.radius)?;

		for vertex in &self.vertices {
			vertex.write(w)?;
		}

		for tag_point in &self.tag_points {
			tag_point.write(w)?;
		}

		self.transform.write(w)?;
		self.collider.aabb.write(w)
	}
}

/// A single vertex. Contains the position, a relevant vertex normal, and the position on the material's texture.
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
	pub position: Point3<f32>,
	pub normal:   Vector3<f32>,
	pub texture:  Point2<f32>
}

impl Vertex {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Vertex {
			position: Point3::read(r)?,
			normal: Vector3::read(r)?,
			texture: Point2::read(r)?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		self.position.write(w)?;
		self.normal.write(w)?;
		self.texture.write(w)
	}
}