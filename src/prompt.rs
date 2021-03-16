// Transpire
// MIT License - 2021
// prompt.rs

// Dependencies of the file - argparse library
extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store, Collect};

/**
 * Structure to represent variables given in arguments.
 */
pub struct ArgFlags  {
	// Source files
	pub src_files: 		Vec<String>, // Source files

	// Language specifications
	pub prog_lang: 		String, 	// Programming language to parse.
	pub src_lang: 		String,		// Source language.
	pub target_lang: 	String,		// Target language.

	// Flags for options
	pub verbose_flag: 	bool,		// Flag for verbosity.
	pub update_flag: 	bool,		// Translations update flag.
}

pub fn process_args() -> ArgFlags {
	// Create structure for arguments
	let mut main_flags = ArgFlags {
		// Set default values for flags
		src_files: 		Vec::new(),
		target_lang:  	"en".to_string(),
		src_lang: 		"".to_string(),
		prog_lang: 		"".to_string(),
		verbose_flag: 	false,
		update_flag: 	false
	};

	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Translate code in your native language
						to a compileable/interpretable program!");
		ap.refer(&mut main_flags.verbose_flag)
		  .add_option(&["-v","--verbose"], StoreTrue,
					  "Enable verbose output.");
		ap.refer(&mut main_flags.update_flag)
		  .add_option(&["-u", "--update"], StoreTrue,
		  "Update the translations repository if already installed.");
		ap.refer(&mut main_flags.src_lang)
		  .add_option(&["-s", "--source"], Store,
					  "Spoken language of given source files. ")
			.required();
		ap.refer(&mut main_flags.target_lang)
		  .add_option(&["-t", "--target"], Store,
		  "Target spoken language to
			translate to - defaults to english.");
		ap.refer(&mut main_flags.prog_lang)
		  .add_option(&["-l", "--lang"], Store,
					  "Target programming language.")
			.required();
		ap.refer(&mut main_flags.src_files)
		  .add_argument("FILES", Collect,
						"Files to parse and translate.")
			.required();
	}
	return main_flags;
}
