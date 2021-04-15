/* Standard */
use std::fmt;

use std::fs;
use std::fs::File;

use std::io;
use std::io::prelude::*;

use std::str::FromStr;

/* External crates */
use regex::Regex;

use chrono::{DateTime, Date, Datelike, Utc, TimeZone, Duration};

/* Types declaration and implementation */


enum TimeRange{
	Year,
	Month,
	Week
}

#[derive(Debug)]
#[derive(PartialEq)]
enum FileType{
	/*Task files*/
	Year,
	Month,
	Week,

	/*other files*/
	Basic,
	Habit,
	Appt
}
#[derive(Debug)]
struct RorgFile{
	file_type: FileType,
	date:      Option<Date<Utc>>,
	title:     String,
	forcast:   Option<String>,
	events:    Vec<Event>,
	notes:     Option<String>,
	records:   Option<String>
}

impl RorgFile{
	fn from_file(path: &str) -> RorgFile {

		/*Open the file and get it's content*/
		let mut file = File::open(path).expect("OPEN error in function from_file");
		let mut file_content = String::new();
		file.read_to_string(&mut file_content).expect("READ error in function from_file");
		drop(file);

		let file_regex = Regex::new(r"(?m)#\+TITLE: (?P<Title>.*)\n*(?:\* Forcast\n)?(?P<Forcast>^[^\*]*\n*)?\*(?:.*\n)+?\* Notes\n(?P<Notes>^[^\*]*\n*)\n\* Records\n(?P<Records>^[^\*]*\n*)").unwrap();
		let file_capture = file_regex.captures(file_content.as_str()).unwrap();

		let event_regex = Regex::new(r"(?m)^\*{3} (?P<state>[A-Z]{3,4})? ?\[?#?(?P<priority>\d*)?]? ?(?P<name>.*)\n:PROPERTIES:\n:STYLE: (?P<style>[A-z]+)\n(?::[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>^[^:]*\n*):END:\n:NOTES:\n(?P<notes>^[^:]*\n*):END:").unwrap();
		let mut event_vector = Vec::new();

		for event in event_regex.captures_iter(&file_content) {
			event_vector.push(Event::from_str(event.get(0).unwrap().as_str()).unwrap())
		}

		let forcast: Option<String>;
		let notes: Option<String>;
		let records: Option<String>;
		if file_capture["Forcast"].is_empty(){
			forcast = None;
		} else {
			forcast = Some(file_capture["Forcast"].to_string());
		}
		if file_capture["Records"].is_empty(){
			records = None;
		} else {
			records = Some(file_capture["Records"].to_string());
		}
		if file_capture["Notes"].is_empty(){
			notes = None;
		} else {
			notes = Some(file_capture["Notes"].to_string());
		}

		RorgFile{
			file_type:FileType::Year,
			date: date_from_path(path),
			title: file_capture["Title"].to_string(),
			forcast,
			events:event_vector,
			notes,
			records,
		}

	}
	pub fn to_file(self, path: &str) -> std::io::Result<u8>{
		match self.file_type {
			FileType::Year | FileType::Month => {
				let mut events = String::new();
				for entry in self.events{
					events = format!("{}\n\n{:#}",events,entry)
				}

				let forcast: String;
				let notes: String;
				let records: String;
				match self.forcast{
					Some(value) => forcast = value,
					None        => forcast = "".to_string()
				}
				match self.notes{
					Some(value) => notes = value,
					None        => notes = "".to_string()
				}
				match self.records{
					Some(value) => records = value,
					None        => records = "".to_string()
				}

				let file_content = format!("#+TITLE: {}\n* Forcast\n{}* Todo\n{}\n\n* Notes\n{}\n* Records\n{}",
					self.title, forcast, events, notes, records);

				let mut file = File::create(path)?;
				file.write_all(file_content.as_bytes());
				Ok(0)

			},
			FileType::Week => {
				let mut events = String::new();
				for entry in self.events{
					events = format!("{}\n{:#}",events,entry)
				}
				let file_content = format!("#+TITLE: {}\n* Todo\n{}\n* Notes\n{}\n* Records\n{}",
					self.title, events, self.notes.unwrap(), self.records.unwrap());

				let mut file = File::create(path)?;
				file.write_all(file_content.as_bytes());
				Ok(0)

			},
			FileType::Basic | FileType::Habit | FileType::Appt => {

				let mut events = String::new();
				for entry in self.events{
					events = format!("{}\n{:#}",events,entry)
				}
				let file_content = format!("#+TITLE: {}\n{}",
					self.title, events);

				let mut file = File::create(path)?;
				file.write_all(file_content.as_bytes());
				Ok(0)

			}
		}
	}
	pub fn add_event(&mut self, event: Event) {
		self.events.push(event);
	}
}

