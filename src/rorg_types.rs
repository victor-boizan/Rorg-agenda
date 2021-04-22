
use regex::Regex;
use std::str::FromStr;
use std::fmt;
use chrono::{Date, Datelike, TimeZone, Utc, Duration};
use chrono::naive::{NaiveDate,NaiveTime};
use std::fs::File;
use std::io::prelude::*;

/*Setting some constant*/
const TIMESTAMP_REGEX: &str = r"<(?P<date>\d{4}-\d{2}-\d{2}) .{3}(?: (?P<time>\d{2}:\d{2})(?:-(?P<duration>\d{2}:\d{2}))?)?(?: -(?P<delay>\d*)d)?(?: \+(?P<frequency>\d*)d)?>";
const EVENT_REGEX: &str = r"(?m)^\*{3} (?P<state>[A-Z]{3,4})? ?\[?#?(?P<priority>\d*)?]? ?(?P<name>.*)\n(?:SCHEDULE: (?P<schedule><.*>)\n)?(?:DEADLINE: (?P<deadline><.*>)\n)?:PROPERTIES:\n:STYLE: (?P<style>[A-z]+)\n(?::[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>^[^:]*\n*):END:\n:NOTES:\n(?P<notes>^[^:]*\n*):END:";
const FILE_REGEX: &str = r"(?m)#\+TITLE: (?P<title>.*)\n(?:\* Forcast\n(?P<forcast>(?:^[^*].*\n)*))?(?:\* Notes\n(?P<notes>(?:^[^*].*\n)*))?(?:\* Records\n(?P<records>(?:^[^*].*\n)*))?";
const PATH_REGEX: &str = r"\./rorg/(?P<lvl1>[^/\n]*)/?(?P<lvl2>[^/\n]*)?/?(?P<lvl3>[^/\n]+?)?(?P<nb>\d{2})?(?:[^\d\n]*\.org)$";

/* Definition */
#[derive(Debug,Clone)]
pub enum EventState {
	TODO,
	WIP,
	FAILED,
	REPORT,
	DONE
}
#[derive(Debug,Clone)]
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
#[derive(Debug,Clone)]
pub struct TimeStamp{
	date: NaiveDate,
	time: Option<NaiveTime>,
	duration: Option<Duration>,
	delay: Option<Duration>,
	frequency: Option<Duration>
}
#[derive(Debug, Clone)]
pub struct Event{
	pub name: String,
	pub state: Option<EventState>,
	pub schedule: Option<TimeStamp>,
	pub deadline: Option<TimeStamp>,
	pub style: EventStyle,
	pub priority: Option<u8>,
	pub description: Option<String>,
	pub logs: Option<String>,
	pub notes: Option<String>
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

		let file_regex = Regex::new(FILE_REGEX).unwrap();
		let file_capture = file_regex.captures(file_content.as_str()).unwrap();

		let event_regex = Regex::new(EVENT_REGEX).unwrap();
		let mut event_vector = Vec::new();
		for event in event_regex.captures_iter(&file_content) {
			event_vector.push(Event::from_str(event.get(0).unwrap().as_str()).unwrap())
		}

		let forcast: Option<String>;
		let notes: Option<String>;
		let records: Option<String>;
		match file_capture.name("forcast") {
			Some(_) => forcast = Some(file_capture["forcast"].to_string()),
			None    => forcast = None
		}
		match file_capture.name("notes") {
			Some(_) => notes = Some(file_capture["notes"].to_string()),
			None    => notes = None
		}
		match file_capture.name("records") {
			Some(_) => records = Some(file_capture["records"].to_string()),
			None    => records = None
		}

