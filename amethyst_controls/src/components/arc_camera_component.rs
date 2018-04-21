use amethyst_core::specs::{Component, HashMapStorage};
use amethyst_core::cgmath::{Deg, Vector3};

/// Add this to a camera if you want it to be an arc camera.
/// You need to add the ArcBallCameraBundle or the required systems for it to work.

pub struct ArcCameraComponent{
	pub up_vector : Vector3<f32>,
	pub look_target : Vector3<f32>,
	pub radius : f32,
	pub azimuth : Deg<f32>,
	pub elevation : Deg<f32>
}

impl Default for ArcCameraComponent {
	fn default() -> ArcCameraComponent {
		ArcCameraComponent{
			up_vector : Vector3::unit_z(),
			look_target : Vector3::new(0.0, 0.0, 0.0),
			radius : 0.0,
			azimuth : Deg(0.0),
			elevation : Deg(0.0)
		}
	}
}

impl Component for ArcCameraComponent {
	type Storage = HashMapStorage<ArcCameraComponent>;
}
