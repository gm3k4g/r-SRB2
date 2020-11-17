
/*
// <'a> used due to the &str used for `printf()`
pub struct ConsPrinter<'a> {
	// === .printf()
	//// Used for making a 'variadic' function
	pub msg: &'a str, 		//the string to display
	pub argi32: Option<i32>	//can depict i32
	// TODO: add more values here to be
	// implemented into `printf`?
}
impl<'a> ConsPrinter<'a> {
	pub fn new(mmsg: &'a str) -> ConsPrinter<'a> {
		ConsPrinter {
			msg: mmsg,
			argi32: None
		}
	}

	pub fn argi32(self, ai32: i32) -> Self {
		ConsPrinter {
			argi32: Some(ai32),
			..self
		}
	}

	pub fn printf(mmsg: &'a str, ai32: i32) {
		println!("")
	}
}
*/

pub struct Cons {
	// TODO: ACCEPTS an I_mutex
	//pub con_mutex: I_mutex,
	// top clip value for view render: do not draw part of view hidden by console
	pub con_clipviewtop: i32, //(useless)

	pub yellowmap: u8,
	pub magentamap: u8,
	pub lgreenmap: u8,
	pub bluemap: u8,
	pub graymap: u8,
	pub redmap: u8,
	pub orangemap: u8,
	pub skymap: u8,
	pub purplemap: u8,
	pub aquamap: u8,
	pub peridotmap: u8,
	pub azuremap: u8,
	pub brownmap: u8,
	pub rosymap: u8,
	pub invertmap: u8,

	// Console bg color (auto updated to match)
	pub consolebgmap: u8,
	pub promptbgmap: u8,

	/////////// statics
	// console has been initialised
	pub con_started: bool,
	// true at game startup, screen need refreshing
	pub con_startup: bool,
	// at startup toggle console translucency when first off
	pub con_forcepic: bool,
	// set true when screen size has changed
	pub con_recalc: bool,

	// console ticker for anim or blinking prompt cursor
	// TODO: accepts a tic_t
	//pub con_tick: tic_t

	// true when console key pushed, ticker will handle
	pub consoletoggle: bool,
	// console prompt is ready
	pub consoleready: bool,

	// vid lines used by console at final position
	// 0 means console if off, or moving out
	pub con_destlines: i32,
	// vid lines currently used by console
	pub con_curlines: i32,
	// number of console heads up message lines
	pub con_hudlines: i32,
 	// remaining time of display for hud msg lines
	pub con_hudtimes: Vec<i32>,
	// lines of top of screen to refresh
	// top screen lines to refresh when view reduced
	pub con_clearlines: i32,
	// hud messages have changed, need refresh
	// when messages scroll, we need a backgrnd refresh
	pub con_hudupdate: bool,

	// console text output
	pub con_line: char,		// console text output current line
	// TODO: ACCEPTS a size_t
	//pub con_cx: size_t,		// cursor position in current line
	//pub con_cy: size_t,		// cursor line number in con_buffer, is always
							// increasing, and wrapped around in the text
							// buffer using modulo.
	// lines of console text into the console buffer
	//pub con_totallines: size_t,	
	// columns of chars, depend on vid mode width
	//pub con_width: size_t,
	// how many rows of text to scroll up (pgup/pgdn)
	//pub con_scrollup: size_t,

	// console text scale factor, text size scale factor
	pub con_scalefactor: u32,

	// hold 32 last lines of input for history
	//#define CON_MAXPROMPTCHARS 256
	//#define CON_PROMPTCHAR '$'

	// hold last 32 prompt lines
	pub inputlines: Vec<char>, //char inputlines[32][CON_MAXPROMPTCHARS]
	// current input line number
	pub inputline: i32,
	// line number of history input line to restore
	pub inputhist: i32,
	// position of cursor in line
	//pub input_cur: size_t,
	// position of selection marker (I.E.: anything between this and input_cur is "selected")
	//pub input_sel: size_t,
	// length of current line, used to bound cursor and such
	//pub input_len: size_t,
// notice: input does NOT include the "$" at the start of the line. - 11/3/16
	
////////////========= Console vars and Commands//
	pub con_buffer: Vec<char>,

