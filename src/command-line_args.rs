use std::os;

fn main() {
	let args = os::args();
	println("All args:");
	for arg in args.iter() {
		println(*arg);
	}
}
