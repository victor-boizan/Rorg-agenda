/* external crates */
use chrono::{Date, Utc, Duration};
use chrono::naive::{NaiveDate,NaiveTime};

use std::str::FromStr;
use std::io;
use std::io::prelude::*;
use std::fs;

/*import other files*/
mod rorg_types;

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
					file.to_file(path.as_str());

				}
			}
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
/*Tests*/
//#[cfg(test)]
/*mod test {
	use super::*;

	#[test]
	fn path_test(){
		let date = Utc.ymd(2021,04,15);

		assert_eq!(Ok("./rorg/current/2021.org".to_string()),path_generator(FileType::Year, Some(date)));
		assert_eq!(Ok("./rorg/current/months/04-April.org".to_string()),path_generator(FileType::Month, Some(date)));
		assert_eq!(Ok("./rorg/current/weeks/w15.org".to_string()),path_generator(FileType::Week, Some(date)));
		assert_eq!(Err(()),path_generator(FileType::Year, None));
		assert_eq!(Ok("./rorg/events.org".to_string()),path_generator(FileType::Basic, Some(date)));
		assert_eq!(Ok("./rorg/events.org".to_string()),path_generator(FileType::Basic, None));
		assert_eq!(Ok("./rorg/habits.org".to_string()),path_generator(FileType::Habit, None));
		assert_eq!(Ok("./rorg/appointments.org".to_string()),path_generator(FileType::Appt, None));
	}
	#[test]
	fn time_stamp_test(){
		let stamp = TimeStamp{
			date: NaiveDate::from_ymd(2003,09,16),
			time: Some(NaiveTime::from_hms(9,39,0)),
			duration: Some(Duration::minutes(171)),
			delay: Some(Duration::days(2)),
			frequency: Some(Duration::days(5))
		};
		assert_eq!("<2003-09-16 Tue 09:39-12:30 -2d +5d>".to_string(),format!("{:#}",stamp))
	}
	#[test]
	fn event_test(){

		let stamp = TimeStamp{
			date: NaiveDate::from_ymd(2003,09,16),
			time: Some(NaiveTime::from_hms(9,39,0)),
			duration: Some(Duration::minutes(171)),
			delay: Some(Duration::days(2)),
			frequency: Some(Duration::days(5))
		};
		let mut event = Event::new(EventStyle::Task, "test".to_string());
		assert_eq!("*** TODO test\n:PROPERTIES:\nSTYLE: Task\n:END:\n",format!("{:#}",event));
		event.priority = Some(4);
		assert_eq!("*** TODO [#4] test\n:PROPERTIES:\nSTYLE: Task\n:END:\n",format!("{:#}",event));
		event.schedule = Some(stamp);
		assert_eq!("*** TODO [#4] test\nSCHEDULE: <2003-09-16 Tue 09:39-12:30 -2d +5d>\n:PROPERTIES:\nSTYLE: Task\n:END:\n",format!("{:#}",event));
		event.description = Some("a cool description for testing".to_string());
		assert_eq!("*** TODO [#4] test\nSCHEDULE: <2003-09-16 Tue 09:39-12:30 -2d +5d>\n:PROPERTIES:\nSTYLE: Task\n:END:\n:DESCRIPTION:\na cool description for testing\n:END:\n",format!("{:#}",event));
	}
}*/
