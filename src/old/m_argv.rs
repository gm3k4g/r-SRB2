static mut MYARGC: i32 = 0;

// TODO: remove unsafes!


// return bool or i32?
pub fn m_check_parm(check: &str, argv: &Vec<String>, found: usize) -> i32 {
	let mut i: usize = 1; //starting from the first arg of the array length
	while i < argv.len() {
		if check == argv[i] {
			return i as i32;
		}
		i = i + 1;
	}
	0 as i32
}

pub fn m_is_next_parm(argv: &Vec<String>, found: usize) -> i32 {
	if found > 0 && found + 1 < argv.len() 
		&& argv.get(found+1).unwrap().chars().next().unwrap() != '-' 
		&& argv.get(found+1).unwrap().chars().next().unwrap() != '+' {
		return found as i32;
	}
	0 as i32 
}

pub fn m_get_next_parm(argv: &Vec<String>, mut found: usize) -> std::string::String {
	if m_is_next_parm(&argv, found) != 0 {
		found = found + 1;
		return argv.get(found).unwrap().to_string();
	}

	std::string::String::from("")
}