		RorgFile{
			file_type: FileType::from_str(path).unwrap(),
			date: date_from_path(path),
			title: file_capture["title"].to_string(),
			forcast,
			events: event_vector,
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
				let file_content = format!("#+TITLE: {}\n* Forcast\n{}* Notes\n{}\n* Records\n{}\n* Todo\n{}\n",
					self.title, forcast, events, notes, records);
				let mut file = File::create(path)?;
				file.write_all(file_content.as_bytes())?;
				Ok(0)
			},
			FileType::Week => {
				let mut events = String::new();
				for entry in self.events{
					events = format!("{}\n{:#}",events,entry)
				}
				let file_content = format!("#+TITLE: {}\n* Notes\n{}\n* Records\n{}\n* Todo\n{}",
					self.title, events, self.notes.unwrap(), self.records.unwrap());
				let mut file = File::create(path)?;
				file.write_all(file_content.as_bytes())?;
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
				file.write_all(file_content.as_bytes())?;
				Ok(0)
			}
		}
	}
	pub fn add_event(&mut self, event: Event) {
		self.events.push(event);
	}
	pub fn path_generator(&self) -> Result<String,()> {
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

impl FromStr for FileType {
	type Err = ();
	fn from_str(input: &str) -> Result<FileType, ()> {
		match input.to_lowercase().as_str() {
			"year"  => Ok(FileType::Year),
			"month" => Ok(FileType::Month),
			"week"  => Ok(FileType::Week),
			"basic" => Ok(FileType::Basic),
			"habit" => Ok(FileType::Habit),
			"appt"  => Ok(FileType::Appt),
			_ => {
				let path_regex = Regex::new(PATH_REGEX).unwrap();
				let path_capture = path_regex.captures(input).unwrap();
				if path_capture["lvl1"].to_string() == "current".to_string(){
					match path_capture.name("lvl3"){
						Some(_) => {
							if &path_capture["lvl3"] == "w" { Ok(FileType::Week) }
							else { Ok(FileType::Month) }
						}
						None  => Ok(FileType::Year),
					}
				} else {
					match &path_capture["lvl1"]{
						"appointments" => Ok(FileType::Appt),
						"habits"       => Ok(FileType::Habit),
						"special_time" => Ok(FileType::Basic),
						_ => Err(())
					}
				}
			}
		}
	}
}
impl FromStr for TimeStamp {
	type Err = ();
	fn from_str(input: &str) -> Result<TimeStamp, ()> {
		let regex = Regex::new(TIMESTAMP_REGEX).unwrap();
		let caps = regex.captures(input).unwrap();

		let time: Option<NaiveTime>;
		let duration: Option<Duration>;
		let delay: Option<Duration>;
		let frequency: Option<Duration>;

		match caps.name("time") {
			Some(_) => {time = Some(NaiveTime::from_str(format!("{}:00",&caps["time"]).as_str()).unwrap())},
			_       => {time = None}
		}
		match caps.name("duration") {
			Some(_) => {
				let duration_timepoint = NaiveTime::from_str(format!("{}:00",&caps["duration"]).as_str()).unwrap();
				duration = Some(duration_timepoint - time.unwrap())
			}
			None    => {duration = None}
		}
		match caps.name("delay") {
			Some(_) => {delay = Some(Duration::days(caps["delay"].parse().unwrap()))}
			None    => {delay = None}
		}
		match caps.name("frequency") {
			Some(_) => {frequency = Some(Duration::days(caps["frequency"].parse().unwrap()))}
			None    => {frequency = None}
		}

		Ok(TimeStamp{
			date: NaiveDate::from_str(&caps["date"]).unwrap(),
			time,
			duration,
			delay,
			frequency
		})
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
			let event_regex = Regex::new(EVENT_REGEX).unwrap();
			let caps = event_regex.captures(input).unwrap();
			let state: Option<EventState>;
			let schedule: Option<TimeStamp>;
			let deadline: Option<TimeStamp>;
			match caps.name("state"){
				Some(_) => {state = Some(EventState::from_str(&caps["state"]).unwrap())},
				None    => {state = None}
			}
			match caps.name("schedule"){
				Some(_) => {schedule = Some(TimeStamp::from_str(&caps["schedule"]).unwrap())},
				None    => {schedule = None}
			}
			match caps.name("deadline"){
				Some(_) => {deadline = Some(TimeStamp::from_str(&caps["deadline"]).unwrap())},
				None    => {deadline = None}
			}
			Ok(Event{
				name: caps["name"].to_string(),
				schedule,
				deadline,
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
	let path_regex = Regex::new(PATH_REGEX).unwrap();
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
