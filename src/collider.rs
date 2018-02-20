use std::io::{self, Read, Write};

use std::f32;
use types::Pos3;

const INFINITE_AABB: Aabb = Aabb {
	lower: Pos3(f32::INFINITY, f32::INFINITY, f32::INFINITY),
	upper: Pos3(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY)
};

pub struct CenterBuilder(Aabb);
impl CenterBuilder {
	pub fn begin() -> Self {
		CenterBuilder(INFINITE_AABB)
	}

	pub fn update(&mut self, point: Pos3) {
		self.0 = self.0.with(point);
	}

	pub fn build(&self) -> Pos3 {
		Pos3 (
			(self.0.upper.0 + self.0.lower.0) / 2.0,
			(self.0.upper.1 + self.0.lower.1) / 2.0,
			(self.0.upper.2 + self.0.lower.2) / 2.0
		)
	}
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Collider {
	pub aabb: Aabb,
	pub radius: f32
}

#[derive(Debug)]
pub struct ColliderBuilder {
	center: Pos3,
	aabb: Aabb,
	radius_squared: f32
}

impl ColliderBuilder {
	pub fn begin(center: Pos3) -> Self {
		ColliderBuilder {
			center,
			aabb: INFINITE_AABB,
			radius_squared: 0.0
		}
	}

	pub fn update(&mut self, point: Pos3) {
		let relative = Pos3 (
			point.0 - self.center.0,
			point.1 - self.center.1,
			point.2 - self.center.2
		);

		let parts = (
			relative.0 * relative.0,
			relative.1 * relative.1,
			relative.2 * relative.2
		);

		self.radius_squared = self.radius_squared.max (parts.0 + parts.1 + parts.2);

		self.aabb = self.aabb.with(point);
	}

	pub fn build(&self) -> Collider {
		Collider {
			aabb: if self.aabb == INFINITE_AABB { Aabb::default() } else { self.aabb },
			radius: self.radius_squared.sqrt()
		}
	}
}

/// An axis-aligned bounding box containing a lower corner and upper corner.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Aabb {
	pub lower: Pos3,
	pub upper: Pos3
}

impl Aabb {
	pub fn with(self, point: Pos3) -> Aabb {
		Aabb {
			lower: Pos3 (
				self.lower.0.min(point.0),
				self.lower.1.min(point.1),
				self.lower.2.min(point.2)
			),
			upper: Pos3 (
				self.upper.0.max(point.0),
				self.upper.1.max(point.1),
				self.upper.2.max(point.2)
			)
		}
	}

	pub fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Aabb {
			lower: Pos3::read(r)?,
			upper: Pos3::read(r)?
		})
	}

	pub fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		self.lower.write(w)?;
		self.upper.write(w)
	}
}