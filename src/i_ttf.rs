/*
#if defined(HAVE_SDL) && defined(HAVE_TTF)
#include "SDL.h"
#include "SDL_ttf.h"
#include "../doomdef.h"
#include "../doomstat.h"
#include "../d_netfil.h"
#include "../filesrch.h"
#include "i_ttf.h"

*/

const FONTSEARCHPATH1: &'static str = "/usr/share/fonts";
const FONTSEARCHPATH2: &'static str = "/usr/local/share/fonts";
const FONTSEARCHPATH3: &'static str = "/usr/games/SRB2";
const FONTSEARCHPATH4: &'static str = "/usr/local/games/SRB2";
const FONTSEARCHPATH5: &'static str = "/usr/local/share/games/SRB2";
//#else
//const FONTSEARCHPATH1 "."
const FONTHANDLE: i8 = -1;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn i_startup_ttf(fontpointsize: u32, argv: Vec<String>) {
	let fontpath: &str;
	let fontstatus: i32 = -1;
	let bitsperpixel: i32 = 8;


	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
        .build()
        .unwrap();
    let canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
}