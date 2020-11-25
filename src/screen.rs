
//ifdef NOPOSTPROCESSING
pub const NUMSCREENS: i32 = 2;
//else
//pub const NUMSCREENS: i32 = 5;
//end

pub const ST_HEIGHT: i32 = 32;
pub const ST_WIDTH: i32 = 320;

pub const MAXVIDWIDTH: i32 = 1920; 	// don't set this too high because actually
pub const MAXVIDHEIGHT: i32 = 1200; // lots of tables are allocated with the MAX size.

pub const BASEVIDWIDTH: i32 = 320;	// NEVER CHANGE THIS! This is the original
pub const BASEVIDHEIGHT: i32 = 200;	// resolution of the graphics.

// TODO: find me a better name
pub enum OpenGLThing { // don't need numpages for OpenGL, so we can use it for fullscreen/windowed mode
	NumPages= 1,  // always 1, page flipping todo
	Windowed 	  // windowed or fullscren mode?
}
impl Default for OpenGLThing {
	fn default() -> Self {
		crate::screen::OpenGLThing::NumPages
	}
}

#[derive(Default)]
pub struct VidDefT {
	pub modenum: i32, // vidmode num indexes videomodes list

	pub buffer: u8, // invisible screens buffer
	pub width: i32, // bytes per scanline of the VIDEO mode
	pub height: i32,

	//union -> ??? Use enum..
	/*
		
	*/
	pub u: OpenGLThing,

	pub recalc: i32, // if true, recalc vid-based stuff
	pub direct: u8, // linear frame buffer, or vga base mem.
	pub dupx: i32, // scale 1, 2, 3 value for menus & overlays
	pub dupy: i32,

	pub fdupx: i32, // same as dupx, dupy, but exact value when aspect ratio isn't 320/200
	pub fdupy: i32,

	pub bpp: i32, // BYTES per pixel: 1 = 256color, 2 = highcolor
	pub baseratio: i32, // Used to get the correct value for lighting walls

	// For win32 ver.
	//what
	//DNWH WndParent;  // handle of the application's window
	pub smalldupx: u8, // factor for a little bit of scaling
	pub smalldupy: u8,
	pub meddupx: u8, // factor for moderate, but not full, scaling
	pub meddupy: u8,

	//ifdef hwrender
	pub fsmalldupx: i32,
	pub fsmalldupy: i32,

	pub fmeddupx: i32,
	pub fmeddupy: i32,
	//endif
}
impl VidDefT {
	pub fn new() -> Self {
		Self::default()
	}
}

//#define VIDWIDTH vid.width
//#define VIDHEIGHT vid.height

#[derive(Default)]
pub struct VesaExtraT {
	pub vesamode: i32 // vesa mode number plus LINEAR_MODE bit
	// void *plinearmem; // linear address of start of frame buffer
}
impl VesaExtraT {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Default)]
pub struct VModeT {
	pub pnext: Option<Box<VModeT>>,
	pub name: String,
	
	pub width: u32,
	pub height: u32,

	pub rowbytes: u32, // bytes per scanline
	pub bytesperpixel: u32,

	pub windowed: i32,
	pub numpages: i32,
	pub pextradata: VesaExtraT,

	/*
	#ifdef _WIN32
	INT32 (WINAPI *setmode)(viddef_t *lvid, struct vmode_s *pcurrentmode);
	#else
	INT32 (*setmode)(viddef_t *lvid, struct vmode_s *pcurrentmode);
	#endif
	*/
	pub misc: i32
}
impl VModeT {
	pub fn new() -> Self {
		Self::default()
	}
}

pub const NUMSPECIALMODES: i32 = 4;
//extern vmode_t specialmodes[NUMSPECIALMODES];

pub const BASEDRAWFUNC: isize = 0;
pub enum ColDrawFunc {
	ColDrawFuncBase = BASEDRAWFUNC,
	ColDrawFuncFuzzy,
	ColDrawFuncTrans,
	ColDrawFuncShade,
	ColDrawFuncShadowed,
	ColDrawFuncTranstrans,
	ColDrawFuncTwosmultipatch,
	ColDrawFuncTwosmultipatchtrans,
	ColDrawFuncFog,

	ColDrawFuncMax
}

pub enum SpanDrawFunc {
	SpanDrawFuncBase = BASEDRAWFUNC,
	SpanDrawFuncTrans,
	SpanDrawFuncSplat,
	SpanDrawFuncTranssplat,
	SpanDrawFuncFog,
//#ifndef NOWATER
	SpanDrawFuncWater,
//#endif
	SpanDrawFuncTilted,
	SpanDrawFuncTiltedtrans,
	SpanDrawFuncTiltedsplat,
//#ifndef NOWATER
	SpanDrawFuncTiltedwater,
//#endif

	SpanDrawFuncMax
}

// -----
// CPUID
// -----
/*
extern boolean R_ASM;
extern boolean R_486;
extern boolean R_586;
extern boolean R_MMX;
extern boolean R_3DNow;
extern boolean R_MMXExt;
extern boolean R_SSE2;
*/

#[derive(Default)]
pub struct Screen {
	// ------------------
	// global video state
	// ------------------
	pub vid: VidDefT,
	pub setmodeneeded: i32,
	pub setrenderneeded: u8,

	//pub scr_depth_cons_t: Vec<ConsVarT>,

	//pub cv_scr_width: ConsVarT,
	//pub cv_scr_height: ConsVarT,
	//pub cv_scr_depth: ConsVarT,
	//pub cv_scr_renderview: ConsVarT
}
impl Screen {
	pub fn new() -> Self {
		Self::default()
	}
}