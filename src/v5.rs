use ::{ModelHeader, MAGIC, v2};
use types::Pos3;

pub const EXPECTED_MODEL_HEADER: ModelHeader = ModelHeader { magic: MAGIC, major: 5, minor: 0 };

struct Quantities {

}

struct V5 {
	quantities: Quantities,
	name: String,
	center: Pos3,
	common_vertices: (),
	triangles: (),
	materials: Vec<v2::Material>,
	tag_points: Vec<String>,
	frames: Vec<Frame>,
	points: (), // TODO
	// Len-prefixed
	shadow: Vec<ShadowEdge>
}

struct Frame {
	// TODO
}

struct ShadowEdge {
	unknown0: u32,
	unknown1: [u16; 4]
}