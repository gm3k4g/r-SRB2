use crate::g_state::GameActionT;
use crate::g_state::GameStateT;

use crate::sounds::Sounds;

// enums in C are essentially typedef'd integers..
bitflags! {
	pub struct ControlStyle: u32 {
		const CS_LEGACY 	= 0;
        const CS_LMAOGALOG 	= 1;
        const CS_STANDARD 	= 2;
        const CS_SIMPLE = Self::CS_LEGACY.bits | Self::CS_STANDARD.bits;
	}
}

bitflags! {
	pub struct LockAssist: u32 {
		const LOCK_BOSS = 1<<0;
		const LOCK_ENEMY = 1<<1;
		const LOCK_INTERESTS = 1<<2;
	}
}

pub struct SearchDim {
	pub pos: u8, //UINT8
	pub siz: u8, //UINT8
}

pub struct MapSearchFreq {
	pub mapnum: i16,
	pub matchc: u8,
	pub matchd: SearchDim, /* offset that a pattern was matched */
	pub keywhc: u8,
	pub keywhd: SearchDim, /* ...in KEYWORD */
	pub total: u8, /* total hits */
}

pub struct JoyStickVector2T {
	pub xaxis: i32,
	pub yaxis: i32,
}

pub enum AxisInput {
	AxisNone = 0,
	AxisTurn,
	AxisMove,
	AxisLook,
	AxisStrafe,

	AxisDigital, // axes belowt his use digital deadzone

	AxisJump,
	AxisSpin,
	AxisFire,
	AxisFireNormal,
}

// long butt list of fields so since
// we're lazy we use Default to initialize everything
#[derive(Default)]
pub struct Game {
	pub gameaction: GameActionT,
	pub gamestate: GameStateT,
	pub ultimatemode: u8, //UINT8

	pub botingame: bool,
	pub botskin: u8,
	pub botcolor: u16,

	// TODO: ACCEPTS a JoyTypeT
	//pub joystick: JoyTypeT,
	//pub joystick2: JoyTypeT,

	// 1024 bytes is plenty for a savegame
	//#define SAVEGAMESIZE (1024)

	pub gamedatafilename: String,
	pub timeattackfolder: String,
	pub customversionstring: String,

	pub mapmusname: String,
	pub mapmusflags: u16,
	pub mapmusposition: u32,

	pub gamemap: i16,
	pub maptol: u32,
	pub globalweather: u8,
	pub cur_weather: i32,
	pub cur_save_slot: i32,
	//INT16 lastmapsaved = 0; // Last map we auto-saved at
	pub last_map_loaded: i16,
	pub gamecomplete: u8,

	// TODO: ACCEPTS a MarathonModeT
	//pub marathonmode: MarathonModeT,
	// TODO: ACCEPTS a tic_t
	//pub marathontime: tic_t,

	pub numgameovers: u8,
	//uhhhhhhhhhhh
	pub starting_lives_balance: i8, //SINT8 startinglivesbalance[maxgameovers+1] = {3, 5, 7, 9, 12, 15, 20, 25, 30, 40, 50, 75, 99, 0x7F};

	pub mainwads: i16,
	pub modifiedgame: bool,
	pub savemoddata: bool,
	pub paused: u8,
	pub modeattacking: u8,
	pub disable_speed_adjust: bool,
	pub imcontinuing: bool,
	pub runemeraldmanager: bool,

	pub emeraldspawndelay: u16,

	//menu demo things
	pub num_demos: u8,
	pub demo_delay_time: u32,
	pub demo_idle_time: u32,

	pub netgame: bool,
	pub multiplayer: bool,
	// todo: is this array?
	pub playeringame: bool,
	pub addedtogame: bool,
	//todo: is this array?
	// TODO: ACCEPTS a PlayerT
	//pub players: PlayerT

	pub consoleplayer: i32,	// player taking events and displaying
	pub displayplayer: i32,	// view being displayed
	pub secondarydisplayplayer: i32, //splitscreen

	//pub gametic: TicT,
	//pub levelstarttic: TicT, //gametic at level start
	pub ssspheres: u32, //old special stage
	pub lastmap: i16, // last level you were at (returning from special stages)
	// pub timeinmap: TicT //Ticker for time spent in level (used for levelcard display)

	pub spstage_start: i16,
	pub spmarathon_start: i16,
	pub sstage_start: i16,
	pub sstage_end: i16,
	pub smpstage_start: i16,
	pub smpstage_end: i16,

	pub titlemap: i16,
	pub hidetitlepics: bool,
	pub bootmap: i16, // bootmap for loading a map on startup

	pub tutorialmap: i16, //map to load for tutorial
	pub tutorialmode: bool, // are we in tutorial right now?
	pub tutorialgcs: i32, // Which control scheme is loaded?
	pub tutorialusemouse: i32, //store cv_usemouse user value
	pub tutorialfreelook: i32, //store cv_alwaysfreelook user value
	pub tutorialmousemove: i32, //store cv_mousemove user value
	pub tutorialanalog: i32, //store cv_analog[0] user value

	pub looptitle: bool,

	pub skincolor_redteam: u16,
	pub skincolor_blueteam: u16,
	pub skincolor_redring: u16,
	pub skincolor_bluering: u16,

	//pub countdowntimer: TicT,
	pub countdowntimeup: bool,
	pub exitfadestarted: bool,

	// todo: is this array?
	// TODO: ACCEPTS a CutSceneT
	//pub cutscenes: CutSceneT,
	// todo: is this array?
	// TODO: ACCEPTS a TextPromptT
	//pub textprompts: TextPromptT,