	// TODO: ACCEPTS a consvar_t
	//pub cons_msgtimeout: consvar_t,
	// number of lines displayed on the HUD
	//pub cons_hudlines: consvar_t,

	// TODO: ACCEPTS a CV_PossibleValue_t
	// number of lines console move per frame
	// (con_speed needs a limit, apparently)
	//pub speed_cons_t: CV_PossibleValue_t,
	//pub cons_speed: consvar_t,
	// percentage of screen height to use for console
	//pub cons_height: consvar_t,
	// whether to use console background picture, or translucent mode
	//pub backpic_cons_t: consvar_t,

	//pub backcolor_cons_t: CV_PossibleValue_t,

	//pub cons_backcolor: consvar_t
}
impl Cons {
	pub fn new() -> Self {
		Cons {
			con_clipviewtop: 0,
				yellowmap: 0,
				magentamap: 0,
				lgreenmap: 0,
				bluemap: 0,
				graymap: 0,
				redmap: 0,
				orangemap: 0,
				skymap: 0,
				purplemap: 0,
				aquamap: 0,
				peridotmap: 0,
				azuremap: 0,
				brownmap: 0,
				rosymap: 0,
				invertmap: 0,
				consolebgmap: 0,
				promptbgmap: 0,
				con_started: false,
				con_startup: false,
				con_forcepic: false,
				con_recalc: false,
				consoletoggle: false,
				consoleready: false,
				con_destlines: 0,
				con_curlines: 0,
				con_hudlines: 0,
			 	con_hudtimes: Vec::new(),
				con_clearlines: 0,
				con_hudupdate: false,
				con_line: ' ',	
				con_scalefactor: 1,
				inputlines: Vec::new(),
				inputline: 0,
				inputhist: 0,
				con_buffer: Vec::new(),
		}
	}

	// hellish..
	//
	//
	// No args
	/*
	pub fn printf(&self, msg: &str) {
		//print!("{}", msg);	

		let mut txt: &str = "";
		let mut startup: bool = false;

		// echo console prints to log file
		//DEBFILE(txt);

		// write message in con text buffer
		if self.con_started {
			self.print(txt);
		}

		//CON_LogMessage(txt);

		//Lock_state();

		// Make sure new text is visible
		self.con_scrollup = 0;
		startup = self.con_startup;

		// Unlock_state();

		// WHATIS: setrenderneed ?
		// if not in display loop, force screen update
		if startup && !setrenderneeded {
			//#ifdef windows
			// TODO: ACCEPTS a patch_t
			// TODO: WHATIS W_CachePatchName?
			let mut con_backpic: patch_t =  W_CachePatchName("CONSBACK", PU_PATCH);

			// TODO: WHATIS V_DrawScaledPatch?
			V_DrawScaledPatch(0, 0, 0, con_backpic);
			// TODO: ACCEPTS a ISystem struct
			// windows only
			i_system.loading_screen(txt);
			//#else
			// display console text
			self.drawer();
			i_system.finish_update()
		}
		
	}
	*/

	// args: String array
	/*
	pub fn printf_string(&self, msgs: &[&str], args: [&std::string::String]) {
		// array index
		let mut i: usize = 0;

		for msg in msgs {
			if i < args.len() {
				print!("{}", args[i]);	
				i = i + 1;
			} else {
				print!("{}", msg);
			}
		}
		
		let mut txt: &str = "";
		let mut startup: bool = false;

		// echo console prints to log file
		//DEBFILE(txt);

		// write message in con text buffer
		if self.con_started {
			self.print(txt);
		}

		//CON_LogMessage(txt);

		//Lock_state();

		// Make sure new text is visible
		self.con_scrollup = 0;
		startup = self.con_startup;

		// Unlock_state();

		// WHATIS: setrenderneed ?
		// if not in display loop, force screen update
		if startup && !setrenderneeded {
			//#ifdef windows
			// TODO: ACCEPTS a patch_t
			// TODO: WHATIS W_CachePatchName?
			let mut con_backpic: patch_t =  W_CachePatchName("CONSBACK", PU_PATCH);

			// TODO: WHATIS V_DrawScaledPatch?
			V_DrawScaledPatch(0, 0, 0, con_backpic);
			// TODO: ACCEPTS a ISystem struct
			// windows only
			i_system.loading_screen(txt);
			//#else
			// display console text
			self.drawer();
			i_system.finish_update()
		}
		
	}*/

