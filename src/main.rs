use std::{fs, time};

use chrono::Timelike;
use math::round;
use owo_colors::OwoColorize;
use argh::FromArgs;


mod ipa_report;
mod raw_ipa_report;

#[derive(FromArgs)]
///Parser for Kaelus Sweep Tester Reports.
struct Arguments {
    ///path of Report.xml
    #[argh(positional)]
    path: String,
    ///sort the cables by tag
    #[argh(switch, short = 's')]
    sort: bool,
    ///displays extra information
    #[argh(switch, short = 'v')]
    verbose: bool
}

fn main() {

    let args: Arguments = argh::from_env();

    let start = time::Instant::now();

    print!("\nStart:({})", format!("{}ms", start.elapsed().as_millis()).red());
 
    let data = fs::read_to_string(args.path).unwrap();  
    
    let raw_report = raw_ipa_report::raw_report_from_str(remove_non_ascii(data)).unwrap();

    print!(", Parsed:({})", format!("{}ms", start.elapsed().as_millis()).red());

    let (formatted_report, warnings) = ipa_report::SweepReport::from_raw_ipa_report(raw_report).unwrap();

    println!(", Processed:({})\n", format!("{}ms", start.elapsed().as_millis()).red());

    for warning in warnings{
        println!("{} ({}) ({}:{})", "WARN: ".yellow().bold(), warning.message, warning.expected, warning.result)
    }
    
    match args.verbose{
        true => detailed_summary(args.sort, formatted_report),
        false => summary(args.sort, formatted_report),
    }

}


fn detailed_summary(sort: bool, mut input: ipa_report::SweepReport){
    for i in 0..input.devices.len() {

        let devce = input.devices.get(i).unwrap();
        
        println!("\n{}: Model: {}, SN: {}, Version: {}, FCal_Date: {}\n",
            format!("{}{}{}", "Device: (".green(), i.red(), ")".green()), 
            devce.model.yellow(),
            devce.serial_number.yellow(),
            devce.sw_version.yellow(),
            devce.calibration_date.yellow()
        )
    }

    if sort{
        input.reports.sort_by(|a, b| a.tag.cmp(&b.tag));
    }

    // println!("{}: {: <15} {}: {: <6} {}: {: <6} {}: {: <6}",

    for input in input.reports {
        println!("\nTag: {}\n", input.tag.bold().underline().green());

        println!("  {:<4}: Length (m): {:<16} VSWR: {:<46} || T+Cal: {} Date: {}",
            "DTF".red().bold(),
            match input.dtf_marker {
                Some(e) => round::half_away_from_zero(e, 2).green().to_string(),
                None => "N/A".red().to_string(),
            },
            match input.dtf_result.clone() {
                Some(e) => format!("{} at {}{}",round::half_away_from_zero(e.max.1, 2).green(), round::half_away_from_zero(e.max.0, 2).green(), "m".green()),
                None => "N/A".red().to_string(),
            },
            match input.dtf_result.clone() {
                Some(e) => format!("{:0>2}:{:0>2}", e.calibrated.hour(), e.calibrated.minute()).green().to_string(),
                None => "N/A".red().to_string(),
            },
            match input.dtf_result.clone() {
                Some(e) => format!("{} {:0>2}:{:0>2}", e.time.date(), e.time.hour(), e.time. minute()).green().to_string().to_string(),
                None => "N/A".red().to_string(),
            }

            
        );

        println!("  {:<4}: RL (dBm):  {:<41}|| T+Cal: {} Date: {}",
            input.rl_state.unwrap().test_type.blue().bold(),
            match input.rl_result.clone() {
                Some(e) => round::half_away_from_zero(e.max.1, 2).green().to_string(),
                None => "N/A".red().to_string(),
            },
            match input.rl_result.clone() {
                Some(e) => format!("{:0>2}:{:0>2}", e.calibrated.hour(), e.calibrated.minute()).green().to_string(),
                None => "N/A".red().to_string(),
            },
            match input.rl_result.clone() {
                Some(e) => format!("{} {:0>2}:{:0>2}", e.time.date(), e.time.hour(), e.time. minute()).green().to_string().to_string(),
                None => "N/A".red().to_string(),
            }
        );

    }
}

fn summary(sort: bool,mut input: ipa_report::SweepReport){

    for i in 0..input.devices.len() {

        let devce = input.devices.get(i).unwrap();
        
        println!("\n{}: Model: {}, SN: {}, Version: {}, FCal_Date: {}\n",
            format!("{}{}{}", "Device: (".green(), i.red(), ")".green()), 
            devce.model.yellow(),
            devce.serial_number.yellow(),
            devce.sw_version.yellow(),
            devce.calibration_date.yellow()
        )
    }

    if sort{
        input.reports.sort_by(|a, b| a.tag.cmp(&b.tag));
    }
    
    for input in input.reports{

        println!("{}: {: <15} {}: {: <6} {}: {: <6} {}: {: <6}",
        "Tag".green(),
        input.tag,
        "Length".green(),
        match input.dtf_marker {
            Some(e) => round::half_away_from_zero(e, 2).to_string(),
            None => "N/A".red().to_string(),
        },
        "VSWR".green(),
        match input.dtf_result {
            Some(e) => round::half_away_from_zero(e.max.1, 2).to_string(),
            None => "N/A".red().to_string(),
        },
        "RL".green(),
        match input.rl_result {
            Some(e) => round::half_away_from_zero(e.max.1, 2).to_string(),
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

    cleaned_string
    
}