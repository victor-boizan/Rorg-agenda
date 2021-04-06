use std::fs;

use std::fs::File;
use std::io::prelude::*;

use chrono::{DateTime, Utc, TimeZone, Duration};


enum TimeRange{
    Year,
    Month,
    Week
}


fn main() -> std::io::Result<()> {

    dir_init()
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
                            \t** In Progress\n\n\
                            \t** To be done\n\n\
                            \t** Done\n\n\n\
                            * Notes\n\n\
                            * Records\n";

    format!("{}{}{}",file_title,begin_content,generic_content)

}
