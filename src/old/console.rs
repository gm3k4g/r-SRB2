static mut con_started: bool = false;
static mut con_startup: bool = false;
static mut con_forcepic: bool = true;
static mut con_recalc: bool = false; //set true when screen size has changed

//static mut con_tick: tic_t;

pub fn cons_print(msg: &mut str) {
	let l: usize;
	let controlchars: i32 = 0; 	// Color changing
	//let color: char = '\x80';	// keep color across lines

/* //if pointing to NIL?
	if msg == "" {
		return
	}
*/

/*
	if msg == "\3" {
		s_startsound(None, SFX_RADIO);
	} else if msg == "\4" {
		msg = '\x82'; //yellow
		s_startsound(None, SFX_RADIO);
	}

	*/

	// lock_state;

}

pub fn cons_printf() {

	let startup: bool;

}

