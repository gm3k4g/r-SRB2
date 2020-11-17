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

	pub fn m_find_response_file(&self) {
		cons_printf!("TODO: m_find_response_file()...\n");

		let mut i: usize = 1;
		// TODO: make it a for loop?
		while i < self.myargc as usize {
			if self.myargv[i].chars().next().unwrap() == '@' {
				let mut _handle: std::fs::File;
				let mut _handlefile : String = self.myargv[i].to_string();
				//handlefile = handlefile.replace("@", ""); // Removes '@' from given parameter

				let (_k, _p_index, _index_in_file): (i32, i32, i32);
				let _size: i32;
				let _inquote: bool = false;
				let _infile: u8; //UINT8
				let _file: &str = "";
				let _moreargs: Vec::<&str> = Vec::new();
				let _firstargv: &str = "";

				// read response file into memory
				_handle = match std::fs::OpenOptions::new().write(false)
					.read(true)
					.create_new(false)
					.open(_handlefile.as_str()) {
						Ok(file) => file,
						Err(e) => {
							// WHATIS, WHEREIS: I_Error() ?
							//I_Error("Response file ", self.myargv[i], "not found\n");
							cons_printf!("Response file ", self.myargv[i], " not found: ", e, "\n");
							return;
						}
					};
				// WHATIS, WHEREIS: M_GetText?
				//cons_printf!(M_GetText("Found response file %s\n"), &myargv[i][1]);
				cons_printf!("Found response file ", self.myargv[i],"\n");
			}
			i = i + 1;
		}
	}
}