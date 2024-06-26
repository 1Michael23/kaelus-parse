#![allow(non_snake_case)]

use std::{fs, env};

use crate::raw_ipa_report;
use chrono::{DateTime, FixedOffset, prelude::*};

fn read_csv_marker_position(path: String) -> f64 {

    let args: Vec<String> = env::args().collect();
    let env_path = args.get(1).unwrap().trim_end_matches("Report.xml").to_string();

    let raw_csv = fs::read_to_string(env_path+&path).unwrap();
    let lines_vec: Vec<&str> = raw_csv.lines().collect();

    let marker_data: Vec<&str> = lines_vec.get(13).unwrap().split(',').collect();
    
    return marker_data.get(2).unwrap().to_owned().to_owned().parse().unwrap();
}

pub struct Warning {
    pub message: String,
    pub expected: String,
    pub result: String
}

enum TestType {
    DTF,
    ReturnLoss,
}

impl SweepReport {

    pub fn from_raw_ipa_report(input: raw_ipa_report::Bundle) -> Result<(SweepReport, Vec<Warning>), String> {

        //Create Vector for parse warnings
        let mut warnings: Vec<Warning> = Vec::new();

        //Create Vector for the testing devices
        let mut tmp_devices: Vec<Device> = Vec::new();

            //Loop over devices in the raw report
            for device in input.Devices.Device.clone() {

                let details_vec = device.Details.DeviceDetails;
                let item;
                
                if details_vec.is_empty() {
                    return Err("Empty device details vector".to_string())
                }
                
                if details_vec.len() != 1{
                    warnings.push(Warning { 
                        message: "Unexpected details Vec len".to_string(), 
                        expected: "1".to_string(), 
                        result: details_vec.len().to_string() });
                }

                item = details_vec.get(0).unwrap();
                

                let tmp_device: Device = Device { 
                    serial_number: (device.SerialNumber), 
                    model: (device.Model), 
                    ID: (item.ID.clone()), 
                    sw_version: (item.SWVersions.clone()), 
                    calibration_date: (DateTime::parse_from_rfc3339(item.CalDate.as_str()).expect("Time Parse Error")), 
                    signature: (item.Signature.clone()) 
                };

                tmp_devices.push(tmp_device);
            }

            let mut tmp_reports: Vec<Report> = Vec::new();

            for test in input.Reports.Report.get(0).unwrap().Items.Test.clone() {
                
                let mut duplicate_tag: bool = false;
               
                let test_type: TestType = match test.Results.TestResult.Unit.as_str() {
                    "VSWR" => TestType::DTF,
                    "dB" => TestType::ReturnLoss,
                    _ => panic!("Unknown test type"),
                };

                let mut associated_state: Option<raw_ipa_report::State> = None;
                let mut associated_csv_path: Option<String> = None;

                for raw_state in input.States.State.clone() {
                    if raw_state.ID == test.StateID{
                        associated_state = Some(raw_state);
                        break;
                    }
                }

                if associated_state.is_none() {
                    panic!("No associated state found.")
                }

                for asset in test.Assets.Asset {
                    if asset.ends_with(".csv"){
                        associated_csv_path = Some(asset);
                    }
                }

                if associated_csv_path.is_none(){
                    panic!("No associated CSV file linked in report")
                }

                let max: Vec<&str> = test.Results.TestResult.Maximum.split(':').collect();
                let min: Vec<&str> = test.Results.TestResult.Minimum.split(':').collect();

                    let tmp_result: TestResult = TestResult { 
                        measurement_type: test.Results.TestResult.MeasurementType, 
                        unit: test.Results.TestResult.Unit, 
                        p1: test.Results.TestResult.P1.parse().unwrap(), 
                        p2: test.Results.TestResult.P2.parse().unwrap(), 
                        max: (max.first().unwrap().to_string().parse().unwrap(), max.get(1).unwrap().to_string().parse().unwrap()), 
                        min: (min.first().unwrap().to_string().parse().unwrap(), min.get(1).unwrap().to_string().parse().unwrap()), 
                        avg: test.Results.TestResult.Average.parse().unwrap(), 
                        ripple: test.Results.TestResult.Ripple.parse().unwrap(), 
                        pass: test.Results.TestResult.Pass.parse().unwrap(),
                        calibrated: test.Calibrated.parse().unwrap(),
                        time: Utc.datetime_from_str(&test.Time, "%Y-%m-%d %H:%M").expect("Invalid time format").naive_local(),
                    };

                for tmp_test in &mut tmp_reports {

                    if test.Tags.is_empty() {
                        panic!("No tags found on cable, Check Report.")
                    }
                    if test.Tags.len() > 1 {
                        panic!("Too many tags found on cable, Check Report.")
                    }

                    if test.Tags.get(0).unwrap().Tag == tmp_test.tag {
                        duplicate_tag = true;
 
                        for raw_state in input.States.State.clone() {
                            if raw_state.ID == test.StateID{
                                associated_state = Some(raw_state);
                                break;
                            }
                        }
                        if associated_state.is_none() {
                            panic!("No associated state found.")
                        }

                        match test_type {
                            TestType::DTF => {
                                tmp_test.dtf_state_id = Some(test.StateID.clone());
                                tmp_test.dtf_state = Some(DtfState::from_raw(associated_state.clone().unwrap()));
                                tmp_test.dtf_result = Some(tmp_result.clone());
                                tmp_test.dtf_marker = Some(read_csv_marker_position(associated_csv_path.clone().unwrap()))
                            },
                            TestType::ReturnLoss => {
                                tmp_test.rl_state_id = Some(test.StateID.clone());
                                tmp_test.rl_state = Some(RlState::from_raw(associated_state.clone().unwrap()));
                                tmp_test.rl_result = Some(tmp_result.clone());
                            },
                        }
                    }
                }

                if !duplicate_tag {
                    match test_type {
                        TestType::DTF => {
                            let tmp_report: Report = Report { 
                                tag: test.Tags.get(0).unwrap().Tag.clone(), 
                                dtf_state_id: Some(test.StateID), 
                                rl_state_id: None, 
                                dtf_state: Some(DtfState::from_raw(associated_state.unwrap())), 
                                rl_state: None, 
                                dtf_marker: Some(read_csv_marker_position(associated_csv_path.unwrap())),
                                dtf_result: Some(tmp_result),
                                rl_result: None
                                };
                                tmp_reports.push(tmp_report)
                        },
                        TestType::ReturnLoss => {
                            let tmp_report: Report = Report { 
                                tag: test.Tags.get(0).unwrap().Tag.clone(), 
                                dtf_state_id: None, 
                                rl_state_id: Some(test.StateID), 
                                dtf_state: None, 
                                rl_state: Some(RlState::from_raw(associated_state.unwrap())), 
                                dtf_marker: None,
                                dtf_result: None,
                                rl_result: Some(tmp_result)
                                };
                                tmp_reports.push(tmp_report)
                        },
                    }    
                }
            }

        let sweep_report: SweepReport = SweepReport { devices: (tmp_devices), reports: (tmp_reports) };

        Ok((sweep_report, warnings))
    }
}