	// TODO: attempt to make this a variadic function
	// Output text into console text buffer
	pub fn print(&self, msgs: &[&str]) {
		for msg in msgs {
			print!("{}", msg);
		}
		// TODO: ACCEPTS a size_t
		//let mut l: size_t;
		/*
		// for color changing
		let mut controlchars: i32 = 0;
		// keep color across lines
		let mut color: char = "\x80";

		if msg == "" {
			return;
		}

			// chat text, makes ding sound
		if msg == '\3' {
			// WHEREIS: S_StartSound ?
			//S_StartSound(sfx_radio)
		} else if msg == '\4' {
			// chat action, dings and is in yellow
			msg = '\x82';
			S_StartSound(sfx_radio);
		}

		// Lock_state();

		if !(msg & 0x80) {
			self.con_line[con_cx+1] = '\x80';
			controlchars = 1;
		}

	while (*msg)
	{
		// skip non-printable characters and white spaces
		while (*msg && *msg <= ' ')
		{
			if (*msg & 0x80)
			{
				color = con_line[con_cx++] = *(msg++);
				controlchars++;
				continue;
			}
			else if (*msg == '\r') // carriage return
			{
				con_cy--;
				CON_Linefeed();
				color = '\x80';
				controlchars = 0;
			}
			else if (*msg == '\n') // linefeed
			{
				CON_Linefeed();
				con_line[con_cx++] = color;
				controlchars = 1;
			}
			else if (*msg == ' ') // space
			{
				con_line[con_cx++] = ' ';
				if (con_cx - controlchars >= con_width-11)
				{
					CON_Linefeed();
					con_line[con_cx++] = color;
					controlchars = 1;
				}
			}
			else if (*msg == '\t')
			{
				// adds tab spaces for nice layout in console

				do
				{
					con_line[con_cx++] = ' ';
				} while ((con_cx - controlchars) % 4 != 0);

				if (con_cx - controlchars >= con_width-11)
				{
					CON_Linefeed();
					con_line[con_cx++] = color;
					controlchars = 1;
				}
			}
			msg++;
		}

		if (*msg == '\0')
		{
			Unlock_state();
			return;
		}

		// printable character
		for (l = 0; l < (con_width-11) && msg[l] > ' '; l++)
			;

		// word wrap
		if ((con_cx - controlchars) + l > con_width-11)
		{
			CON_Linefeed();
			con_line[con_cx++] = color;
			controlchars = 1;
		}

		// a word at a time
		for (; l > 0; l--)
			con_line[con_cx++] = *(msg++);
	}
	*/
	//Unlock_state();
	}

	// Console refresh drawer, call each frame	
	pub fn drawer(&self) {
		// Lock_state();
		/*
		// WHATIS: graphics_started ??
		if !self.con_started { //|| !graphics_started {
			//Unlock_state();
			//return();
		}

		// WHATIS: needpatchrecache ??
		// WHATIS: HU_LoadGraphics() ??
		// WHEREIS: ^^ ?
		//if needpatchrecache {
		//	HU_LoadGraphics();
		//}

		if self.con_recalc {
			self.recalcsize();
			if self.con_curlines <= 0 {
				self.clearhud();
			}
		}

		if self.con_curlines > 0 {
			self.drawconsole();
			//WHATIS: gamestate ??
		} else if gamestate == GS_LEVEL
		|| gamestate == GS_INTERMISSION || gamestate == GS_ENDING || gamestate == GS_CUTSCENE
		|| gamestate == GS_CREDIT || gamestate == GS_EVALUATION {
			self.drawhudlines();
		}*/
		// Unlock_state();
	}
}