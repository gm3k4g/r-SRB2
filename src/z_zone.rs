#[derive(Default)]
pub struct MemBlockT {
	//void* real,
	pub hdr: MemHdrT,

	//void** user,
	pub tag: i32, //purgelevel

	pub size: isize, //including the header and blocks
	pub realsize: isize, // size of real data only

	pub ownerfile: String,
	pub ownerline: i32,

	pub next: Box<MemBlockT>,
	pub prev: Box<MemBlockT>,
} // both the head and tail of the zone memory block list

#[derive(Default)]
pub struct MemHdrT {
	pub block: Box<MemBlockT>, // Describing this memory
	pub id: u32, // should be ZONEID
} //ATTRPACK

#[derive(Default)]
pub struct Zone {
	pub z_calloc: bool,
}
impl Zone {
	pub fn new() -> Self {
		Zone {
			z_calloc: false,
		}
	}

	pub fn z_init(&self) {
		cons_printf!("TODO: make z_init() work! \n");
	}
}
