use amethyst_core::specs::{Fetch, FetchMut, System};
use amethyst_renderer::{ScreenDimensions, WindowMessages};

/// The system that locks the mouse to the center of the screen. Useful for first person camera.
pub struct MouseCenterLockSystem;

impl<'a> System<'a> for MouseCenterLockSystem {
	type SystemData = (Fetch<'a, ScreenDimensions>, FetchMut<'a, WindowMessages>);

	fn run(&mut self, (dim, mut msg): Self::SystemData) {
		let half_x = dim.width() as i32 / 2;
		let half_y = dim.height() as i32 / 2;
		msg.send_command(move |win| {
			if let Err(err) = win.set_cursor_position(half_x, half_y) {
				error!("Unable to set the cursor position! Error: {:?}", err);
			}
		});
	}
}
