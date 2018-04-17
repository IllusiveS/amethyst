use amethyst_core::cgmath::Vector3;
use amethyst_core::specs::{Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst_core::timing::Time;
use amethyst_core::transform::Transform;
use amethyst_input::InputHandler;
use std::hash::Hash;
use std::marker::PhantomData;

use components::fly_control_tag::FlyControlTag;

/// The system that manages the fly movement.
/// Generic parameters are the parameters for the InputHandler.
pub struct FlyMovementSystem<A, B> {
	/// The movement speed of the movement in units per second.
	speed: f32,
	/// The name of the input axis to locally move in the x coordinates.
	right_input_axis: Option<A>,
	/// The name of the input axis to locally move in the y coordinates.
	up_input_axis: Option<A>,
	/// The name of the input axis to locally move in the z coordinates.
	forward_input_axis: Option<A>,
	_marker: PhantomData<B>,
}

impl<A, B> FlyMovementSystem<A, B>
	where
		A: Send + Sync + Hash + Eq + Clone + 'static,
		B: Send + Sync + Hash + Eq + Clone + 'static,
{
	pub fn new(
		speed: f32,
		right_input_axis: Option<A>,
		up_input_axis: Option<A>,
		forward_input_axis: Option<A>,
	) -> Self {
		FlyMovementSystem {
			speed,
			right_input_axis,
			up_input_axis,
			forward_input_axis,
			_marker: PhantomData,
		}
	}

	fn get_axis(name: &Option<A>, input: &InputHandler<A, B>) -> f32 {
		name.as_ref()
			.and_then(|ref n| input.axis_value(n))
			.unwrap_or(0.0) as f32
	}
}

impl<'a, A, B> System<'a> for FlyMovementSystem<A, B>
	where
		A: Send + Sync + Hash + Eq + Clone + 'static,
		B: Send + Sync + Hash + Eq + Clone + 'static,
{
	type SystemData = (
		Fetch<'a, Time>,
		WriteStorage<'a, Transform>,
		Fetch<'a, InputHandler<A, B>>,
		ReadStorage<'a, FlyControlTag>,
	);

	fn run(&mut self, (time, mut transform, input, tag): Self::SystemData) {
		let x = FlyMovementSystem::get_axis(&self.right_input_axis, &input);
		let y = FlyMovementSystem::get_axis(&self.up_input_axis, &input);
		let z = FlyMovementSystem::get_axis(&self.forward_input_axis, &input);

		let dir = Vector3::new(x, y, z);

		for (transform, _) in (&mut transform, &tag).join() {
			transform.move_local(dir, time.delta_seconds() * self.speed);
		}
	}
}