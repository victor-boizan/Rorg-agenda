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
	Task,
	Habit,
	Appt,
	BasicEvent
}
impl EventStyle{
	fn defaultstate(&self) -> EventState {
		match self {
			EventStyle::Task => EventState::TODO,
			EventStyle::Habit => EventState::TODO,
			EventStyle::Appt => EventState::TODO,
			EventStyle::BasicEvent => EventState::Null,
		}
	}
}
impl FromStr for EventStyle {
	type Err = ();
	fn from_str(input: &str) -> Result<EventStyle, ()> {
		match input {
			"Task"  =>Ok(EventStyle::Task),
			"Habit"  => Ok(EventStyle::Habit),
			"Appt" => Ok(EventStyle::Appt),
			"BasicEvent"  => Ok(EventStyle::BasicEvent),
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
	fn event_extractor(path: &str) -> Vec<Event> {

		let mut file = File::open(path).expect("OPEN error in function file_extractor");
		let mut file_content = String::new();
		file.read_to_string(&mut file_content).expect("READ error in function file_extractor");
		drop(file);

		//this regex match all events types
		let event_regex = Regex::new(r"(?m)^\*{5} (?P<state>[A-Z]{3,4})? ?\[?#?(?P<priority>\d*)?]? ?(?P<name>.*)\n:PROPERTIES:\n:STYLE: (?P<style>[A-z]+)\n(?::[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>^[^:]*\n*):END:\n:NOTES:\n(?P<notes>^[^:]*\n*):END:").unwrap();
		let mut event_vector = Vec::new();

		for event in event_regex.captures_iter(&file_content) {
			event_vector.push(Event::from_str(event.get(0).unwrap().as_str()).unwrap())
		}

		return event_vector;
	}
}
impl fmt::Display for Event {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.style {
			EventStyle::Task => {
				write!(f, "***** {:?} [#{:?}] {}\
					\n:PROPERTIES:\n:STYLE: Task\n:END:\
					\n:DESCRIPTION:\n{}\n:END:\
					\n:NOTES:\n{}\n:END:",
					self.state, self.priority, self.name, self.description, self.notes
				)
			},
			EventStyle::Habit  => {
				write!(f, "***** {:?} {}\
					\n:PROPERTIES:\n:STYLE: Habit\n:END:\
					\n:DESCRIPTION:\n{}\n:END:\
					\n:NOTES:\n{}\n:END:",
					self.state, self.name, self.description, self.notes
				)
			},
			EventStyle::Appt => {
				write!(f, "***** {:?} {}\
					\n:PROPERTIES:\n:STYLE: Appt\n:END:\
					\n:DESCRIPTION:\n{}\n:END:\
					\n:NOTES:\n{}\n:END:",
					self.state, self.name, self.description, self.notes
				)
			},
			EventStyle::BasicEvent => {
				write!(f, "***** {}\
					\n:PROPERTIES:\n:STYLE: BasicEvent\n:END:\
					\n:DESCRIPTION:\n{}\n:END:\
					\n:NOTES:\n{}\n:END:",
					self.name, self.description, self.notes
				)
			},
		}
	}
}
impl FromStr for Event {
	type Err = ();

	fn from_str(input: &str) -> Result<Event, ()> {
		let event_regex = Regex::new(r"(?m)^\*{5} (?P<state>[A-Z]{3,4})? ?\[?#?(?P<priority>\d*)?]? ?(?P<name>.*)\n:PROPERTIES:\n:STYLE: (?P<style>[A-z]+)\n(?::[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>^[^:]*\n*):END:\n:NOTES:\n(?P<notes>^[^:]*\n*):END:").unwrap();
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

	while current_argument <= last_argument{
		println!("{}. ----------------------",current_argument);
		let argument = args.nth(0).unwrap();
		println!("argument nb{}/{} : {}\n",current_argument+1,last_argument+1,argument);
		match argument.as_str(){
			"--init" => {
				println!("{:?}",dir_init());
			},
			"--read" => {
				println!{"can't read for now"}
			}
			"--add" => {
				if current_argument == last_argument{

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
								"2" | "H" | "habit" => {
									let event = Event::new(EventStyle::Habit,name);
									println!("{}",event);
								},
								"3" | "a" | "appointment" | "appt" => {
									let event = Event::new(EventStyle::Appt,name);
									println!("{}",event);
								},
								"4" | "b" | "basic event" | "base" => {
									let event = Event::new(EventStyle::BasicEvent,name);
									println!("{}",event);
								},
								_ => println!("{} is no a valid input.",style)
							}
						},
						Err(e) => println!("error: {}",e)
					}

				} else {
					current_argument += 1;
					let next_argument = args.nth(0).unwrap();
					current_argument += 1;
					let name = args.nth(0).unwrap();
					//println!("{:?}",next_argument);
					match next_argument.as_str(){
						"TODO" => {
							let event = Event::new(EventStyle::Task,name);
							println!("{}",event);
						},
						"HABIT" => {
							let event = Event::new(EventStyle::Task,name);
							println!("{}",event);
						},
						"APPT" => {
							let event = Event::new(EventStyle::Task,name);
							println!("{}",event);
						},
						"BASE" => {
							let event = Event::new(EventStyle::Task,name);
							println!("{}",event);
						},
						&_ => println!("{} is not a valid value.\nValide values are TODO, HABIT, APPT, BASE",next_argument)
					}
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

            //generate the calendar
            let header = "|----+----+----+----+----+----+----+----|\n\
                          | WW | Mo | Tu | We | Th | Fr | Sa | Su |\n\
                          |----+----+----+----+----+----+----+----|";
            let footer = "|----+----+----+----+----+----+----+----|";
            let mut cal_content: String = "".to_string();

            cal_content = loop {

                let week_nb = month_day.format("%W");
                let mut weekday = format!("{}",month_day.format("%u")).parse::<usize>().unwrap();

                let mut week_array: [String; 7] = ["  ".to_string(),"  ".to_string(),"  ".to_string(),
                                                   "  ".to_string(),"  ".to_string(),"  ".to_string(),
                                                   "  ".to_string()];

                month_day = loop{
                    /*
                        take the weekday and place it at the right place in the week_array
                        (Monday is 0 and sunday is 6)
                    */
                    week_array[weekday - 1] = month_day.format("%d").to_string();

                    month_day = month_day + Duration::days(1);

                    weekday += 1 ;
                    if weekday > 7 || format!("{}",month_day.format("%m")).parse::<u32>().unwrap() != date {break month_day;}
                };


                let cal_line = format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                                        week_nb,week_array[0],week_array[1],week_array[2],
                                        week_array[3],week_array[4],week_array[5],week_array[6]);

                cal_content = format!("{}{}",cal_content,cal_line);

                if format!("{}",month_day.format("%m")).parse::<u32>().unwrap() != date {break cal_content;}
            };
            begin_content = format!("{}\n{}{}\n\n* Forcast\n\n",header,cal_content,footer);
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
