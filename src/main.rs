use clap::{Arg, SubCommand};
use pitr_util::pitrmap::PitrMap;
use pitr_util::errors::{FromFileError, FromStringError};

fn get_map(filename: &str) -> Result<PitrMap, String> {
	let map = PitrMap::from_file(filename);

	match map {
		Ok(parsed_map) => Ok(parsed_map),
		Err(error) => match error {
			FromFileError::FileError(reason) => Err(reason.to_string()),
			FromFileError::FromStringError(kind) => match kind {
				FromStringError::UnsupportedSaveVersion(v) => Err(format!("unsupported save version ({})", v)),
				FromStringError::UnsupportedGameVersion(v) => Err(format!("unsupported game version ({})", v)),
				FromStringError::ParseError(_) => Err(String::from("invalid map file"))
			}
		}
	}
}

fn main() {
	let matches = clap::App::new("pitr_tool")
		.version(clap::crate_version!())
		.subcommand(SubCommand::with_name("info")
			.arg(Arg::with_name("files")
				.required(true)
				.multiple(true)))
		.subcommand(SubCommand::with_name("merge")
			.arg(Arg::with_name("files")
				.required(true)
				.multiple(true))
			.arg(Arg::with_name("output")
				.short("o")
				.long("output")
				.takes_value(true)))
		.get_matches();

	let subcommand = matches.subcommand();

	match subcommand {
		("info", Some(sub_matches)) => {
			let file_names = sub_matches.values_of("files").unwrap();

			for filename in file_names {
				match get_map(filename) {
					Ok(parsed_map) => {
						println!("---info for {}---", filename);

						println!("Blocks: {}", parsed_map.Blocks.len());
						println!("Props: {}", parsed_map.Props.len());
						println!("Enemies: {}", parsed_map.Enemies.len());
					},
					Err(help_text) => {
						eprintln!("---error in {}---\n{}", filename, help_text);
					}
				}
			}
		},
		("merge", Some(sub_matches)) => {
			let input_files = sub_matches.values_of("files").unwrap();

			let mut new_map = PitrMap::new();

			for filename in input_files {
				let map = get_map(filename);

				match map {
					Ok(mut parsed_map) => {
						new_map.Blocks.append(&mut parsed_map.Blocks);
						new_map.Props.append(&mut parsed_map.Props);
						new_map.Enemies.append(&mut parsed_map.Enemies);
					},
					Err(help_text) => {
						panic!("{}", help_text);
					}
				}
			}

			let serialized_map = new_map.to_string().unwrap();
			let output = sub_matches.value_of("output");

			match output {
				Some(filename) => {
					std::fs::write(filename, serialized_map).expect("cannot write result file");
				}
				None => {
					println!("{}", serialized_map);
				}
			}
		},
		_ => {
			println!("specify a subcommand");
		}
	}
}
