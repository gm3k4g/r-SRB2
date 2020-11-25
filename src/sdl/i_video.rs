use crate::cmd_args;

use crate::m_argv::MArgv;
use crate::d_main::DMain;
use crate::command::Command;
use crate::command::ConsVarT;
use crate::screen::Screen;

// For SDL interface
use crate::sdl::i_main::IMain;
use crate::sdl::i_system::ISystem;
use crate::sdl::mixer_sound::MixerSound;
use crate::sdl::sdlmain::SdlMain;


//
use sdl2_sys::SDL_bool;

#[derive(PartialEq)]
pub enum RenderModeT {
	RenderSoft = 1,

	RenderOpengl = 2,

	// Dedicated server
	RenderNone = 3
}
impl Default for RenderModeT {
	fn default() -> Self {
		RenderModeT::RenderNone
	}
}

const MAXWINMODES: usize = 18;

// kill me now...... sublime to the rescue!
pub struct IVideo<'a> {
	pub num_vid_modes: i32,
	pub vid_mode_name: Vec<String>,

	pub render_mode: RenderModeT,
	pub chosen_render_mode: RenderModeT,
	pub high_color: bool,

	pub cv_vidwait: ConsVarT,
	pub cv_stretch: ConsVarT,
	pub cv_alwaysgrabmouse: ConsVarT,

	pub graphics_started: u8, //is used in console.c and screen.c
	pub vid_opengl_state: i32,

	// To disable fullscreen at startup; is set in VID_PrepareModeList
	pub allow_fullscreen: bool,

	//pub sdl_bool: SDL_FALSE,
	pub disable_fullscreen: sdl2_sys::SDL_bool,
	pub disable_mouse: sdl2_sys::SDL_bool,//sdl2_sys::SDL_bool, //SDL_FALSE,
	

	pub use_fullscreen: sdl2_sys::SDL_bool, //(disable_fullscreen||!allow_fullscreen)?0:cv_fullscreen.value

	pub ico_surface: Option<sdl2::surface::Surface<'a>>,

	pub realwidth: u16,
	pub realheight: u16,

	pub mousegrabok: sdl2_sys::SDL_bool,
	pub wrapmouseok: sdl2_sys::SDL_bool,

	pub videoblitok: sdl2_sys::SDL_bool,
	pub exposevideo: sdl2_sys::SDL_bool,
	pub usesdl2soft: sdl2_sys::SDL_bool,
	pub borderlesswindow: sdl2_sys::SDL_bool,

	pub windowed_modes: [(u32, u32); MAXWINMODES]
}
impl IVideo<'_> {
	pub fn new() -> Self {
		Self {
			num_vid_modes: 0,
			vid_mode_name: Vec::new(),
			render_mode: RenderModeT::RenderNone,
			chosen_render_mode: RenderModeT::RenderNone,
			high_color: false,
			cv_vidwait: ConsVarT::new(),
			cv_stretch: ConsVarT::new(),
			cv_alwaysgrabmouse: ConsVarT::new(),
			graphics_started: 0,
			vid_opengl_state: 0,
			allow_fullscreen: false,
			disable_fullscreen: sdl2_sys::SDL_bool::SDL_FALSE,
			disable_mouse: sdl2_sys::SDL_bool::SDL_FALSE,
			use_fullscreen: sdl2_sys::SDL_bool::SDL_FALSE,
			ico_surface: None,

			realwidth: crate::screen::BASEVIDWIDTH as u16,
			realheight: crate::screen::BASEVIDHEIGHT as u16,

			mousegrabok: sdl2_sys::SDL_bool::SDL_FALSE,
			wrapmouseok: sdl2_sys::SDL_bool::SDL_FALSE,
			videoblitok: sdl2_sys::SDL_bool::SDL_FALSE,
			exposevideo: sdl2_sys::SDL_bool::SDL_FALSE,
			usesdl2soft: sdl2_sys::SDL_bool::SDL_FALSE,
			borderlesswindow: sdl2_sys::SDL_bool::SDL_FALSE,
			windowed_modes: [
					(1920,1200), // 1.60,6.00
					(1920,1080), // 1.66
					(1680,1050), // 1.60,5.25
					(1600,1200), // 1.33
					(1600, 900), // 1.66
					(1366, 768), // 1.66
					(1440, 900), // 1.60,4.50
					(1280,1024), // 1.33?
					(1280, 960), // 1.33,4.00
					(1280, 800), // 1.60,4.00
					(1280, 720), // 1.66
					(1152, 864), // 1.33,3.60
					(1024, 768), // 1.33,3.20
					( 800, 600), // 1.33,2.50
					( 640, 480), // 1.33,2.00
					( 640, 400), // 1.60,2.00
					( 320, 240), // 1.33,1.00
					( 320, 200), // 1.60,1.00
							]
		}
	}

	pub fn i_startup_graphics(&mut self,
		d_main: &mut DMain,
		command: &mut Command,
		i_system: &mut ISystem,
		mixer_sound: &MixerSound,
		i_main: &mut IMain,
		m_argv: &mut MArgv,

		//SDL
		sdl_context: &mut Option<sdl2::Sdl>,
		video_subsystem: &mut Option<sdl2::VideoSubsystem>,
		window_canvas: &mut Option<sdl2::render::WindowCanvas>,
		event_pump: &mut Option<sdl2::EventPump>,

		sdlmain: &mut SdlMain,
		screen: &Screen
		) {
		if d_main.dedicated {
			self.render_mode = RenderModeT::RenderNone;
		}

		// if graphics already started, return
		if self.graphics_started != 0 {
			return;
		}

		/*
		COM_AddCommand ("vid_nummodes", VID_Command_NumModes_f);
		COM_AddCommand ("vid_info", VID_Command_Info_f);
		COM_AddCommand ("vid_modelist", VID_Command_ModeList_f);
		COM_AddCommand ("vid_mode", VID_Command_Mode_f);
		*/
		command.com_addcommand("vid_nummodes", i_system, &self, mixer_sound, i_main);
		command.com_addcommand("vid_info", i_system, &self, mixer_sound, i_main);
		command.com_addcommand("vid_modelist", i_system, &self, mixer_sound, i_main);
		command.com_addcommand("vid_mode", i_system, &self, mixer_sound, i_main);

		command.cv_register_var(&self.cv_vidwait);
		command.cv_register_var(&self.cv_stretch);
		command.cv_register_var(&self.cv_alwaysgrabmouse);
		// matching: insert argument about some weird C shit
		self.disable_mouse = 
			match m_argv.m_check_parm(cmd_args::NOMOUSE) {
				0 => SDL_bool::SDL_FALSE,
				_ => SDL_bool::SDL_TRUE,
			};
		self.disable_fullscreen = 
			match m_argv.m_check_parm(cmd_args::WIN) {
				0 => SDL_bool::SDL_FALSE,
				_ => SDL_bool::SDL_TRUE,
			};

		i_system.keyboard_started = true;


		// TODO: this is incorrect... right?
		// "initializing" sdl here...
		*sdl_context = Some(sdl2::init()
			.unwrap());
		*video_subsystem = Some(sdl_context.as_ref().unwrap().video()
			.unwrap());
		// Can get window context from WindowCanvas so having window doesn't matter
		let window = Some(video_subsystem.as_ref().unwrap().window("SRB2", self.windowed_modes[17].0, self.windowed_modes[17].1)
        	.position_centered()
        	.build()
        	.unwrap());
		*window_canvas = Some(window.unwrap().into_canvas().build()
			.unwrap());
		*event_pump = Some(sdl_context.as_ref().unwrap().event_pump()
			.unwrap());

		// .. in order to get the current video driver
		let vd: &str = video_subsystem.as_ref().unwrap().current_video_driver();

		if 	vd.to_lowercase() != "gcvideo" ||
			vd.to_lowercase() != "fbcon" ||
			vd.to_lowercase() != "wii" ||
			vd.to_lowercase() != "psl1ght" 
			{
			sdlmain.framebuffer = sdl2_sys::SDL_bool::SDL_TRUE;
		}

		//ifdef HWRENDER
		match m_argv.m_check_parm(cmd_args::OPENGL) {
			0 => {},
			_ => {
				self.chosen_render_mode = RenderModeT::RenderOpengl;
				self.render_mode = RenderModeT::RenderOpengl;
			}
		};
		match m_argv.m_check_parm(cmd_args::SOFTWARE) {
			0 => {},
			_ => {
				self.chosen_render_mode = RenderModeT::RenderSoft;
				self.render_mode = RenderModeT::RenderSoft;
			}
		};
		//endif

		self.usesdl2soft = match m_argv.m_check_parm(cmd_args::OPENGL) {
			0 => SDL_bool::SDL_FALSE,
			_ => SDL_bool::SDL_TRUE,
		};
		self.borderlesswindow = match m_argv.m_check_parm(cmd_args::BORDERLESS) {
			0 => SDL_bool::SDL_FALSE,
			_ => SDL_bool::SDL_TRUE,
		};

		//SDL_EnableKeyRepeat(SDL_DEFAULT_REPEAT_DELAY>>1,SDL_DEFAULT_REPEAT_INTERVAL<<2);
		self.vid_command_mode_list_f();

		//ifdef HWRENDER
		match m_argv.m_check_parm(cmd_args::NOGL) {
			0 => {},
			_ => {
				self.vid_opengl_state = -1; //don't startup Ogl
			}
		};
		if self.chosen_render_mode == RenderModeT::RenderOpengl {
			self.vid_startup_opengl();
		}
		//endif

		//Window icon
		//ifdef HAVE_IMAGE
		//self.ico_surface = IMG_ReadXPMFromArray(sdl_icon_xpm);
		//pub fn set_icon<S: AsRef<SurfaceRef>>(&mut self, icon: S)
		//window.set_icon()
		//endif

		// TODO: do this: vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv ?

		// Fury: we do window initialization after GL setup to allow
		// SDL_GL_LoadLibrary to work well on Windows

		// Create window
		//Impl_CreateWindow(USE_FULLSCREEN);
		//Impl_SetWindowName("SRB2 "VERSIONSTRING);
		self.vid_set_mode(self.vid_get_mode_for_size(crate::screen::BASEVIDWIDTH, crate::screen::BASEVIDHEIGHT)); //VID_SetMode(VID_GetModeForSize(BASEVIDWIDTH, BASEVIDHEIGHT));

		match m_argv.m_check_parm(cmd_args::NOMOUSEGRAB) {
		// if 0 //defined (_DEBUG)
			0 => {
				/*char videodriver[4] = {'S','D','L',0};
				if (!M_CheckParm("-mousegrab") &&
			    *strncpy(videodriver, SDL_GetCurrentVideoDriver(), 4) != '\0' &&
			    strncasecmp("x11",videodriver,4) == 0)
				mousegrabok = SDL_FALSE; //X11's XGrabPointer not good
				*/
				if vd.to_lowercase() != "x11" {
					self.mousegrabok = SDL_bool::SDL_FALSE;
				}
			}
		//endif
			_ => {
				self.mousegrabok = SDL_bool::SDL_FALSE;
			}
		}

		self.realwidth = crate::screen::BASEVIDWIDTH as u16;  //vid.width as u16;
		self.realheight = crate::screen::BASEVIDHEIGHT as u16; //vid.height as u16;

		self.vid_command_info_f(m_argv);


	}

	pub fn vid_command_info_f(&mut self, m_argv: &mut MArgv) {
		//if 0 
		// SDL2STUB()
		//else
		//if 0
		/*
		const SDL_VideoInfo *videoInfo;
		videoInfo = SDL_GetVideoInfo(); //Alam: Double-Check
		if (videoInfo)
		{
		CONS_Printf("%s", M_GetText("Video Interface Capabilities:\n"));
		if (videoInfo->hw_available)
			CONS_Printf("%s", M_GetText(" Hardware surfaces\n"));
		if (videoInfo->wm_available)
			CONS_Printf("%s", M_GetText(" Window manager\n"));
		//UnusedBits1  :6
		//UnusedBits2  :1
		if (videoInfo->blit_hw)
			CONS_Printf("%s", M_GetText(" Accelerated blits HW-2-HW\n"));
		if (videoInfo->blit_hw_CC)
			CONS_Printf("%s", M_GetText(" Accelerated blits HW-2-HW with Colorkey\n"));
		if (videoInfo->wm_available)
			CONS_Printf("%s", M_GetText(" Accelerated blits HW-2-HW with Alpha\n"));
		if (videoInfo->blit_sw)
		{
			CONS_Printf("%s", M_GetText(" Accelerated blits SW-2-HW\n"));
			if (!M_CheckParm("-noblit")) videoblitok = SDL_TRUE;
		}
		if (videoInfo->blit_sw_CC)
			CONS_Printf("%s", M_GetText(" Accelerated blits SW-2-HW with Colorkey\n"));
		if (videoInfo->blit_sw_A)
			CONS_Printf("%s", M_GetText(" Accelerated blits SW-2-HW with Alpha\n"));
		if (videoInfo->blit_fill)
			CONS_Printf("%s", M_GetText(" Accelerated Color filling\n"));
		//UnusedBits3  :16
		if (videoInfo->video_mem)
			CONS_Printf(M_GetText(" There is %i KB of video memory\n"), videoInfo->video_mem);
		else
			CONS_Printf("%s", M_GetText(" There no video memory for SDL\n"));
		// *vfmt
		}
		*/
		//else
		match m_argv.m_check_parm(cmd_args::NOBLIT) {
			0 => self.videoblitok = SDL_bool::SDL_TRUE,
			_ => {},
		};
		//endif
		//SurfaceInfo(bufSurface, M_GetText("Current Engine Mode"));
		//SurfaceInfo(vidSurface, M_GetText("Current Video Mode"));

	}

	pub fn surface_info(info_surface: Option<sdl2::surface::Surface>, surface_text: Option<&str>, m_argv: &mut MArgv) {
		let mut vfbpp: i32;

		match info_surface {
			None => return,
			_ => {}
		};
		match surface_text {
			None => cons_printf!("Unknown Surface\n"),//m_get_text("Unknown Surface");,
			_ => {}
		};

		//infoSurface->format?infoSurface->format->BitsPerPixel:0;
		//vfbpp = info_surface.unwrap().
		cons_printf!(/*" \x82",*/ surface_text.unwrap(), "\n");
		//cons_printf!(m_argv.m_get_text(, "x", ," at ", " bit color\n"), info_surface.w, info_surface.h, vfbpp);

		/*
		if (infoSurface->flags&SDL_PREALLOC)
		CONS_Printf("%s", M_GetText(" Uses preallocated memory\n"));
		else
		CONS_Printf("%s", M_GetText(" Stored in system memory\n"));
		if (infoSurface->flags&SDL_RLEACCEL)
		CONS_Printf("%s", M_GetText(" Colorkey RLE acceleration blit\n"));
		*/
	}

	pub fn vid_get_mode_for_size(&self, w: i32, h: i32) -> i32 {
		0
	}

	pub fn vid_set_mode(&self, mode_num: i32) -> i32 {
		0
	}

	pub fn vid_startup_opengl(&self) {
		cons_printf!("TODO: make vid_startup_opengl() work!\n");
	}

	pub fn vid_command_mode_list_f(&self) {
		// List windowed modes
		let mut i: usize = 0;
		cons_printf!("NOTE: Under SDL2, all modes are supported on all platforms.\n");
		cons_printf!("Under opengl, fullscreen only supports native desktop resolution.\n");
		cons_printf!("Under software, the mode is stretched up to desktop resolution.\n");

		while i < MAXWINMODES {
			cons_printf!(" ", i, ":", self.windowed_modes[i].0, "x", self.windowed_modes[i].1, "\n");
			i = i + 1;
		}
	}

	pub fn vid_command_nummodes_f(&self) {
		cons_printf!(self.vid_nummodes(), " video mode(s) available(s)\n");
	}

	pub fn vid_nummodes(&self) -> i32 {
		cons_printf!("TODO: make vid_nummodes() work!\n");
		-2 // means idk
		/*
		if USE_FULLSCREEN && numVidModes != -1 {
			return numVidModes - firstEntry;
		} else {
			return MAXWINMODES;
		}*/
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