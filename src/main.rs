use std::fs;

use std::fs::File;
use std::io::prelude::*;

use chrono::{DateTime, Utc};

fn main() -> std::io::Result<()> {

    dir_init()

}

fn dir_init() -> std::io::Result<()> {

    let now: DateTime<Utc> = Utc::now();

    /*Create the folders*/
    fs::create_dir_all("./rorg/current/month")?;
    fs::create_dir_all("./rorg/current/weeks")?;
    fs::create_dir_all("./rorg/next-years")?;
    fs::create_dir_all("./rorg/.achive")?;

    /*Create the files*/

    //current year

    let current_folder = "./rorg/current/";

    let year_file_name = format!("{}/{}.org",current_folder, now.format("%Y"));
    let file = File::create(year_file_name);

    //weeks
    for i in 1..53{
        //idk if their is a better way to do it

        let filename: String;

        if i < 10{
            filename = format!("{}weeks/w0{}.org",current_folder,i);
        }else{
            filename = format!("{}weeks/w{}.org",current_folder,i);
        }
        let file = File::create(filename)?;
    }

    //month
    let file = File::create("./rorg/current/month/01-januarry.org");
    let file = File::create("./rorg/current/month/02-february.org");
    let file = File::create("./rorg/current/month/03-march.org");
    let file = File::create("./rorg/current/month/04-april.org");
    let file = File::create("./rorg/current/month/05-may.org");
    let file = File::create("./rorg/current/month/06-june.org");
    let file = File::create("./rorg/current/month/07-jully.org");
    let file = File::create("./rorg/current/month/08-august.org");
    let file = File::create("./rorg/current/month/09-september.org");
    let file = File::create("./rorg/current/month/10-october.org",);
    let file = File::create("./rorg/current/month/11-november.org");
    let file = File::create("./rorg/current/month/12-december.org");

    //other files

    let file = File::create("./rorg/habits.org");
    let file = File::create("./rorg/appoitment.org");
    let file = File::create("./rorg/special_days.org");

    Ok(())
}
