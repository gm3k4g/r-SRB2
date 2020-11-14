static QUIT_FUNCS: Vec<fn(&[String])> = std::vec::Vec::new();

static mut RETURN_WAD_PATH: [char; 256] = [' '; 256]; //empty?

static mut JOYSTICK_STARTED: i32 = 0;

static mut JOYSTICK2_STARTED: i32 = 0;

// have_termios
static FDMOUSE2: i32 = -1;
static MOUSE2_STARTED: i32 = 0;

// UINT8 = false
static KEYBOARD_STARTED: bool = false;

pub fn i_startup_system() -> i32 {
	//i_start_threads();

	//i_add_exit_func(i_stop_threads);

	//i_startup_console();

	//i_fork();

	//i_register_signals();

	// 	Get the version of SDL that is linked against your program.
	//i_output_msg("Linked with SDL version: {}");
	println!("Linked with SDL version: {}",sdl2::version::version());
	// TODO: also want to get the version of SDL that is compiled against your program?

	// TODO: matching
	// let sdl_context = sdl2::init().unwrap();

	// i_setup_mumble();

	0
}