#[derive(Debug)]
pub struct SweepReport {
    pub devices: Vec<Device>,
    pub reports: Vec<Report>

}
    #[derive(Debug)]
    pub struct Device {
        pub serial_number: String,
        pub model: String,
        pub ID: String,
        pub sw_version: String,
        pub calibration_date: DateTime<FixedOffset>,
        pub signature: String
    }

    #[derive(Debug, Clone)]
    pub struct Report {
        pub tag: String,
        pub dtf_state_id: Option<String>,
        pub rl_state_id: Option<String>,
        pub dtf_state: Option<DtfState>,
        pub rl_state: Option<RlState>,
        pub dtf_marker: Option<f64>, 
        pub dtf_result: Option<TestResult>,
        pub rl_result: Option<TestResult>,
    }

        impl DtfState {
            pub fn from_raw(input: raw_ipa_report::State) -> DtfState {
                let input_clone = input.clone().Rx_kHz.unwrap();
                let rx_khz: Vec<&str> = input_clone.split(':').collect();
                let limit = input.Limits.Limit.get(0).unwrap().clone();

                let result: DtfState = DtfState { 
                    id: input.ID, 
                    test_type: input.TestType, 
                    rx_khz: (rx_khz.first().unwrap().to_string().parse().unwrap(), rx_khz.get(1).unwrap().to_string().parse().unwrap() ), 
                    points: input.Points.parse().unwrap(), 
                    limit_distance: input.Distance_m.unwrap().parse().unwrap(), 
                    cable_loss_dbm: input.CableLoss_dB_per_m.unwrap().parse().unwrap(), 
                    limit: Limit { 
                        limit_type: limit.Type, 
                        measurement_type: limit.MeasurementType, 
                        unit: limit.Unit, 
                        name: limit.Name, 
                        reference_value: limit.Reference} 
                };
                result
            }
        }

        #[derive(Debug, Clone)]
        pub struct DtfState {
            pub id: String,
            pub test_type: String,
            pub rx_khz: (u64,u64),
            pub points: u64,
            pub limit_distance: u64,
            pub cable_loss_dbm: f64,

            pub limit: Limit
        }

        impl RlState {
            pub fn from_raw(input: raw_ipa_report::State) -> RlState { 

                let limit = input.Limits.Limit.get(0).unwrap().clone();

                let result: RlState = RlState { 
                    id: input.ID, 
                    test_type: input.TestType, 
                    points: input.Points.parse().unwrap(), 
                    limit: Limit { 
                        limit_type: limit.Type, 
                        measurement_type: limit.MeasurementType, 
                        unit: limit.Unit, 
                        name: limit.Name, 
                        reference_value: limit.Reference 
                    } 
                };

                result
            }
        }

        #[derive(Debug, Clone)]
        pub struct RlState {
            pub id: String, 
            pub test_type: String,
            pub points: u64,

            pub limit: Limit
        }
    
            #[derive(Debug, Clone)]
            pub struct Limit {
                pub limit_type: String,
                pub measurement_type: String,
                pub unit: String,
                pub name: String,
                pub reference_value: String
            }

        #[derive(Debug, Clone)]
        pub struct TestResult {
            pub measurement_type: String,
            pub unit: String,
            pub p1: u32,
            pub p2: u32,
            pub max: (f64, f64),
            pub min: (f64, f64),
            pub avg: f64,
            pub ripple: f64,
            pub pass: bool,
            pub calibrated: chrono::NaiveTime,
            pub time: chrono::NaiveDateTime
        }
