use crate::cmd_args;

use crate::d_main::DMain;
use crate::m_argv::MArgv;
use crate::console::Cons;
use crate::m_misc::Misc;
use crate::g_game::Game;
use crate::sounds::Sounds;
use crate::dehacked::Dehacked;
use crate::z_zone::Zone;
use crate::command::Command;
use crate::screen::Screen;

// from SDL interface
use crate::sdl::i_system::ISystem;
use crate::sdl::i_video::IVideo;
use crate::sdl::mixer_sound::MixerSound;
use crate::sdl::sdlmain::SdlMain;

pub struct IMain {
	// LogMessages
	pub logstream: Option<std::fs::File>,
	pub logfilename: std::string::String,
}
impl IMain {
	pub fn new() -> Self {
		IMain {
			logstream: None,
			logfilename: std::string::String::from("None"),
		}
	}

	//fn make_code_writeable()

	//ifdef LOGMESSAGES
	pub fn init_logging(&self) {
		/*
		let mut logdir: &str;

		//let mut my_time: time_t;
		//let mut timeinfo: tm;
		let mut format: &str;
		let mut reldir: &str;

		let mut left: i32;
		let mut fileabs: bool;

		//if defined unix/mac
		let mut link: &str;
		//endif

		//TODO: ACCEPTS a DMain struct
		//logdir = d_main::d_home;

		//TODO: ACCEPTS a Time struct
		//my_time = Time::new();
		
		// timeinfo = localtime(&my_time);

		// TODO: ACCEPTS a MArgv struct
		if MArgv::m_check_parm(cmd_args::LOGFILE) && MArgv::m_is_next_parm() {
			format = MArgv::m_get_next_parm();
			fileabs = MArgv::m_is_path_abs(format);
		} else {
			format = "log-%Y-%m-%d_%H-%M-%S.txt";
			fileabs = false;
		}

		if fileabs {
			strftime(logfilename, sizeof logfilename, format, timeinfo);
		} else {
			if MArgv::m_check_parm(cmd_args::LOGDIR) && MArgv::m_is_next_parm() {
				reldir = MArgv::m_get_next_parm();
			} else {
				reldir = "logs";
			}

			if MArgv::m_is_path_abs(reldir) {
				left = snprintf(logfilename, sizeof logfilename,
					"%s"PATHSEP, reldir);
			} else {
				// ifdef DEFAULTDIR
				// `if logdir {}` ?
				if logdir != "" {
					left = snprintf(logfilename, sizeof logfilename,
						"%s"PATHSEP DEFAULTDIR PATHSEP"%s"PATHSEP, logdir, reldir);
				} else
				//endif
				{
					left = snprintf(logfilename, sizeof logfilename,
						"."PATHSEP"%s"PATHSEP, reldir);
				}
				strftime(&logfilename[left], sizeof logfilename - left,
					format, timeinfo);
			}
			//TODO: ACCEPTS a M??<- which struct to use for this? is also used here
			M??::m_mkdir_each_until(logfilename,
				M??::m_path_parts(logdir) - 1,
				M??::m_path_parts(logfilename) - 1, 0755);
			// if defined unix/mac
			logstream = fopen(logfilename, "w");

			//ifdef DEFAULTDIR
			//`if logdir {}` ?
			if logdir != "" {
				link = va("%s/"DEFAULTDIR"/latest-log.txt", logdir);
			} else 
			//endif 
			{
				link = "latest-log.txt";
			}
			// unlink: WHATIS?
			unlink(link);
			// symlink: WHATIS?
			if symlink(logfilename, link) == -1 {
				//TODO: ACCEPTS a ISystem struct
				ISystem::i_output_msg("Error symlinking latest-log.txt: %s\n", strerror(errno));
			}
			//else
			logstream = fopen("latest-log.txt", "wt+");
			//endif
			*/
			cons_printf!("Logging is on.\n");
		}
		//endif

	pub fn i_main(&mut self, 
		mut m_argv: MArgv, 
		console: Cons,
		mut d_main: DMain,
		m_misc: Misc,
		mut g_game: Game,
		sounds: Sounds,
		dehacked: Dehacked,
		mut i_system: ISystem,
		mut i_video: IVideo<'static>,
		mixer_sound: MixerSound,
		z_zone: Zone,
		mut command: Command,
		mut sdlmain: SdlMain,
		screen: Screen
		) {
		//ifdef LOGMESSAGES
		if m_argv.m_check_parm(cmd_args::NOLOG) == 0 {
			self.init_logging();
		}
		
		//endif

		// SDL things
		let mut sdl_context: Option<sdl2::Sdl> = None;
    	let mut video_subsystem: Option<sdl2::VideoSubsystem> = None;
    	let mut window_canvas: Option<sdl2::render::WindowCanvas> = None;
    	let mut event_pump: Option<sdl2::EventPump> = None;

		cons_printf!("Setting up SRB2...\n");
		d_main.srb2_main( 
			&console, &m_misc, &mut m_argv, &mut g_game,
			&sounds, &dehacked, &mut i_system, &mut i_video,
			// SDL
			&mut sdl_context, &mut video_subsystem, &mut window_canvas,
			&mut event_pump,

			&mixer_sound, self, &z_zone, &mut command,
			&mut sdlmain, &screen
		);
		
		/*
		//ifdef LOGMESSAGES
		if m_argv.m_check_parm(cmd_args::NOLOG) == 0 {
			cons_printf!("Logfile: ", self.logfilename, "\n");
		}
		
		cons_printf!("Entering main game loop...\n");
		// Entrypoint to the game's main loop
		d_main.srb2_main_loop();

		// Quit successful
		//std::process::exit(0);
		*/
	}
}
