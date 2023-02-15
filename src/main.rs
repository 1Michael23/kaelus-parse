use std::{fs, env::args};

mod ipa_report;
mod raw_ipa_report;
//use owo_colors::OwoColorize;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Usage: ./testparser <file>");
    }
 
    let data = fs::read_to_string(args.get(1).unwrap()).unwrap();  

    let mut cleaned_data: Vec<char> = Vec::new();

    for digit in data.as_bytes() {
        let char = digit.to_owned() as char;
        if char.is_ascii() {
            cleaned_data.push(char);
        }
    }

    let cleaned_string: String = cleaned_data.iter().cloned().collect::<String>();

    let raw_report = raw_ipa_report::raw_report_from_str(cleaned_string).unwrap();

    let formatted_report = ipa_report::SweepReport::from_raw_ipa_report(raw_report.clone());


    for cable in formatted_report.reports {
        println!("Tag: {: <15} Length: {: <25} VSWR: {: <25} RL: {: <25}", cable.tag, cable.dtf_marker.unwrap(), cable.dtf_result.unwrap().max.1, cable.rl_result.unwrap().max.1)
    }

}

