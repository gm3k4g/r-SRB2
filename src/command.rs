// For SDL interface
use crate::sdl::i_main::IMain;
use crate::sdl::i_system::ISystem;
use crate::sdl::i_video::IVideo;
use crate::sdl::mixer_sound::MixerSound;

//===================================
// Command buffer & command execution
//===================================

/* Lua command registration flags. */
enum CommandLua {
	ComAdmin 		= 1,
	ComSplitscreen  = 2,
	ComLocal		= 4,
}

/* Command buffer flags. */
enum CommandBuffer {
	ComSafe = 1,
}

// ======================
// Variable sized buffers
// ======================
pub struct VsbufT {
	pub allowoverflow: bool,
	pub overflowed: bool,
	pub data: u8, // UINT8 *data;
	// TODO: is the equivalent of size_t in rust `isize`?
	pub maxsize: isize,
	pub cursize: isize,
}
impl VsbufT {
	// TODO check out command.c
	/*
void VS_Alloc(vsbuf_t *buf, size_t initsize);
void VS_Free(vsbuf_t *buf);
void VS_Clear(vsbuf_t *buf);
void *VS_GetSpace(vsbuf_t *buf, size_t length);
void VS_Write(vsbuf_t *buf, const void *data, size_t length);
void VS_WriteEx(vsbuf_t *buf, const void *data, size_t length, int flags);
void VS_Print(vsbuf_t *buf, const char *data); // strcats onto the sizebuf

	*/
}

//==================
// Console variables
//==================
// console vars are variables that can be changed through code or console,
// at RUN TIME. They can also act as simplified commands, because a func-
// tion can be attached to a console var, which is called whenever the
// variable is modified (using flag CV_CALL).

// flags for console vars
enum CVFlagsT {
	CvSave = 1,   // save to config when quit game
	CvCall = 2,   // call function on change
	CvNetvar = 4, // send it when change (see logboris.txt at 12-4-2000)
	CvNoinit = 8, // dont call function when var is registered (1st set)
	CvFloat = 16, // the value is fixed 16 : 16, where unit is FRACUNIT
	               // (allow user to enter 0.45 for ex)
	               // WARNING: currently only supports set with CV_Set()
	CvNotinnet = 32,    // some varaiable can't be changed in network but is not netvar (ex: splitscreen)
	CvModified = 64,    // this bit is set when cvar is modified
	CvShowmodif = 128,  // say something when modified
	CvShowmodifonetime = 256, // same but will be reset to 0 when modified, set in toggle
	CvNoshowhelp = 512, // Don't show variable in the HELP list Tails 08-13-2002
	CvHiden = 1024, // variable is not part of the cvar list so cannot be accessed by the console
	                 // can only be set when we have the pointer to it
                   // used on menus
	CvCheat = 2048, // Don't let this be used in multiplayer unless cheats are on.
	CvNolua = 4096,/* don't let this be called from Lua */
}

#[derive(Default)]
pub struct CvPossibleValueT {
	pub value: i32,
	pub strvalue: String,
}
impl CvPossibleValueT {
	pub fn new() -> Self {
		Self::default()
	}
}

//
//what
//
//NULL, NULL, 0, NULL, NULL |, 0, NULL, NULL, 0, 0, NULL
#[derive(Default)]
pub struct ConsVarT {
	pub name: String,
	pub defaultvalue: String,
	pub flags: i32,
	pub possible_value: CvPossibleValueT,
	//what
	//void (*func)(void);   // called on change, if CV_CALL set 
	pub value: i32,
	pub string: String,
	pub zstring: String,

	pub netid: u16,
	pub changed: char,
	// Finite size
	pub next: Option<Box<ConsVarT>>, // access like *next (smart pointer) or .whatever()
}
impl ConsVarT {
	pub fn new() -> Self {
		Self::default()
	}
}

pub struct OldDemoVar {
	pub checksum: u16, //UINT16
	pub collides: bool,

	pub cvar: ConsVarT,
	// Finite size
	pub next: Option<Box<OldDemoVar>>,
}


// =========================================================================
//                            COMMAND EXECUTION
// =========================================================================

