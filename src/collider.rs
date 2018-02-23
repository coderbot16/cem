use std::io::{self, Read, Write};

use cgmath::{Point3, MetricSpace};
use std::f32;
use Encode;

const INFINITE_AABB: Aabb = Aabb {
	lower: Point3 { x:  f32::INFINITY, y:  f32::INFINITY, z:  f32::INFINITY},
	upper: Point3 { x: -f32::INFINITY, y: -f32::INFINITY, z: -f32::INFINITY}
};

pub struct CenterBuilder(Aabb);
impl CenterBuilder {
	pub fn begin() -> Self {
		CenterBuilder(INFINITE_AABB)
	}

	pub fn update(&mut self, point: Point3<f32>) {
		self.0 = self.0.with(point);
	}

	pub fn build(&self) -> Point3<f32> {
		Point3 {
			x: (self.0.upper.x + self.0.lower.x) / 2.0,
			y: (self.0.upper.y + self.0.lower.y) / 2.0,
			z: (self.0.upper.z + self.0.lower.z) / 2.0
		}
	}
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Collider {
	pub aabb: Aabb,
	pub radius: f32
}

#[derive(Debug)]
pub struct ColliderBuilder {
	center: Point3<f32>,
	aabb: Aabb,
	radius_squared: f32
}

impl ColliderBuilder {
	pub fn begin(center: Point3<f32>) -> Self {
		ColliderBuilder {
			center,
			aabb: INFINITE_AABB,
			radius_squared: 0.0
		}
	}

	pub fn update(&mut self, point: Point3<f32>) {
		self.radius_squared = self.radius_squared.max (
			point.distance2(self.center)
		);

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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb {
	pub lower: Point3<f32>,
	pub upper: Point3<f32>
}

impl Aabb {
	pub fn with(self, point: Point3<f32>) -> Aabb {
		Aabb {
			lower: Point3 {
				x: self.lower.x.min(point.x),
				y: self.lower.y.min(point.y),
				z: self.lower.z.min(point.z)
			},
			upper: Point3 {
				x: self.upper.x.max(point.x),
				y: self.upper.y.max(point.y),
				z: self.upper.z.max(point.z)
			}
		}
	}
}

impl Encode for Aabb {
	fn read<R>(r: &mut R) -> io::Result<Self> where R: Read {
		Ok(Aabb {
			lower: Point3::read(r)?,
			upper: Point3::read(r)?
		})
	}

	fn write<W>(&self, w: &mut W) -> io::Result<()> where W: Write {
		self.lower.write(w)?;
		self.upper.write(w)
	}
}

impl Default for Aabb {
	fn default() -> Self {
		Aabb {
			lower: Point3::new(0.0, 0.0, 0.0),
			upper: Point3::new(0.0, 0.0, 0.0)
		}
	}
}