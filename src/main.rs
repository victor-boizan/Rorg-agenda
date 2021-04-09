/* standard */
use std::fs;

use std::fmt;

use std::fs::File;
use std::io::prelude::*;

use std::str::FromStr;


use regex::Regex;

use chrono::{DateTime, Utc, TimeZone, Duration};


enum TimeRange{
    Year,
    Month,
    Week
}
#[derive(Debug)]
enum EventState{
    TODO,
    WIP,
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
            "REPORT"  => Ok(EventState::REPORT),
            "DONE" => Ok(EventState::DONE),
            "" => Ok(EventState::Null),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum EventStyle{
    Task,
    Habit,
    Appointment,
    BasicEvent
}

#[derive(Debug)]
struct Task {
    name: String,
    state: EventState,
    style: EventStyle,
    priority: u8,
    description: String,
    logs: String,
    notes: String
}
impl Task{
    fn new(name: String, priority: u8) -> Task{
        Task{
            name: name,
            state: EventState::TODO,
            style: EventStyle::Task,
            priority: priority,
            description: "".to_string(),
            logs: "".to_string(),
            notes: "".to_string()
        }
    }
}
impl fmt::Display for Task {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "***** {:?} [#{}] {}\
                   \n:PROPERTIES:\
                   \n:STYLE: {:?}\
                   \n:END:\
                   \n:DESCRIPTION:\
                   \n{}\
                   \n:END:\
                   \n:NOTES:\
                   \n{}\
                   \n:END:\
                   ",
                    self.state, self.priority, self.name, self.style, self.description, self.notes)
    }
}
impl FromStr for Task {

    type Err = ();
    fn from_str(input: &str) -> Result<Task, ()> {

        let task_re = Regex::new(r"(?m)^\*{5} (?P<state>\S{3,4}) \[#(?P<priority>\d+)] (?P<name>.*)\n:PROPERTIES:\n(:[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>(^[^:]*\n)*):END:\n:NOTES:\n(?P<notes>(^[^:]*\n)*):END:").unwrap();
        let caps = task_re.captures(input).unwrap();

        Ok(Task{
            name: caps["name"].to_string(),
            state: EventState::from_str(&caps["state"]).unwrap(),
            style: EventStyle::Task,
            priority: caps["priority"].parse::<u8>().unwrap(),
            description: caps["description"].to_string(),
            logs: "".to_string() ,
            notes: caps["notes"].to_string()
        })
    }
}

#[derive(Debug)]
struct Habit {
    name: String,
    state: EventState,
    style: EventStyle,
    description: String,
    logs: String,
    notes: String
}
impl Habit{
    fn new(name: String) -> Habit{
        Habit{
            name: name,
            state: EventState::TODO,
            style: EventStyle::Habit,
            description: "".to_string(),
            logs: "".to_string(),
            notes: "".to_string()
        }
    }
}
impl fmt::Display for Habit {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "***** {:?} {}\
                   \n:PROPERTIES:\
                   \n:STYLE: {:?}\
                   \n:END:\
                   \n:DESCRIPTION:\
                   \n{}\
                   \n:END:\
                   \n:NOTES:\
                   \n{}\
                   \n:END:\
                   ",
                    self.state, self.name, self.style, self.description, self.notes)
    }
}
impl FromStr for Habit {

    type Err = ();
    fn from_str(input: &str) -> Result<Habit, ()> {

        let task_re = Regex::new(r"(?m)^\*{5} (?P<state>\S{3,4}) (?P<name>.*)\n:PROPERTIES:\n(:[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>(^[^:]*\n)*):END:\n:NOTES:\n(?P<notes>(^[^:]*\n)*):END:").unwrap();
        let caps = task_re.captures(input).unwrap();

        Ok(Habit{
            name: caps["name"].to_string(),
            state: EventState::from_str(&caps["state"]).unwrap(),
            style: EventStyle::Habit,
            description: caps["description"].to_string(),
            logs: "".to_string() ,
            notes: caps["notes"].to_string()
        })
    }
}

#[derive(Debug)]
struct Appointment {
    name: String,
    state: EventState,
    style: EventStyle,
    description: String,
    logs: String,
    notes: String
}
impl Appointment{
    fn new(name: String) -> Appointment{
        Appointment{
            name: name,
            state: EventState::TODO,
            style: EventStyle::Appointment,
            description: "".to_string(),
            logs: "".to_string(),
            notes: "".to_string()
        }
    }
}
impl fmt::Display for Appointment {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "***** {:?} {}\
                   \n:PROPERTIES:\
                   \n:STYLE: {:?}\
                   \n:END:\
                   \n:DESCRIPTION:\
                   \n{}\
                   \n:END:\
                   \n:NOTES:\
                   \n{}\
                   \n:END:\
                   ",
                    self.state, self.name, self.style, self.description, self.notes)
    }
}
impl FromStr for Appointment {

    type Err = ();
    fn from_str(input: &str) -> Result<Appointment, ()> {

        let task_re = Regex::new(r"(?m)^\*{5} (?P<state>\S{3,4}) (?P<name>.*)\n:PROPERTIES:\n(:[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>(^[^:]*\n)*):END:\n:NOTES:\n(?P<notes>(^[^:]*\n)*):END:").unwrap();
        let caps = task_re.captures(input).unwrap();

        Ok(Appointment{
            name: caps["name"].to_string(),
            state: EventState::from_str(&caps["state"]).unwrap(),
            style: EventStyle::Appointment,
            description: caps["description"].to_string(),
            logs: "".to_string() ,
            notes: caps["notes"].to_string()
        })
    }
}