#[derive(Default)]
pub struct XCommandT {
	pub name: String,
	pub next: Option<Box<XCommandT>>
}
impl XCommandT {
	pub fn new() -> Self {
		Self::default()
	}
}
// TODO: implement a structure that has .inter() method for going through a 'list'?
/*
impl Iterator for XCommandT {

	// When iterator finishes, None returns
	// Otherwise, next value is wrapped in 'Some' and returned
	fn next(&mut self) -> Option<u32> {

	}
}*/


#[derive(Default)]
pub struct Command {
	// TODO: are these arrays?
	pub cv_onoff: CvPossibleValueT, //CV_PossibleValue_t CV_OnOff[];
	pub cv_yesno: CvPossibleValueT,
	pub cv_unsigned: CvPossibleValueT,
	pub cv_natural: CvPossibleValueT,

	// Filter consvars by version
	pub cv_execversion: ConsVarT,

	// current commands
	pub com_commands: XCommandT,

	// =========================================================================
	//
	//                           CONSOLE VARIABLES
	//
	//   console variables are a simple way of changing variables of the game
	//   through the console or code, at run time.
	//
	//   console vars acts like simplified commands, because a function can be
	//   attached to them, and called whenever a console var is modified
	//
	// =========================================================================
	pub cv_null_string: String

}
impl Command {
	pub fn new() -> Self {
		Self::default()
	}

	/* 	* Add a console command.
		*
		* param name: Name of the command
		* param func: Function called when the command is run
		*/
	pub fn com_addcommand(&mut self,
		name: &str, 
		//_func: fn(), //why?
		i_system: &mut ISystem,
		i_video: &IVideo,
		mixer_sound: &MixerSound,
		i_main: &mut IMain
		) {
		let cmd: XCommandT = XCommandT::new();

		// fail if command is a variable name
		match self.cv_string_value(name.to_string()).chars().next() {
			Some(character) => {
				if character != '\0' {
					i_system.i_error(&[name, " is a variable name\n"], &i_video, &mixer_sound, i_main);
					return;
				}
			},
			None => {
				cons_printf!("com_addcommand(): no character detected!\n");
			},
		};

		//fail if the command already exists
		// TODO: go through cmd's `next` field until the end

		// function is passed to cmd
		// i_video.vid_command_nummodes_f();
		cons_printf!("TODO: make com_addcommand() work!\n");
	}

	/*	* Registers a variable for later use from the console
		*
		* param variable: the variable to register
		*/
	pub fn cv_register_var(&self, variable: &ConsVarT) {
		// Check if already was defined
		match self.cv_find_var(variable.name.clone()) {
			Some(consvar) => {
				cons_printf!("Variable ", consvar.name, " is already defined\n");
				return;
			},
			None => {},
		}
	}


	/* 	* Initializes command buffer and adds basic commands

		*/
	pub fn com_init(&self) {
		// allocate command buffer
		cons_printf!("TODO: make com_init() work!\n");
	}

	// =========================================================================
	//                      VARIABLE SIZE BUFFERS
	// =========================================================================

	/*	* Initialize a 'variable' size buffer?
		* 
		* param buf: 		The buffer to initialize.
		* param initsize: 	The initial size for the buffer.
		*/
	pub fn vs_alloc(&self/*, Vec<?>: buf, initsize: usize?*/) {
		// WHEREIS: VSBUFMINSIZE
		// allocate command buffer
		cons_printf!("TODO: make vs_alloc() work!\n");
	}

	/*	* Finds the string value of a console variable
		*
		* param var_name: The variable's name	
		* return the string value or "" if variable not found.
		*/
	pub fn cv_string_value(&self, _var_name: String) -> String {
		let var: Option<ConsVarT> = None; //fixme

		cons_printf!("FIX_ME: cv_string_value() \n");
		//var = self.cv_find_var(var_name);
		match var {
			Some(variable) => {
				return variable.string;
			}
			None => {
				return self.cv_null_string.clone();
			}
		}
	}

	/*	* Searches if a variable has been registered
		* 
		* param name: Variable to search for
		* return Variable if found, otherwise None
		*/
	pub fn cv_find_var(&self, _name: String) -> Option<ConsVarT> {
		cons_printf!("TODO: make cv_find_var() work!\n");
		/*let mut cvar: ConsVarT = self.consvar_vars;

		while cvar == cvar.next {

		}*/
		None
	}
}