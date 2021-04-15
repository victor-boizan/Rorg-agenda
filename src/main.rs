/* Standard */
use std::fmt;

use std::fs;
use std::fs::File;

use std::io;
use std::io::prelude::*;

use std::str::FromStr;

/* External crates */
use regex::Regex;

use chrono::{DateTime, Utc, TimeZone, Duration};

/* Types declaration and implementation */
enum TimeRange{
	Year,
	Month,
	Week
}

#[derive(Debug)]
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
        match input {
            "TODO"  =>Ok(EventState::TODO),

            "WIP"  => Ok(EventState::WIP),

            "FAILED" => Ok(EventState::FAILED),
            "REPORT"  => Ok(EventState::REPORT),

            "DONE" => Ok(EventState::DONE),

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
	let mut current_argument = 0;

	while args.len() >= 1 {
		/* Code for testing purpose */
		println!("{}. ----------------------",current_argument);
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
									let event = Event::new(EventStyle::Task,name);
									println!("{}",event);
								},
								"2" | "h" | "habit" => {
									let event = Event::new(EventStyle::Habit,name);
									println!("{}",event);
								},
								"3" | "a" | "appointment" | "appt" => {
									let event = Event::new(EventStyle::Appt,name);
									println!("{}",event);
								},
								"4" | "b" | "basic" | "" => {
									let event = Event::new(EventStyle::Basic,name);
									println!("{}",event);
								},
								_ => println!("{} is no a valid input.",style)
							}
						},
						Err(e) => println!("error: {}",e)
					}

				} else {
					if args.len() < 3{
						println!("ERROR:\n You need to provide 3 parameter or none for --add.\n\
									Exemples:\n rorg --add\n rorg --add taskname task ./rorg/current/week/w00.org");
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
			"--remove" => {
				if current_argument == last_argument{
					println!("can't remove anything for now");
				} else {
					current_argument += 1;
					let next_argument = args.nth(0).unwrap();
					match next_argument.as_str(){
						"TODO" => println!("it look like you want to remove a TODO entry"),
						"HABIT" => println!("it look like you want to remove an habit entry"),
						"APPT" => println!("it look like you want to remobe a appointments entry"),
						"BASE" => println!("it look like you want to remove a basic event entry"),
						&_ => println!("{} is not a valid value.\nValide values are TODO, HABIT, APPT, BASE",next_argument)
					}

				}
			}
			&_ => {println!("no clue of what to do with \"{}\"",argument)}
		}
		current_argument += 1;
	}

	Ok(())

}

/* Functions */
fn dir_init() -> std::io::Result<u8> {

    /*Create the folders*/
    fs::create_dir_all("./rorg/current/months")?;
    fs::create_dir_all("./rorg/current/weeks")?;
    fs::create_dir_all("./rorg/next-years")?;
    fs::create_dir_all("./rorg/.achives")?;

    /*Create the files*/
    let current_folder = "./rorg/current/";

	/*get the year,month and week and store them in u32 and i32 for the year*/
    let now: DateTime<Utc> = Utc::now();

    let year = format!("{}",now.format("%Y")).parse::<i32>().unwrap();
    let month = format!("{}",now.format("%m")).parse::<u32>().unwrap();
    let week = format!("{}",now.format("%W")).parse::<u32>().unwrap();

	/*current year*/
    let filename = format!("{}/{}.org",current_folder, now.format("%Y"));
    let mut file = File::create(filename)?;
    let content = file_generator(TimeRange::Year,year,0);
    file.write_all(content.as_bytes())?;

	/*weeks*/
    for file_week in week..53{
        let filename: String;
        if file_week < 10{
            filename = format!("{}weeks/w0{}.org",current_folder,file_week);
        }else{
            filename = format!("{}weeks/w{}.org",current_folder,file_week);
        }
        let mut file = File::create(filename)?;
        let content = file_generator(TimeRange::Week,year,file_week);
        file.write_all(content.as_bytes())?;
    }

	/*month*/
    for file_month in month..13{

        let work_month = Utc.ymd(year, file_month, 1);
        let filename = format!("{}months/{}.org",current_folder,work_month.format("%m-%B"));
        let mut file = File::create(filename)?;
        let content = file_generator(TimeRange::Month,year,file_month);
        file.write_all(content.as_bytes())?;
    }

	/*other files*/

    let mut file = File::create("./rorg/habits.org")?;
    let content = "#+TITLE: Habits";
    file.write_all(content.as_bytes())?;

    let mut file = File::create("./rorg/appointments.org")?;
    let content = "#+TITLE: Appointments";
    file.write_all(content.as_bytes())?;

    let mut file = File::create("./rorg/special_times.org")?;
    let content = "#+TITLE: Special times\n# I use this file for evey recurent events like birthdays";
    file.write_all(content.as_bytes())?;

	println!("directories initialised");

    Ok(0)
}
fn file_generator(time: TimeRange,year: i32,date: u32) -> String {

    let file_title: String;
    let begin_content: String;

    match time {
        TimeRange::Year => {
            file_title = format!("#+TITLE: ToDo in {}\n",year);
            begin_content = String::from("* Forcast\n");
        },
        TimeRange::Month => {

            let mut month_day = Utc.ymd(year,date,1);

            file_title = format!("#+TITLE: ToDo {}-{}\n",month_day.format("%B"),year);
			begin_content = format!("* Forcast\n");
        },
        TimeRange::Week =>{
            file_title = format!("#+TITLE: ToDo {}-W{}\n",year,date);
            begin_content = String::from("");
        }
    }
    let generic_content = "* ToDos\n\n\
                            ** In Progress\n\n\
                            ** To be done\n\n\
                            ** Done\n\n\n\
                            * Notes\n\n\
                            * Records\n";

    format!("{}{}{}",file_title,begin_content,generic_content)

}
fn event_add(event: Event, path: String){
	/*open and read the file*/
	let mut file = File::open(&path).expect("OPEN error in function event_add");
	let mut file_content = String::new();
	file.read_to_string(&mut file_content).expect("READ error in function event_add");
	drop(file);
	/*concatenate file content and the event*/
	file_content = format!("{}\n{:#}",file_content,event);
	/*rewrite the file*/
	let mut new_file = File::create(&path).unwrap();
	new_file.write_all(file_content.as_bytes());
}
