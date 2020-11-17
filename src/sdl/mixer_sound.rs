#[derive(Default)]
pub struct MixerSound {

}
impl MixerSound {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn i_shutdown_music(&self) {
		cons_printf!("TODO: make i_shutdown_music() work! \n");
	}

	pub fn i_shutdown_sound(&self) {
		cons_printf!("TODO: make i_shutdown_sound() work! \n");
	}

}