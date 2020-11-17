#[derive(Default)]
pub struct IVideo {
	pub num_vid_modes: i32,
	pub vid_mode_name: Vec<String>,
	// TODO: ACCEPTS a rendermode_t
	//pub render_mode: RenderModeT,
	//pub chosen_render_mode: RenderModeT,
	pub high_color: bool,

}
impl IVideo {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn sdl_force_ungrab_mouse(&self) {
		cons_printf!("TODO: make sdl_force_ungrab_mouse() !\n");
	}

	pub fn i_shutdown_joystick(&self) {
		cons_printf!("TODO: make i_shutdown_joystick() work! \n");
	}

	pub fn i_shutdown_graphics(&self) {
		cons_printf!("TODO: make i_shutdown_graphics() work! \n");
	}
}