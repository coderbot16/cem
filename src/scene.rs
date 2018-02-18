use std::io::{self, Read, Write};
use std::borrow::Cow;
use ModelHeader;

pub struct NodeData<'a> {
	pub additional_models: u32,
	pub name: Cow<'a, str>
}

pub struct Scene<M: Model> {
	pub name:     String,
	pub model:    M,
	pub children: Vec<Scene<M>>
}

impl<M: Model> Scene<M> {
	pub fn root(model: M) -> Self {
		Scene {
			name: "Scene Root".to_string(),
			model,
			children: Vec::new()
		}
	}

	pub fn single(name: String, model: M) -> Self {
		Scene {
			name,
			model,
			children: Vec::new()
		}
	}

	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		let header = ModelHeader::read(r)?;

		if header != M::HEADER {
			return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Wrong model header: expected {:?}, got {:?}", M::HEADER, header)));
		}

		Self::read_without_header(r)
	}

	pub fn read_without_header<R>(r: &mut R) -> io::Result<Self> where R: Read {
		let (mut scene, additional_models) = {
			let (model, node) = M::read(r)?;

			let scene = Scene::single(node.name.into_owned(), model);

			(scene, node.additional_models)
		};

		for _ in 0..additional_models {
			scene.children.push(Scene::read(r)?);
		}

		Ok(scene)
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		let node = NodeData {
			additional_models: self.children.len() as u32,
			name: Cow::Borrowed(&self.name)
		};

		M::HEADER.write(w)?;
		self.model.write(w, node)?;

		for child in &self.children {
			child.write(w)?;
		}

		Ok(())
	}
}

pub trait Model: Sized {
	const HEADER: ModelHeader;

	fn read<R>(r: &mut R) -> io::Result<(Self, NodeData)> where R: Read;
	fn write<W>(&self, w: &mut W, data: NodeData) -> io::Result<()> where W: Write;
}

/*/// Contains information about this node of the scene.
#[derive(Debug)]
pub struct SceneNode {
	/// Name of this sub-model, or just "Scene SceneNode" if this is the root model.
	pub name: String,
	/// The point that represents the center of the model.
	pub center: Pos3
}

impl SceneNode {
	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(SceneNode {
			name: string::read_string_iso(r)?,
			center: Pos3::read(r)?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		string::write_string_iso(w, &self.name)?;
		self.center.write(w)
	}
}*/