#[derive(Debug)]
enum EventState{
	TODO,

	WIP,

	FAILED,
	REPORT,

	DONE,

	Null,
}
impl FromStr for EventState {
	type Err = ();
	fn from_str(input: &str) -> Result<EventState, ()> {
		match input.to_lowercase().as_str() {
			"todo"  =>Ok(EventState::TODO),
			"wip"  => Ok(EventState::WIP),
			"failed" => Ok(EventState::FAILED),
			"report"  => Ok(EventState::REPORT),
			"done" => Ok(EventState::DONE),
			_ => Ok(EventState::Null),
		}
	}
}

#[derive(Debug)]
enum EventStyle{
	Appt,
	Basic,
	Habit,
	Task,
}
impl EventStyle{
	fn defaultstate(&self) -> EventState {
		match self {
			EventStyle::Task => EventState::TODO,
			EventStyle::Habit => EventState::TODO,
			EventStyle::Appt => EventState::TODO,
			EventStyle::Basic => EventState::Null,
		}
	}
}
impl FromStr for EventStyle {
	type Err = ();
	fn from_str(input: &str) -> Result<EventStyle, ()> {
		match input.to_lowercase().as_str() {
			"task"  =>Ok(EventStyle::Task),
			"habit"  => Ok(EventStyle::Habit),
			"appt" => Ok(EventStyle::Appt),
			"basic"  => Ok(EventStyle::Basic),
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
struct Event{
	name: String,
	state: EventState,
	style: EventStyle,
	priority: u8,
	description: String,
	logs: String,
	notes: String
}
impl Event {
	fn new(style: EventStyle,name: String) -> Event {
		Event{
			name,
			state: style.defaultstate(),
			style,
			priority: 0,
			description: String::new(),
			logs: String::new(),
			notes: String::new()
		}
	}
	/* when use with RorgFile::add_event, Could not evaluate DW_OP_GNU_entry_value

		fn new_from_cli() -> Event {
		println!("What is the name of the entry?");
		let mut name = String::new();
		match io::stdin().read_line(&mut name){
			Ok(_) => {
				name.pop();
				println!("You will create an event named {}",name);
			},
			Err(e) => println!("error: {}",e)
		}

		let mut style = String::new();
		println!("What style of entry \"{}\" is?\n1.Task\n2.Habit\n3.appointment\n4.Basic event\n",name);
		match io::stdin().read_line(&mut style){
			Ok(_) => {
				style.pop();
				style = style.to_lowercase();
				match style.as_str(){
				/*  nb | letter | name | alt*/
					"1" | "t" | "task" | "todo"=> {
						return Event::new(EventStyle::Task,name)
					},
					"2" | "h" | "habit" => {
						return Event::new(EventStyle::Habit,name)
					},
					"3" | "a" | "appointment" | "appt" => {
						return Event::new(EventStyle::Appt,name)
					},
					"4" | "b" | "basic" | "" => {
						Event::new(EventStyle::Basic,name)
					},
					_ => {
						println!("{} is no a valid input.",style);
						std::process::exit(-1);

					}
				}
			},
			Err(e) => {
				println!("error: {}",e);
				std::process::exit(-1);
			}
		}
	}*/
}
impl fmt::Display for Event {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		if formatter.alternate() {
			match self.style {
				EventStyle::Appt  => {
					write!(formatter, "*** {:?} {}\
						\n:PROPERTIES:\n:STYLE: Appt\n:END:\
						\n:DESCRIPTION:\n{}\n:END:\
						\n:NOTES:\n{}\n:END:",
						self.state, self.name, self.description, self.notes
					)
				},
				EventStyle::Basic => {
					write!(formatter, "*** {}\
						\n:PROPERTIES:\n:STYLE: Basic\n:END:\
						\n:DESCRIPTION:\n{}\n:END:\
						\n:NOTES:\n{}\n:END:",
						self.name, self.description, self.notes
					)
				}
				EventStyle::Habit => {
					write!(formatter, "*** {:?} {}\
						\n:PROPERTIES:\n:STYLE: Habit\n:END:\
						\n:DESCRIPTION:\n{}\n:END:\
						\n:NOTES:\n{}\n:END:",
						self.state, self.name, self.description, self.notes
					)
				},
				EventStyle::Task  => {
					write!(formatter, "*** {:?} [#{:?}] {}\
						\n:PROPERTIES:\n:STYLE: Task\n:END:\
						\n:DESCRIPTION:\n{}\n:END:\
						\n:NOTES:\n{}\n:END:",
						self.state, self.priority, self.name, self.description, self.notes
					)
				},
			}
		} else {
			match self.style {
				EventStyle::Appt  =>{
					write!(formatter, "{:?} {}\nDescription:\n{}Notes:\n{}",
						self.state, self.name, self.description, self.notes)
				},
				EventStyle::Basic =>{
					write!(formatter, "{}\nDescription:\n{}Notes:\n{}",
						self.name, self.description, self.notes)
				},
				EventStyle::Habit =>{
					write!(formatter, "{:?} {}\nDescription:\n{}Notes:\n{}",
						self.state, self.name, self.description, self.notes)
				},
				EventStyle::Task  =>{
					write!(formatter, "[#{:?}] {:?} {}\nDescription:\n{}Notes:\n{}",
						self.priority, self.state, self.name, self.description, self.notes)
				},
			}
		}
	}
}
impl FromStr for Event {
	type Err = ();

