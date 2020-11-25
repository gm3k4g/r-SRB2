// unable to implement Default?
pub struct SdlMain {
	pub consolevent: sdl2_sys::SDL_bool,
	pub framebuffer: sdl2_sys::SDL_bool
}
impl SdlMain {
	pub fn new() -> Self {
		SdlMain {
			consolevent: sdl2_sys::SDL_bool::SDL_FALSE,
			framebuffer: sdl2_sys::SDL_bool::SDL_FALSE,
		}
	}
}