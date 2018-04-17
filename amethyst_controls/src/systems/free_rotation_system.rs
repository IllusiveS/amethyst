use amethyst_core::cgmath::{Deg, Vector3};
use amethyst_core::specs::{Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst_core::transform::Transform;
use amethyst_input::InputHandler;
use amethyst_renderer::ScreenDimensions;
use std::hash::Hash;
use std::marker::PhantomData;

use components::fly_control_tag::FlyControlTag;

/// The system that manages the view rotation.
/// Controlled by the mouse.
pub struct FreeRotationSystem<A, B> {
	sensitivity_x: f32,
	sensitivity_y: f32,
	_marker1: PhantomData<A>,
	_marker2: PhantomData<B>,
}

impl<A, B> FreeRotationSystem<A, B> {
	pub fn new(sensitivity_x: f32, sensitivity_y: f32) -> Self {
		FreeRotationSystem {
			sensitivity_x,
			sensitivity_y,
			_marker1: PhantomData,
			_marker2: PhantomData,
		}
	}
}

impl<'a, A, B> System<'a> for FreeRotationSystem<A, B>
	where
		A: Send + Sync + Hash + Eq + Clone + 'static,
		B: Send + Sync + Hash + Eq + Clone + 'static,
{
	type SystemData = (
		Fetch<'a, InputHandler<A, B>>,
		Fetch<'a, ScreenDimensions>,
		WriteStorage<'a, Transform>,
		ReadStorage<'a, FlyControlTag>,
	);

	fn run(&mut self, (input, dim, mut transform, tag): Self::SystemData) {
		// take the same mid-point as the MouseCenterLockSystem
		let half_x = dim.width() as i32 / 2;
		let half_y = dim.height() as i32 / 2;

		if let Some((posx, posy)) = input.mouse_position() {
			let offset_x = half_x as f32 - posx as f32;
			let offset_y = half_y as f32 - posy as f32;
			for (transform, _) in (&mut transform, &tag).join() {
				transform.rotate_local(
					Vector3::new(1.0, 0.0, 0.0),
					Deg(offset_y * self.sensitivity_y),
				);
				transform.rotate_global(
					Vector3::new(0.0, 1.0, 0.0),
					Deg(offset_x * self.sensitivity_x),
				);
			}
		}
	}
}