#[derive(Debug)]
struct BasicEvent {
    name: String,
    style: EventStyle,
    description: String,
    notes: String
}
impl BasicEvent{
    fn new(name: String) -> BasicEvent{
        BasicEvent{
            name: name,
            style: EventStyle::BasicEvent,
            description: "".to_string(),
            notes: "".to_string()
        }
    }
}
impl fmt::Display for BasicEvent {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "***** {}\
                   \n:PROPERTIES:\
                   \n:STYLE: {:?}\
                   \n:END:\
                   \n:DESCRIPTION:\
                   \n{}\
                   \n:END:\
                   \n:NOTES:\
                   \n{}\
                   \n:END:\
                   ",
                    self.name, self.style, self.description, self.notes)
    }
}
impl FromStr for BasicEvent {

    type Err = ();
    fn from_str(input: &str) -> Result<BasicEvent, ()> {

        let task_re = Regex::new(r"(?m)^\*{5} (?P<name>.*)\n:PROPERTIES:\n(:[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(?P<description>(^[^:]*\n)*):END:\n:NOTES:\n(?P<notes>(^[^:]*\n)*):END:").unwrap();
        let caps = task_re.captures(input).unwrap();

        Ok(BasicEvent{
            name: caps["name"].to_string(),
            style: EventStyle::BasicEvent,
            description: caps["description"].to_string(),
            notes: caps["notes"].to_string()
        })
    }
}

fn main() -> std::io::Result<()> {

    test_things();
    Ok(())
}

fn dir_init() -> std::io::Result<()> {

    /*Create the folders*/
    fs::create_dir_all("./rorg/current/months")?;
    fs::create_dir_all("./rorg/current/weeks")?;
    fs::create_dir_all("./rorg/next-years")?;
    fs::create_dir_all("./rorg/.achives")?;

    /*Create the files*/
    let current_folder = "./rorg/current/";

    //get the year,month and week and store them in u32 and i32 for the year
    let now: DateTime<Utc> = Utc::now();

    let year = format!("{}",now.format("%Y")).parse::<i32>().unwrap();
    let month = format!("{}",now.format("%m")).parse::<u32>().unwrap();
    let week = format!("{}",now.format("%W")).parse::<u32>().unwrap();

    //current year
    let filename = format!("{}/{}.org",current_folder, now.format("%Y"));
    let mut file = File::create(filename)?;
    let content = file_generator(TimeRange::Year,year,0);
    file.write_all(content.as_bytes())?;

    //weeks
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

    //month
    for file_month in month..13{

        let work_month = Utc.ymd(year, file_month, 1);
        let filename = format!("{}months/{}.org",current_folder,work_month.format("%m-%B"));
        let mut file = File::create(filename)?;
        let content = file_generator(TimeRange::Month,year,file_month);
        file.write_all(content.as_bytes())?;
    }

    //other files

    let mut file = File::create("./rorg/habits.org")?;
    let content = "#+TITLE: Habits";
    file.write_all(content.as_bytes())?;

    let mut file = File::create("./rorg/appointments.org")?;
    let content = "#+TITLE: Appointments";
    file.write_all(content.as_bytes())?;

    let mut file = File::create("./rorg/special_times.org")?;
    let content = "#+TITLE: Special times\n# I use this file for evey recurent events like birthdays";
    file.write_all(content.as_bytes())?;

    Ok(())
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

/* Test functions (will be remove a one point)*/

fn test_things() -> std::io::Result<()>{


    dir_init().expect("cannot init dirs");


    let a_task: Task = Task::new("Task exemple".to_string(), 0);
    let an_habit: Habit = Habit::new("habit exemple".to_string());
    let an_appointment: Appointment = Appointment::new("Appointment exemple".to_string());
    let an_basic_event: BasicEvent = BasicEvent::new("event exemple".to_string());


    let mut file = File::open("./rorg/current/2021.org").expect("cannot open file (1)");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("cannot read file (1)");
    drop(file);


    let mut file = File::create("./rorg/current/2021.org").expect("cannot open file (1)");
    file_content = format!("{}\n{}",file_content,a_task);
    file.write_all(file_content.as_bytes()).expect("cannot write file");
    drop(file);

    let mut file = File::open("./rorg/current/2021.org").expect("cannot open file(2)");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file(2)");

    let task_re = Regex::new(r"(?m)^\*{5} \S{3,4} \[#\d+] .*\n:PROPERTIES:\n(:[A-Z]*: .*\n)*:END:\n:DESCRIPTION:\n(^[^:]*\n)*:END:\n:NOTES:\n(^[^:]*\n)*:END:").unwrap();

    for event in task_re.captures_iter(&contents) {
        let another_task = Task::from_str(event.get(0).unwrap().as_str()).unwrap();
        println!("+-----a task in a a file-----+\n\n{}\n+-----------------------------------+",another_task);
    }

    let an_habit: Habit = Habit::from_str(format!("{}",an_habit).as_str()).unwrap();
    let an_appointment: Appointment = Appointment::from_str(format!("{}",an_appointment).as_str()).unwrap();
    let an_basic_event: BasicEvent = BasicEvent::from_str(format!("{}",an_basic_event).as_str()).unwrap();

    println!("{}\n{}\n{}",an_habit,an_appointment,an_basic_event);
    Ok(())

}
