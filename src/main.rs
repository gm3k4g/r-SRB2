// Variadic-like function but it's not a function, it's a macro : 
#[macro_use]
macro_rules! cons_printf {
    ($($args:expr),*) => {{
        $(
            print!("{}", $args);
        )*
    }}
}	

// Bitflags: because rust enums != C enums
#[macro_use]
extern crate bitflags;

// TODO: remember, cons_printf!() macro does NOT have the same
// functionality as CONS_Printf()!!! Fix this! -> idk man, variadic
// functions don't really even exist in rust... 
//
// But perhaps, I could use this 'variadic' macro with a function.. maybe.

// TODO: declare the following with a 'hierarchy'?
pub mod cmd_args;
pub mod version;

pub mod m_misc;
use m_misc::Misc;

pub mod command;
use command::Command;

pub mod m_argv;
use m_argv::MArgv;

pub mod dehacked;
use dehacked::Dehacked;

pub mod console;
use console::Cons;

pub mod sounds;
use sounds::Sounds;

pub mod g_game;
use g_game::Game;

pub mod g_state;

pub mod i_main;
use i_main::IMain;

pub mod d_main;
use d_main::DMain;

pub mod z_zone;
use z_zone::Zone;

// From SDL interface
pub mod sdl;
use sdl::i_system::ISystem;
use sdl::i_video::IVideo;
use sdl::mixer_sound::MixerSound;


// 1. Initialize all structs (how is this man?)
// 2. Pass them around (hell)
// TODO: create folders SDL, Dummy, Hardware, etc. for respective interfaces ('ports or interfaces')

fn main() {
	// commandline args
	let argv: Vec<String> = std::env::args().collect();

	// From SDL interface: initialization
	let i_system = ISystem::new();
	let i_video = IVideo::new();
	let mixer_sound = MixerSound::new();

	// Base code: initialization
	let _command = Command::new();
	let m_misc = Misc::new();
	let console = Cons::new();
	let d_main = DMain::new();
	let m_argv = MArgv::new(argv);
	let g_game = Game::new();
	let dehacked = Dehacked::new();
	let sounds = Sounds::new();
	let z_zone = Zone::new();

	// the 'main' file
	let mut i_main = IMain::new();

	// Let the program begin
	i_main.main(
		m_argv, console, d_main, m_misc, 
		g_game, sounds, dehacked, i_system,
		i_video, mixer_sound, z_zone
	);
}