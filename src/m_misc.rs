pub struct Misc {

	//////todo: add comments

	// TODO: ACCEPTS a CV_PossibleValue_t
	//pub screenshot_cons_t: CV_PossibleValue_t,
	// TODO: ACCEPTS a consvar_t
	//pub cv_screenshot_option: consvar_t,
	//pub cv_screenshot_folder: consvar_t,

	//pub cv_screenshot_colorprofile: consvar_t,

	//pub moviemode_cons_t: CV_PossibleValue_t,

	//pub cv_moviemode: consvar_t,

	//pub cv_movie_option: consvar_t,
	//pub cv_movie_folder: consvar_t,

	//pub zlib_mem_level_t: CV_PossibleValue_t,

	//pub zlib_level_t: CV_PossibleValue_t,

	//pub zlib_strategy_t: CV_PossibleValue_t,

	//pub zlib_window_bits_t: CV_PossibleValue_t,

	//pub apng_delay: CV_PossibleValue_t,

	// zlib memory usage is as follows:
	// (1 << (zlib_window_bits+2)) +  (1 << (zlib_level+9))
	//pub cv_zlib_memory : consvar_t,
	//pub cv_zlib_level : consvar_t,
	//pub cv_zlib_strategy : consvar_t,
	//pub cv_zlib_window_bits : consvar_t,

	//pub cv_zlib_memorya : consvar_t,
	//pub cv_zlib_levela : consvar_t,
	//pub cv_zlib_strategya : consvar_t,
	//pub cv_zlib_window_bitsa : consvar_t,
	//pub cv_apng_delay : consvar_t,

	pub takescreenshot: bool,

	// TODO: ACCEPTS a moviemode_t
	//pub moviemode: moviemode_t,

	// ========== Configuration file ======= //
	pub configfile: Vec<char>, //char configfile[MAX_WADPATH];

	// ========== Configuration ======== //
	pub gameconfig_loaded: bool,

	// ========== Screenshots ======= //
	pub screenshot_palette: Vec<u8>, //UINT8 screenshot_palette[768];

	//TODO: ACCEPTS a png_structp
	//pub apng_ptr: png_structp,
	//TODO: ACCEPTS a png_infop
	//pub apng_info_ptr: png_infop,
	//TODO: ACCEPTS a apng_infop,
	//pub apng_ainfo_ptr: apng_infop,
	//TODO: ACCEPTS a png_FILE_p
	//pub apng_FILE: png_FILE_p,
	// TODO: ACCEPTS A png_uint_32
	//pub apng_frames: png_uint_32,

	/*
	static P_png_set_acTL aPNG_set_acTL = NULL;
	static P_png_write_frame_head aPNG_write_frame_head = NULL;
	static P_png_write_frame_tail aPNG_write_frame_tail = NULL;
	*/


}
impl Misc {
	pub fn new() -> Self {
		Misc {
			takescreenshot: false,
			configfile: Vec::new(),
			gameconfig_loaded: false,
			screenshot_palette: Vec::new(),
		}
	}

	pub fn m_startup_locale(&self) {
		let mut _textdomhandle: &str = "";

		cons_printf!("TODO: m_startup_locale()...\n");
		
		// is there a thing such as setlocale for rust?
		/*
		setlocale(LC_ALL, "");

		// Do not set numeric locale as that affects atof
		setlocale(LC_NUMERIC, "C");
		
		#ifdef GETTEXT
		// FIXME: global name define anywhere?
		#ifdef GETTEXTDOMAIN1
			textdomhandle = bindtextdomain("srb2", GETTEXTDOMAIN1);
		#endif
		#ifdef GETTEXTDOMAIN2
			if (!textdomhandle)
				textdomhandle = bindtextdomain("srb2", GETTEXTDOMAIN2);
		#endif
		#ifdef GETTEXTDOMAIN3
			if (!textdomhandle)
				textdomhandle = bindtextdomain("srb2", GETTEXTDOMAIN3);
		#endif
		#ifdef GETTEXTDOMAIN4
			if (!textdomhandle)
				textdomhandle = bindtextdomain("srb2", GETTEXTDOMAIN4);
		#endif
			if (textdomhandle)
				textdomain("srb2");
			else
				CONS_Printf("Could not find locale text domain!\n");
		#endif //GETTEXT
		*/
		if _textdomhandle != "" {
			println!("?");
		} else {
			cons_printf!("Could not find locale text domain!\n");
		}
	}

	pub fn m_save_config(&self, filename: &str) {
		let _f: std::fs::File;
		let _filepath: &str;

		// make sure not to write back the config until it's been correctly loaded
		if !self.gameconfig_loaded {
			return;
		}

		// can change the filename
		if filename != "" {
			if !filename.contains(".cfg") {
				//TODO: cons_alert
				//CONS_Alert(CONS_NOTICE, M_GetText("Config filename must be .cfg\n"));
				cons_printf!("Config filename must be .cfg\n");
			}
		}

	}
}