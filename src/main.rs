pub mod cmd_args;
pub mod version;

pub mod m_misc;
use m_misc::Misc;

pub mod m_argv;
use m_argv::MArgv;

pub mod console;
use console::Cons;

pub mod i_main;
use i_main::IMain;

pub mod d_main;
use d_main::DMain;

// 1. Initialize all structs (how is this man?)
// 2. Pass them around (hell)
fn main() {
	// commandline args
	let argv: Vec<String> = std::env::args().collect();

	// Initialize everything
	let console = Cons::new();
	let d_main = DMain::new();
	let m_argv = MArgv::new(argv);
	let i_main = IMain::new();

	// Let the program begin
	i_main.main(m_argv, console, d_main);
}