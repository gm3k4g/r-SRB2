 ///////////// BEGINNING OF d_main.h //////////////
// \file  d_main.h
// \brief game startup, and main loop code, system specific interface stuff.

//#include "d_event.h"
//#include "w_wad.h"   // for MAX_WADFILES
/*
let advancedemo: bool; //extern boolean advancedemo;

// make sure not to write back the config until it's been correctly loaded
let rendergametic: tic_t;//extern tic_t rendergametic;

let srb2home: std::String;//extern char srb2home[256]; //Alam: My Home
let usehome: bool;//extern boolean usehome; //Alam: which path?
let pandf: &str;//extern const char *pandf; //Alam: how to path?
//extern char srb2path[256]; //Alam: SRB2's Home
*/
// the infinite loop of D_SRB2Loop() called from win_main for windows version
//void D_SRB2Loop(void) FUNCNORETURN;

//
// D_SRB2Main()
// Not a globally visible function, just included for source reference,
// calls all startup code, parses command line options.
// If not overrided by user input, calls D_AdvanceDemo.
//
//void D_SRB2Main(void);

// Called by IO functions when input is detected.
//void D_PostEvent(const event_t *ev);
//#if defined (PC_DOS) && !defined (DOXYGEN)
//void D_PostEvent_end(void);    // delimiter for locking memory
//#endif

//void D_ProcessEvents(void);

//const char *D_Home(void);

//
// BASE LEVEL
//
//void D_AdvanceDemo(void);
//void D_StartTitle(void);

//#endif //__D_MAIN__


 ///////////// END OF d_main.h /////////////////////



///// BEGINNING OF d_main.c ///
/*
#if (defined (__unix__) && !defined (MSDOS)) || defined(__APPLE__) || defined (UNIXCOMMON)
#include <sys/stat.h>
#include <sys/types.h>
#endif

#ifdef __GNUC__
#include <unistd.h> // for getcwd
#endif

#ifdef _WIN32
#include <direct.h>
#include <malloc.h>
#endif

#include <time.h>

#include "doomdef.h"
#include "am_map.h"
#include "console.h"
#include "d_net.h"
#include "f_finale.h"
#include "g_game.h"
#include "hu_stuff.h"
#include "i_sound.h"
#include "i_system.h"
#include "i_threads.h"
#include "i_video.h"
#include "m_argv.h"
#include "m_menu.h"
#include "m_misc.h"
#include "p_setup.h"
#include "p_saveg.h"
#include "r_main.h"
#include "r_local.h"
#include "s_sound.h"
#include "st_stuff.h"
#include "v_video.h"
#include "w_wad.h"
#include "z_zone.h"
#include "d_main.h"
#include "d_netfil.h"
#include "m_cheat.h"
#include "y_inter.h"
#include "p_local.h" // chasecam
#include "mserv.h" // ms_RoomId
#include "m_misc.h" // screenshot functionality
#include "dehacked.h" // Dehacked list test
#include "m_cond.h" // condition initialization
#include "fastcmp.h"
#include "keys.h"
#include "filesrch.h" // refreshdirmenu, mainwadstally
#include "g_input.h" // tutorial mode control scheming
#include "m_perfstats.h"

#ifdef CMAKECONFIG
#include "config.h"
#else
#include "config.h.in"
#endif

#ifdef HWRENDER
#include "hardware/hw_main.h" // 3D View Rendering
#endif

#ifdef _WINDOWS
#include "win32/win_main.h" // I_DoStartupMouse
#endif

#ifdef HW3SOUND
#include "hardware/hw3sound.h"
#endif

#include "lua_script.h"

*/
macro_rules! scan {
	( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
		let mut iter = $string.split($sep);
		($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
	}}
}
// Version numbers for netplay :upside_down_face:
//int    VERSION;
//int SUBVERSION;
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
//const SUBVERSION


fn main() {
    d_srb2main()
}

fn d_srb2main() {
	println!("Hello world");

	let p : i32;
	let pstartmap: i32 = 1;
	let autostart: bool = false;

	let mut version = VERSION.unwrap();
	let mut subversion: i32;

	// Break version string into version numbers; for netplay
	d_convertversionnumbers();

	/*
	CONS_Printf(
		"\n\nSonic Robo Blast 2\n"
		"Copyright (C) 1998-2020 by Sonic Team Junior, nil\n\n"
		"This program comes with ABSOLUTELY NO WARRANTY.\n\n"
		"This is free software, and you are welcome to redistribute it\n"
		"and/or modify it under the terms of the GNU General Public License\n"
		"as published by the Free Software Foundation; either version 2 of\n"
		"the License, or (at your option) any later version.\n"
		"See the 'LICENSE.txt' file for details.\n\n"
		"Sonic the Hedgehog and related characters are trademarks of SEGA.\n"
		"We do not claim ownership of SEGA's intellectual property used\n"
		"in this program.\n\n");
	*/


	// initialise locale code
	// m_startuplocale();

	// get params from a response file (i.e. srb2 @parms.txt)
	// m_findresponsefile();

	// maincfg is now taken care of where objctcfg is handled
	// g_loadgamesettings();

	// test dehacked lists
	// deh_check();

	// netgame url special case: change working dir to exe folder.
	//changedirforurlhandler();

	// identify the main iwad file to use
	// identifyversion();
/*
	#if !defined(NOTERMIOS)
		setbuf(stdout, NULL); // non-buffered output
	#endif*/

	/*#if 0 //defined (_DEBUG)
	devparm = M_CheckParm("-nodebug") == 0;
	#else*/
	//let devparm: bool = M_CheckParm("-debug") != 0;
//	#endif
	
	// for dedicated server
	/*
	#if !defined (_WINDOWS) //already check in win_main.c
		dedicated = M_CheckParm("-dedicated") != 0;
	#endif
	*/




}


//static void
fn d_convertversionnumbers() {
	let major: i32;
	let minor: i32;
	println!("{:?}", scan!(VERSION.unwrap(), char::is_ascii, i32, i32, i32));

}

//// END