pub struct MArgv {
	pub myargc: i32,
	pub myargv: Vec<std::string::String>,
	pub myargmalloc: bool, // Do we need this?
	pub found: i32,
}
impl MArgv {
	pub fn new(argv: Vec<std::string::String>) -> Self {
		MArgv {
			myargc: argv.len() as i32, //First parameter is the executable path
			myargv: argv,
			myargmalloc: false,
			found: 0
		}
	}

	pub fn m_check_parm(&mut self, check: &str) -> i32 {
		let mut i: usize = 0;

		while i < self.myargc as usize {
			//if (!strcasecmp(check, myargv[i]))
			if check == self.myargv[i] {
				self.found = i as i32;
				return i as i32;
			}
			i = i + 1;
		}
		self.found = 0 as i32;
		return 0 as i32;
	}
}