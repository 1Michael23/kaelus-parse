#![allow(non_snake_case)]

use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::from_str;

pub fn raw_report_from_str(input: String) -> Result<Bundle, serde_xml_rs::Error> {
    from_str(&input)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bundle {
    pub Version: String,
    pub Devices: Devices,
    pub States: States,
    pub Reports: Reports
}
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Devices {
        pub Device: Vec<Device>,
    }

        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct Device {
            pub SerialNumber: String,
            pub Model: String,
            pub Details: Details
        }
        
            #[derive(Debug, Serialize, Deserialize, Clone)]
            pub struct Details {
                pub DeviceDetails: Vec<DeviceDetails>
            }
                
                #[derive(Debug, Serialize, Deserialize, Clone)]
                pub struct DeviceDetails {
                    pub ID: String,
                    pub SWVersions: String,
                    pub CalDate: String,
                    pub Signature: String,
                }
    
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct States {
        pub State: Vec<State>
    }

        #[derive(Debug, Serialize, Deserialize, Clone)] 
        pub struct State {
            pub ID: String,
            pub TestType: String,
            pub Rx_kHz: Option<String>,
            pub Points: String,
            pub Distance_m: Option<String>,
            pub VF: Option<String>,
            pub Window: Option<String>,
            pub CableLoss_dB_per_m: Option<String>,

            pub Limits: Limits

        }

            #[derive(Debug, Serialize, Deserialize, Clone)] 
            pub struct Limits {
                pub Limit: Vec<Limit>
            }
    
                #[derive(Debug, Serialize, Deserialize, Clone)]
                pub struct Limit {
                    pub Type: String,
                    pub MeasurementType: String,
                    pub Unit: String,
                    pub Name: String,
                    pub Range: Option<String>,
                    pub Reference: String
                }

    #[derive(Debug, Serialize, Deserialize, Clone)]            
    pub struct Reports {
        pub Report: Vec<Report>
    }

        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct Report {
            pub ID: String,
            pub PeakPim_dBm: String,
            pub PeakPimPowerSetPoint_dBm: String,
            pub TestPassed: String,

            pub Items: Items
        }

            #[derive(Debug, Serialize, Deserialize, Clone)]
            pub struct Items {
                pub Test: Vec<Test>
            }

                #[derive(Debug, Serialize, Deserialize, Clone)]
                pub struct Test {
                    pub ID: String,
                    pub StateID: String,
                    pub Time: String,
                    pub Calibrated: String,


                    pub Assets: Assets,
                    pub Tags: Vec<Tag>,
                    pub Results: Results

                }

                    #[derive(Debug, Serialize, Deserialize, Clone)]
                    pub struct Assets {
                        #[serde(rename = "$value")]
                        pub Asset: Vec<String>
                    }

                    #[derive(Debug, Serialize, Deserialize, Clone)] 
                        pub struct Tag {
                            pub Tag: String
                        }

                    #[derive(Debug, Serialize, Deserialize, Clone)] 
                    pub struct TestDevices {
                        pub Devices: Vec<TestDeviceDetails> 
                    }

                        #[derive(Debug, Serialize, Deserialize, Clone)] 
                        pub struct TestDeviceDetails {
                            pub ID: String
                        }
                    
                #[derive(Debug, Serialize, Deserialize, Clone)] 
                pub struct Results {
                    pub TestResult: TestResult
                }

                    #[derive(Debug, Serialize, Deserialize, Clone)] 
                    pub struct TestResult {
                        pub MeasurementType: String,
                        pub Unit: String,
                        pub P1: String,
                        pub P2: String,
                        pub Maximum: String,
                        pub Minimum: String,
                        pub Average: String,
                        pub Ripple: String,
                        pub Pass: String,
                    }