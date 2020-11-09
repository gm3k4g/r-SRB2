pub fn m_check_parm(check: &str, argv: &Vec<String>) -> bool {
	let i: usize = 1;

	while i < argv.len() {
		if check == argv[i] {
			return true;
		}
	}
	false
}