
use regex::Regex;
use std::str::FromStr;
use std::fmt;
use chrono::{Date, Datelike, TimeZone, Utc, Duration};
use chrono::naive::{NaiveDate,NaiveTime};
use std::fs::File;
use std::io::prelude::*;

/* Definition */
#[derive(Debug)]
pub enum EventState {
	TODO,
	WIP,
	FAILED,
	REPORT,
	DONE
}
#[derive(Debug)]
pub enum EventStyle{
	Appt,
	Basic,
	Habit,
	Task,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum FileType{

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
struct TimeStamp{
	date: NaiveDate,
	time: Option<NaiveTime>,
	duration: Option<Duration>,
	delay: Option<Duration>,
	frequency: Option<Duration>
}
#[derive(Debug)]
pub struct Event{
	name: String,
	state: Option<EventState>,
	schedule: Option<TimeStamp>,
	deadline: Option<TimeStamp>,
	style: EventStyle,
	priority: Option<u8>,
	description: Option<String>,
	logs: Option<String>,
	notes: Option<String>
}
pub struct RorgFile{
	pub file_type: FileType,
	pub date:      Option<Date<Utc>>,
	pub title:     String,
	pub forcast:   Option<String>,
	pub events:    Vec<Event>,
	pub notes:     Option<String>,
	pub records:   Option<String>
}

/* Implementation */

impl EventStyle{
	fn defaultstate(&self) -> Option<EventState> {
		match self {
			EventStyle::Task => Some(EventState::TODO),
			EventStyle::Habit => Some(EventState::TODO),
			EventStyle::Appt => Some(EventState::TODO),
			EventStyle::Basic => None,
		}
	}
}
impl Event {
	pub fn new(style: EventStyle,name: String) -> Event {
		Event{
			name,
			state: style.defaultstate(),
			schedule: None,
			deadline: None,
			style,
			priority: None,
			description: None,
			logs: None,
			notes: None
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
impl RorgFile{
	pub fn from_file(path: &str) -> RorgFile {
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
	fn path_generator(&self) -> Result<String,()> {
		let file_type = &self.file_type;
		let date = &self.date;
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
}

impl fmt::Display for TimeStamp {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let date = self.date.format("%Y-%m-%d %a");
		let time: String;
		match self.time{
			Some(_) => time = self.time.unwrap().format(" %R").to_string(),
			None        => time = String::new()
		}
		let duration: String;
		match self.duration{
			Some(_) => {
				let x = self.time.unwrap() + self.duration.unwrap();
				duration = x.format("-%R").to_string();
		},
			None        => duration = String::new()
		}
		let delay: String;
		match self.delay{
			Some(_) => delay = format!(" -{}d",self.delay.unwrap().num_days().to_string()),
			None        => delay = String::new()
		}
		let frequency: String;
		match self.frequency{
			Some(_) => frequency = format!(" +{}d",self.frequency.unwrap().num_days().to_string()),
			None        => frequency = String::new(),
		}
		write!(formatter,"<{}{}{}{}{}>",date,time,duration,delay,frequency)
	}
}
impl fmt::Display for Event {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let state:String ;
		let schedule: String;
		let deadline: String;
		let priority: String;
		let description: String;
		let logs: String;
		let notes: String;
		match &self.state{
		Some(value) => state = format!{"{:?} ",value},
			None    => state = String::new(),
		}
		match &self.schedule{
			Some(value) => schedule = format!("SCHEDULE: {:#}\n",value),
			None    => schedule = String::new(),
		}
		match &self.deadline{
			Some(value) => deadline = format!("DEADLINE: {:#}\n",value),
			None    => deadline = String::new(),
		}
		match &self.priority{
			Some(value) => priority = format!("[#{:?}] ",value),
			None    => priority = String::new(),
		}
		match &self.description{
			Some(value) => description = format!(":DESCRIPTION:\n{}\n:END:\n",value),
			None    => description = String::new(),
		}
		match &self.logs{
			Some(value) => logs = format!(":LOGBOOK:\n{}\n:END:\n",value),
			None    => logs = String::new(),
		}
		match &self.notes{
			Some(value) => notes = format!(":NOTES:\n{}\n:END:\n",value),
			None    => notes = String::new(),
		}
		if formatter.alternate() {
			write!(formatter,"*** {}{}{}\n{}{}:PROPERTIES:\nSTYLE: {:?}\n:END:\n{}{}{}",
				state,priority,self.name,schedule,deadline,self.style,description,notes,logs)
		} else {
			Ok(())
		}
	}
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
			_ => Err(()),
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
impl FromStr for Event {
	type Err = ();
		fn from_str(input: &str) -> Result<Event, ()> {
			let event_regex = Regex::new(r"(?m)^\*{3} (?P<state>[A-Z]{3,4})? ?\[?#?(?P<priority>\d*)?]? ?(?P<name>.*)\n:PROPERTIES:\n:STYLE: (?P<style>[A-z]+)\n(?::[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>^[^:]*\n*):END:\n:NOTES:\n(?P<notes>^[^:]*\n*):END:").unwrap();
			let caps = event_regex.captures(input).unwrap();
			let state: Option<EventState>;

			match caps.name("state"){
				Some(_) => {state = Some(EventState::from_str(&caps["state"]).unwrap())},
				_ => {state = None}
			}

			Ok(Event{
				name: caps["name"].to_string(),
				schedule: None,
				deadline: None,
				state,
				style: EventStyle::from_str(&caps["style"]).unwrap(),
				priority: Some(caps["priority"].parse::<u8>().unwrap_or(0)),
				description: Some(caps["description"].to_string()),
				logs: None ,
				notes: Some(caps["notes"].to_string())
			})
		}
	}

/* Other functions */
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