	fn from_str(input: &str) -> Result<Event, ()> {
		let event_regex = Regex::new(r"(?m)^\*{3} (?P<state>[A-Z]{3,4})? ?\[?#?(?P<priority>\d*)?]? ?(?P<name>.*)\n:PROPERTIES:\n:STYLE: (?P<style>[A-z]+)\n(?::[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>^[^:]*\n*):END:\n:NOTES:\n(?P<notes>^[^:]*\n*):END:").unwrap();
		let caps = event_regex.captures(input).unwrap();
		let state: EventState;

		match caps.name("state"){
			Some(_) => {state = EventState::from_str(&caps["state"]).unwrap()},
			_ => {state = EventState::Null}
		}

		Ok(Event{
			name: caps["name"].to_string(),
			state,
			style: EventStyle::from_str(&caps["style"]).unwrap(),
			priority: caps["priority"].parse::<u8>().unwrap_or(0),
			description: caps["description"].to_string(),
			logs: "".to_string() ,
			notes: caps["notes"].to_string()
		})
	}
}

/* Main */
fn main() -> std::io::Result<()> {

	let mut args = std::env::args();
	println!("{:?}\n{} arguments",args,args.len());
	let last_argument = args.len()-1;

	while args.len() >= 1 {
		/* Code for testing purpose */
		println!("{}. ----------------------",last_argument - args.len());
		let argument = args.nth(0).unwrap();
		println!("argument nb{}/{} : {}\nargs.len() = {}\n",last_argument - args.len(),last_argument,argument,args.len());
		/* ^^^ Code for testing purpose*/

		match argument.as_str(){
			"--init" => {println!("{:?}",dir_init())},
			"--read" => {
				let file = RorgFile::from_file(args.nth(0).unwrap().as_str());
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
					let style = EventStyle::from_str(args.nth(0).unwrap().as_str());
					let event=Event::new(style.unwrap(), name);
					let path = args.nth(0).unwrap();

					let mut file = RorgFile::from_file(path.as_str());
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
fn path_generator(file_type: FileType, date: Option<Date<Utc>>) -> Result<String,()> {
	match file_type {
		FileType::Year | FileType::Week | FileType::Month=> {
			match date {
				Some(_) => {},
				None    => return Err(()),
			}
		}
		_ =>{},
	}
	Ok(
		match file_type {
			FileType::Year  => {
				if date.unwrap().year() == Utc::today().year() {
					format!("./rorg/current/{}.org", date.unwrap().iso_week().year())
				} else {
					format!("./rorg/next_years/{}.org", date.unwrap().iso_week().year())
				}
			},
			FileType::Month => {format!("./rorg/current/months/{}.org", date.unwrap().format("%m-%B"))/*%m-> month number, %B -> month name*/},
			FileType::Week  => {format!("./rorg/current/weeks/w{}.org", date.unwrap().iso_week().week())},

			FileType::Basic => String::from("./rorg/events.org"),
			FileType::Habit => String::from("./rorg/habits.org"),
			FileType::Appt  => String::from("./rorg/appointments.org")

		}
	)
}

fn date_from_path(path: &str) -> Option<Date<Utc>>{
	let path_regex = Regex::new(r"\./rorg/(?P<lvl1>[^/\n]*)/?(?P<lvl2>[^/\n]*)?/?(?P<lvl3>[^/\n]+?)?(?P<nb>\d{2})?(?:[^\d\n]*)(?:\.org)$").unwrap();
	let path_capture = path_regex.captures(path).unwrap();

	if path_capture["lvl1"].to_string() == "current".to_string(){
			match path_capture.name("lvl3"){
				Some(_) => {
					if &path_capture["lvl3"] == "w" {
						Some(Utc.isoywd(Utc::today().year(), path_capture["nb"].parse().unwrap(), chrono::Weekday::Mon))
					} else {
						Some(Utc.ymd(Utc::today().year(), path_capture["lvl3"].parse().unwrap(), 1))
					}
				}
				None  => Some(Utc.yo(path_capture["lvl2"].parse().unwrap(), 1)),
			}
	} else {
		None
	}
}

/*Tests*/
#[cfg(test)]
mod test {
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
}