	pub nextmapoverride: i16,
	pub skipstats: u8,

	// Pointers to each CTF flag
	//TODO: ACCEPTS a MobjT
	//pub redflag: MobjT,
	//pub blueflag: MobjT,
	// Pointers to CTF spawn location
	//TODO: ACCEPTS a MapThingT
	//pub rflagpoint: MapThingT,
	//pub bflagpoint: MapThingT,

	// WHATIS: THIS?
	//struct quake quake

	// Map Header Information
	//todo: is this array?
	//TODO: ACCEPTS A MapHeaderT
	//pub mapheaderinfo: MapHeaderT,

	pub exitgame: bool,
	pub retrying: bool,
	pub retryingmodeattack: bool,

	pub stagefailed: u8, // used for GEMS bonus? Also to see if you beat the stage.

	pub emeralds: u16,
	// TODO: is this array?
	pub luabanks: i32,
	pub token: u32, // Number of tokens collected in a level
	pub tokenlist: u32, //List of tokens collected
	pub gottoken: bool, //Did you get a token? Used for end of act
	pub tokenbits: i32, // Used for setting token bits

	// Old Special Stage
	pub sstimer: i32, //Time allotted in the special stage

	// pub totalplaytime: TicT,
	pub gamedataloaded: bool,

	// Time attack data for levels
	// These are dynamically allocated for space reasons now
	// ...?
	// TODO: ACCEPTS a RecordDataT
	//todo: is this array?
	//pub mainrecords: RecordDataT,
	// TODO: ACCEPTS a NightsDataT,
	//todo: is this array?
	//pub nightsrecords: NightsDataT,
	pub mapvisited: u8,

	// Temporary holding place for nights data for the current map
	//pub ntemprecords: NightsDataT,

	// CTF and Team Match team scores
	pub bluescore: u32,
	pub redscore: u32,

	// Ring count for PERFECT bonus
	pub nummaprings: i32,

	// Eliminates unnecessary searching
	pub check_for_bustable_blocks: bool,
	pub check_for_bouncy_sector: bool,
	pub check_for_quicksand: bool,
	pub check_for_mario_blocks: bool,
	pub check_for_float_bob: bool,
	pub check_for_reverse_gravity: bool,

	// Powerup Durations
	pub invultics: u16,
	pub sneakertics: u16,
	pub flashingtics: u16,
	pub tailsflytics: u16,
	pub underwatertics: u16,
	pub spacetimetics: u16,
	pub extralifetics: u16,
	pub nightslinktics: u16,

	pub gameovertics: i32,

	pub ammoremovaltics: u8,

	pub use1up_sound: u8,
	pub max_xtra_life: u8, // Maximum extra lives from rings
	pub use_continues: u8, // Set to 1 to enable continues outside of no-save scenarios

	pub introtoplay: u8,
	pub creditcutscene: u8,
	pub use_black_rock: u8,

	// Emerald locations
	// pub hunt1: MobjT,
	// pub hunt2: MobjT,
	// pub hunt3: MobjT,

	// For racing
	pub countdown: u32,
	pub countdown2: u32,

	// TODO: ACCEPTS a fixed_t
	// pub: gravity: FixedT,

	pub autobalance: i16, //for CTF team balance
	pub teamscramble: i16, //for CTF team scramble,
	//todo: is this array?
	pub scrambleplayers: i16, //for CTF team scramble
	//todo: is this array?
	pub scrambleteams: i16, //for CTF team scramble
	pub scrambletotal: i16, //for CTF team scramble
	pub scrambecount: i16, //for CTF team scramble

	pub cheats: i32, //for multiplayer cheat commands

	//pub hidetime: TicT,

	// Grading
	pub times_beaten: u32,
	pub times_beaten_with_emeralds: u32,
	pub times_beaten_ultimate: u32,

	pub precache: bool, //if true, load all graphics at start

	pub prevmap: i16,
	pub nextmap: i16,

	pub savebuffer: u8,

	// TODO: continue from line 280 of `g_game.c`
	// for consvar_t and more fields to put here

	//#ifdef SEENAMES
	// pub seenplayer: PlayerT, // player we're aiming at right now
	//#endif

	// now automatically allocated in D_RegisterClientCommands
	// so that it doesn't have to be updated depending on the value of MAXPLAYERS
	//todo: is this array? 
	pub player_names: char, //char player_names[MAXPLAYERS][MAXPLAYERNAME+1];

	// todo: is this array?
	pub player_name_changes: i32, //INT32 player_name_changes[MAXPLAYERS];

	// todo: is this array?
	pub rw_maximums: i16,
	/*	INT16 rw_maximums[NUM_WEAPONS] =
		{
			800, // MAX_INFINITY
			400, // MAX_AUTOMATIC
			100, // MAX_BOUNCE
			50,  // MAX_SCATTER
			100, // MAX_GRENADE
			50,  // MAX_EXPLOSION
			50   // MAX_RAIL
		};
	*/
}
impl Game {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn g_load_game_settings(&mut self, sounds: Sounds) {
		//defaults
		self.spstage_start = 1;
		self.spmarathon_start = 1;
		self.sstage_start = 50;
		self.sstage_end = 56; //7 special stages in vanilla SRB2
		self.sstage_end = self.sstage_end + 1; // plus one weirdo
		self.smpstage_start = 60;
		self.smpstage_end = 66; // 7 multiplayer special stages

		// initialize free sfx slots for skin sounds
		sounds.init_runtime_sounds();
	}

}
