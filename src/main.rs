/* external crates */
use std::str::FromStr;
use std::fs;

/*import other files*/
mod rorg_types;
mod ui;
/* Main */
fn main() -> std::io::Result<()> {

	let mut args = std::env::args();

	while args.len() >= 1 {

		let argument = args.nth(0).unwrap();

		match argument.as_str(){
			"--init" => {println!("{:?}",dir_init())},
			"--read" => {
				let file = rorg_types::RorgFile::from_file(args.nth(0).unwrap().as_str());
				for entry in file.events{println!("{}",entry)}
			}
			"--add" => {
				if args.len() == 0 {
					/* Does not work beacause : Could not evaluate DW_OP_GNU_entry_value.
					   and idk how to fix it

					let event = Event::new_from_cli();
					println!("{}",event);
					let mut file = RorgFile::from_file("./rorg/current/2021.org");
					file.add_event(event);
					file.to_file(".rorg/current/2021.org");
					*/
					println!("Sorry but --add does not work alone for now\nYou need to provide 3 parameter\nExemple:\n rorg --add taskname task ./rorg/current/week/w00.org")

				} else {
					if args.len() < 3{
						println!("ERROR:\n You need to provide 3 parameter for --add.\n\
									Exemple:\n rorg --add taskname task ./rorg/current/week/w00.org");
						std::process::exit(-1);
					}

					let name = args.nth(0).unwrap();
					let style = rorg_types::EventStyle::from_str(args.nth(0).unwrap().as_str());
					let event= rorg_types::Event::new(style.unwrap(), name);
					let path = args.nth(0).unwrap();

					let mut file = rorg_types::RorgFile::from_file(path.as_str());
					file.add_event(event);
					file.to_file(path.as_str())?;

				}
			}
			"--tui" => {
				/*The tui is in wip. I never did that before*/
				ui::init()?;
			},
			&_ => {println!("no clue of what to do with \"{}\"",argument)}
		}
	}

	Ok(())

}

/* Functions */

/*Create the rorg folder and subfolders*/
fn dir_init() -> std::io::Result<u8> {

	fs::create_dir_all("./rorg/current/months")?;
	fs::create_dir_all("./rorg/current/weeks")?;
	fs::create_dir_all("./rorg/next_years")?;
	fs::create_dir_all("./rorg/.achives")?;

	Ok(0)
}
