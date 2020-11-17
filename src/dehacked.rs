#[derive(Default)]
pub struct Dehacked {
	pub free_states: String, //static char *FREE_STATES[NUMSTATEFREESLOTS];
}
impl Dehacked {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn deh_check(&self) {
		cons_printf!("TODO: make deh_check() work!\n");
	}
}