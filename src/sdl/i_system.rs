// For SDL interface
use crate::sdl::i_main::IMain;
use crate::sdl::i_video::IVideo;
use crate::sdl::mixer_sound::MixerSound;


#[derive(Default)]
pub struct ISystem {
	pub return_wad_path: String,

	pub keyboard_started: bool,

	// .i_error()
	pub shutdowning: bool,
	pub errorcount: i32,
}
impl ISystem {
	pub fn new() -> Self {
		ISystem {
			shutdowning: false,
			errorcount: 0,
			..Default::default()
		}
	}

	pub fn i_error(&mut self, msgs: &[&str], 
		i_video: &IVideo,
		mixer_sound: &MixerSound,
		i_main: &mut IMain,
		) {

		/*
		va_list argptr;
		char buffer[8192];
		*/

		if self.shutdowning {
					match self.errorcount {
			1 => {
				i_video.sdl_force_ungrab_mouse();
			},
			2 => {
				mixer_sound.i_shutdown_music();
			},
			3 => {
				mixer_sound.i_shutdown_sound();
			},
			4 => {
				i_video.i_shutdown_graphics();
			},
			5 => {

			},
			6 => {

			},
			7 => {

			},
			8 => {

			},
			_ => {}
			}
			if self.errorcount > 20 {

			}			
		}
		self.shutdowning = true;

		// Display error message in the console before we
		// start shutting it down
		/*
		va_start(argptr, error);
		vsprintf(buffer, error, argptr);
		va_end(argptr);
		I_OutputMsg("\nI_Error(): %s\n", buffer);
		*/
		self.i_output_msg(msgs, i_main);

		// save game config, cvars..

	}

	pub fn i_output_msg(&self, _msgs: &[&str], i_main: &mut IMain) {
		match &i_main.logstream {
			Some(_file) => {
				cons_printf!("writing log to logstream...\n");
			},
			None => {
				cons_printf!("i_output_msg(): no logstream file found!\n");
			}
		}
	}
}