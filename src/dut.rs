use fst_native::*;

pub struct Dut {
	file: String,
	filter: FstFilter,
}

impl Dut {
	pub fn new(file: String) -> Self {
		Self {
			file,
			filter: FstFilter::all(),
		}
	}
}
