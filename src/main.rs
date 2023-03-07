use std::{fs, env::args};

use owo_colors::{self, OwoColorize};
use math::round;

mod ipa_report;
mod raw_ipa_report;
//use owo_colors::OwoColorize;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Usage: ./testparser <file>");
    }
 
    let data = fs::read_to_string(args.get(1).unwrap()).unwrap();  

    let raw_report = raw_ipa_report::raw_report_from_str(remove_non_ascii(data)).unwrap();

    let formatted_report = ipa_report::SweepReport::from_raw_ipa_report(raw_report.clone());

    println!("\n{} Model: {}, SN: {}, Cal Date: {}\n", "Device:".green().underline(), formatted_report.devices.get(0).unwrap().model.yellow(), formatted_report.devices.get(0).unwrap().serial_number.yellow(), formatted_report.devices.get(0).unwrap().calibration_date.to_string().yellow());

    test_summary(formatted_report);

}

fn test_summary(input: ipa_report::SweepReport){
    //input.reports.sort_by(|a, b| a.tag.cmp(&b.tag));

    for input in input.reports{

        println!("{}: {: <15} {}: {: <6} {}: {: <6} {}: {: <6}",
        "Tag".green(),
        input.tag,
        "Length".green(),
        match input.dtf_marker {
            Some(_) => round::half_away_from_zero(input.dtf_marker.unwrap(), 2).to_string(),
            None => "N/A".red().to_string(),
        },
        "VSWR".green(),
        match input.dtf_result {
            Some(_) => round::half_away_from_zero(input.dtf_result.unwrap().max.1, 2).to_string(),
            None => "N/A".red().to_string(),
        },
        "RL".green(),
        match input.rl_result {
            Some(_) => round::half_away_from_zero(input.rl_result.unwrap().max.1, 2).to_string(),
            None => "N/A".red().to_string(),
        },
        );
    }
}

fn remove_non_ascii(input: String) -> String {

    let mut cleaned_data: Vec<char> = Vec::new();

    for digit in input.as_bytes() {
        let char = digit.to_owned() as char;
        if char.is_ascii() {
            cleaned_data.push(char);
        }
    }

    let cleaned_string: String = cleaned_data.iter().cloned().collect::<String>();

    return cleaned_string
    
}