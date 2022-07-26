use clap::{Parser, Subcommand};
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

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
	#[clap(subcommand)]
	command: Commands
}

// TODO: String -> std::path::PathBuf
#[derive(Subcommand)]
enum Commands {
	Info {
		files: Vec<String>
	},
	Merge {
		files: Vec<String>,

		#[clap(short, long)]
		output: Option<String>
	}
}

fn main() {
	let matches = Cli::parse();

	match &matches.command {
		Commands::Info { files } => {
			for filename in files {
				match get_map(filename) {
					Ok(parsed_map) => {
						println!("---info for {}---", filename);

						println!("Map Name: {}", parsed_map.map_name());
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
		Commands::Merge { files, output } => {
			let mut new_map = PitrMap::new();

			for filename in files {
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

			match output {
				Some(filename) => {
					std::fs::write(filename, serialized_map).expect("cannot write result file");
				}
				None => {
					println!("{}", serialized_map);
				}
			}
		},
	}
}
