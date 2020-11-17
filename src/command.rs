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

pub struct CvPossibleValueT {
	pub value: i32,
	pub strvalue: String,
}
impl CvPossibleValueT {
	pub fn new() -> Self {
		CvPossibleValueT {
			value: 0,
			strvalue: String::from(""),
		}
	}
}

//
//what
//
//NULL, NULL, 0, NULL, NULL |, 0, NULL, NULL, 0, 0, NULL
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
		ConsVarT {
			name: String::from(""),
			defaultvalue: String::from(""),
			flags: 0,
			possible_value: CvPossibleValueT::new(),

			value: 0,
			string: String::from(""),
			zstring: String::from(""),

			netid: 0,
			changed: ' ',

			next: None,
		}
	}
}

pub struct OldDemoVar {
	pub checksum: u16, //UINT16
	pub collides: bool,

	pub cvar: ConsVarT,
	// Finite size
	pub next: Option<Box<OldDemoVar>>,
}




pub struct Command {
	// TODO: are these arrays?
	pub cv_onoff: CvPossibleValueT, //CV_PossibleValue_t CV_OnOff[];
	pub cv_yesno: CvPossibleValueT,
	pub cv_unsigned: CvPossibleValueT,
	pub cv_natural: CvPossibleValueT,

	// Filter consvars by version
	pub cv_execversion: ConsVarT,

}
impl Command {
	pub fn new() -> Self {
		Command {
			cv_onoff: CvPossibleValueT::new(),
			cv_yesno: CvPossibleValueT::new(),
			cv_unsigned: CvPossibleValueT::new(),
			cv_natural: CvPossibleValueT::new(),
			cv_execversion: ConsVarT::new(),	
		}
	}
}