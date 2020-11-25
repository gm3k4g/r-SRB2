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
// But perhaps, I could use this 'variadic' macro with a function.. maybe?
// "" 23/11/2020: Just use write! macro: macro > function

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

pub mod d_main;
use d_main::DMain;

pub mod z_zone;
use z_zone::Zone;

pub mod screen;
use screen::Screen;

// From SDL interface
pub mod sdl;
use sdl::i_main::IMain;
use sdl::i_system::ISystem;
use sdl::i_video::IVideo;
use sdl::mixer_sound::MixerSound;
use sdl::sdlmain::SdlMain;


// 1. Initialize all structs (how is this man?)
// 2. Pass them around (hell) -> i_main function will take ownership of everything
// TODO: create folders SDL, Dummy, Hardware, etc. for respective interfaces ('ports or interfaces')

// TODO: Pass things around = Pass references, passing values means functions will 'consume' the variables!!
fn main() {
	// commandline args
	let argv: Vec<String> = std::env::args().collect();

	// From SDL interface: initialization
	let i_system = ISystem::new();
	let i_video = IVideo::new();
	let mixer_sound = MixerSound::new();
	let sdlmain = SdlMain::new();

	// Base code: initialization
	let command = Command::new();
	let m_misc = Misc::new();
	let console = Cons::new();
	let d_main = DMain::new();
	let m_argv = MArgv::new(argv);
	let g_game = Game::new();
	let dehacked = Dehacked::new();
	let sounds = Sounds::new();
	let z_zone = Zone::new();
	let screen = Screen::new();

	// the 'main' file
	let mut i_main = IMain::new();

	// Let the program begin
	// (i_main takes ownership of everything)
	i_main.i_main(
		m_argv, console, d_main, m_misc, 
		g_game, sounds, dehacked, i_system,
		i_video, mixer_sound, z_zone, command,
		sdlmain, screen
	);
}