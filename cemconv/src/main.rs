extern crate cem;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

#[derive(StructOpt, Debug)]
struct Opt {
	#[structopt(short = "i", long = "input", help = "Input file to convert")]
	input: String,
	#[structopt(help = "Output file, default is stdout")]
	output: Option<String>
}

enum Format {
	Cem,
	Obj
}

fn main() {
	use structopt::StructOpt;

	let opt = Opt::from_args();

	println!("I: {}, O: {:?}", opt.input, opt.output);
}
