use amethyst_core::cgmath::{Vector3, Angle, Deg, Point3};
use amethyst_core::specs::{Fetch, Join, ReadStorage, System, WriteStorage};
use amethyst_core::timing::Time;
use amethyst_core::transform::Transform;
use amethyst_input::InputHandler;
use std::hash::Hash;
use std::marker::PhantomData;



use components::arc_camera_component::ArcCameraComponent;

/// The system that manages the fly movement.
/// Generic parameters are the parameters for the InputHandler.
pub struct ArcCameraMovementSystem<A, B> {
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

impl<A, B> ArcCameraMovementSystem<A, B>
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
		ArcCameraMovementSystem {
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

impl<'a, A, B> System<'a> for ArcCameraMovementSystem<A, B>
	where
		A: Send + Sync + Hash + Eq + Clone + 'static,
		B: Send + Sync + Hash + Eq + Clone + 'static,
{
	type SystemData = (
		Fetch<'a, Time>,
		WriteStorage<'a, Transform>,
		Fetch<'a, InputHandler<A, B>>,
		WriteStorage<'a, ArcCameraComponent>,
	);

	fn run(&mut self, (time, mut transform, input, mut arc_camera_components): Self::SystemData) {
		let x = ArcCameraMovementSystem::get_axis(&self.right_input_axis, &input);
		let y = ArcCameraMovementSystem::get_axis(&self.up_input_axis, &input);
		let z = ArcCameraMovementSystem::get_axis(&self.forward_input_axis, &input);

		for (transform, arc_camera_component) in (&mut transform, &mut arc_camera_components).join() {
			arc_camera_component.radius += z * time.delta_seconds() * self.speed;
			arc_camera_component.azimuth += Deg(x * time.delta_seconds() * self.speed);
			arc_camera_component.elevation += Deg(y * time.delta_seconds() * self.speed);

//			arc_camera_component.azimuth.normalize_self();
//			arc_camera_component.elevation.normalize_self();

			let target_vector = Vector3::new(
				arc_camera_component.radius * Angle::sin(arc_camera_component.elevation) * Angle::cos(arc_camera_component.azimuth),
				arc_camera_component.radius * Angle::sin(arc_camera_component.elevation) * Angle::sin(arc_camera_component.azimuth),
				arc_camera_component.radius * Angle::cos(arc_camera_component.elevation)
			);

			let direction_vector = target_vector - transform.translation;

			transform.move_local(direction_vector, 1.0);
			transform.look_at(
				Point3::from_vec(arc_camera_component.look_target),
				Point3::from_vec(transform.translation),
				arc_camera_component.up_vector
			);
		}
	}
}