use serde::{Deserialize, Serialize};

// use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::path::Path;
use std::{env, str};

#[derive(Deserialize, Debug)]
struct Validator {
    address: String,
    voting_power: String,
}

#[derive(Deserialize, Debug)]
struct ValidatorsAtHeight {
    block_height: String,
    validators: Vec<Validator>,
}

#[derive(Deserialize, Debug)]
struct ValidatorsData {
    result: ValidatorsAtHeight,
}

// #[derive(Serialize)]
// struct ValidatorVotingPowerData {
//     address: String,
//     total_vp: i64,
//     validator_vp: i64,
//     rank: i64,
// }

#[derive(Debug)]
struct BlockVPResultData {
    block_height: i64,
    block_voting_power: f64,
    number_of_validators: i32,
    cumulative_validator_vp_percentage: f64,
}

#[derive(Serialize)]
struct CsvBlockVpResultRecord {
    block_height: i64,
    block_voting_power: f64,
    // this contains both the nakamoto coefficient
    // and the takeover threshold
    nc_number_of_validators: i32,
    nc_cumulative_validator_vp_percentage: f64,
    tt_number_of_validators: i32,
    tt_cumulative_validator_vp_percentage: f64,
}

// this is a mapping of height -> data for each validator
// type ResultMap = HashMap<i64, Vec<ValidatorVotingPowerData>>;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    // ../../dydx
    let voting_power_files = &args[1];
    let output_path = &args[2];

    // don't like having to do this, but it's just a data export script
    // todo: wrap CLI args in option type
    let meta_exists = &args[3];
    let vp_files_path = Path::new(voting_power_files);

    //let paths = read_dir(vp_files_path).unwrap();

    // some endpoints don't return meta so the JSON structure is different (sigh)
    if meta_exists.eq("true") {
        let mut paths: Vec<_> = read_dir(vp_files_path)
            .unwrap()
            .map(|res| res.unwrap())
            .collect();
        paths.sort_by_key(|entry| {
            let vp_file = File::open(entry.path()).expect("Error: file not found");
            let vd_json: ValidatorsData =
                serde_json::from_reader(vp_file).expect("Error: error while reading");

            let validators_at_height: ValidatorsAtHeight = vd_json.result;
            let block_height: i64 = validators_at_height.block_height.parse().unwrap();
            block_height
        });

        // set up CSV
        let csv_output_path = output_path.to_owned() + ".csv";
        let csv_out_path = Path::new(&csv_output_path);

        let mut wtr = csv::Writer::from_path(csv_out_path).unwrap();

        for path in paths {
            let vp_file_path = path.path();
            println!("Name: {}", vp_file_path.clone().display());
            let vp_file = File::open(vp_file_path).expect("Error: file not found");

            let vd_json: ValidatorsData =
                serde_json::from_reader(vp_file).expect("Error: error while reading");

            let mut total_vp: f64 = 0.0;

            let validators_at_height: ValidatorsAtHeight = vd_json.result;
            let block_height: i64 = validators_at_height.block_height.parse().unwrap();
            let mut validators: Vec<Validator> = validators_at_height.validators;
            validators.sort_by(|val_a, val_b| {
                let validator_a_voting_power: i64 = val_a.voting_power.parse().unwrap();
                let validator_b_voting_power: i64 = val_b.voting_power.parse().unwrap();
                validator_b_voting_power.cmp(&validator_a_voting_power)
            });

            // first calculate total vp for the block
            for validator in &validators {
                let validator_voting_power: f64 = validator.voting_power.parse().unwrap();
                total_vp = total_vp + validator_voting_power;
            }

            // then calculate the shares for vals
            // and cumulative share
            let mut cumulative_val_vp_percentage: f64 = 0.0;
            let mut cumulative_number_of_validators: i32 = 0;

            let mut nakamoto_coefficient: Option<BlockVPResultData> = None;
            let mut takeover_threshold: Option<BlockVPResultData> = None;

            for validator in validators {
                let validator_voting_power: f64 = validator.voting_power.parse().unwrap();
                let val_vp_percentage: f64 = validator_voting_power / total_vp;

                // add to running totals
                cumulative_val_vp_percentage = cumulative_val_vp_percentage + val_vp_percentage;
                cumulative_number_of_validators = cumulative_number_of_validators + 1;
                // println!(
                //     "Total VP: {}, Val VP: {}, Val pc: {}",
                //     total_vp, validator_voting_power, val_vp_percentage
                // );

                if cumulative_val_vp_percentage >= 0.667 {
                    if takeover_threshold.is_none() {
                        takeover_threshold = Some(BlockVPResultData {
                            block_height: block_height,
                            block_voting_power: total_vp,
                            number_of_validators: cumulative_number_of_validators,
                            cumulative_validator_vp_percentage: cumulative_val_vp_percentage
                                * 100.0,
                        })
                    }
                } else if cumulative_val_vp_percentage >= 0.334 {
                    if nakamoto_coefficient.is_none() {
                        nakamoto_coefficient = Some(BlockVPResultData {
                            block_height: block_height,
                            block_voting_power: total_vp,
                            number_of_validators: cumulative_number_of_validators,
                            cumulative_validator_vp_percentage: cumulative_val_vp_percentage
                                * 100.0,
                        })
                    }
                }
            }

            if let (Some(nc), Some(tt)) = (nakamoto_coefficient, takeover_threshold) {
                println!(
                    "Block Height: {}, Total VP: {}",
                    nc.block_height, nc.block_voting_power
                );
                println!(
                    "{} Validators to exceed 33.3% of VP, with {}% of VP",
                    nc.number_of_validators, nc.cumulative_validator_vp_percentage
                );
                println!(
                    "{} Validators to exceed 66.6% of VP, with {}% of VP",
                    tt.number_of_validators, tt.cumulative_validator_vp_percentage
                );

                wtr.serialize(CsvBlockVpResultRecord {
                    block_height: nc.block_height,
                    block_voting_power: nc.block_voting_power,
                    nc_number_of_validators: nc.number_of_validators,
                    nc_cumulative_validator_vp_percentage: nc.cumulative_validator_vp_percentage,
                    tt_number_of_validators: tt.number_of_validators,
                    tt_cumulative_validator_vp_percentage: tt.cumulative_validator_vp_percentage,
                })
                .unwrap();
            }
        }

        wtr.flush().unwrap();
    } else {
        let mut paths: Vec<_> = read_dir(vp_files_path)
            .unwrap()
            .map(|res| res.unwrap())
            .collect();
        paths.sort_by_key(|entry| {
            let vp_file = File::open(entry.path()).expect("Error: file not found");
            let validators_at_height: ValidatorsAtHeight =
                serde_json::from_reader(vp_file).expect("Error: error while reading");

            let block_height: i64 = validators_at_height.block_height.parse().unwrap();
            block_height
        });

        // set up CSV
        let csv_output_path = output_path.to_owned() + ".csv";
        let csv_out_path = Path::new(&csv_output_path);

        let mut wtr = csv::Writer::from_path(csv_out_path).unwrap();

        for path in paths {
            let vp_file_path = path.path();
            println!("Name: {}", vp_file_path.clone().display());
            let vp_file = File::open(vp_file_path).expect("Error: file not found");

            let validators_at_height: ValidatorsAtHeight =
                serde_json::from_reader(vp_file).expect("Error: error while reading");

            let mut total_vp: f64 = 0.0;

            let block_height: i64 = validators_at_height.block_height.parse().unwrap();
            let mut validators: Vec<Validator> = validators_at_height.validators;
            validators.sort_by(|val_a, val_b| {
                let validator_a_voting_power: i64 = val_a.voting_power.parse().unwrap();
                let validator_b_voting_power: i64 = val_b.voting_power.parse().unwrap();
                validator_b_voting_power.cmp(&validator_a_voting_power)
            });

            for validator in &validators {
                let validator_voting_power: f64 = validator.voting_power.parse().unwrap();
                total_vp = total_vp + validator_voting_power;
            }

            let mut cumulative_val_vp_percentage: f64 = 0.0;
            let mut cumulative_number_of_validators: i32 = 0;

            let mut nakamoto_coefficient: Option<BlockVPResultData> = None;
            let mut takeover_threshold: Option<BlockVPResultData> = None;

            for validator in validators {
                let validator_voting_power: f64 = validator.voting_power.parse().unwrap();
                let val_vp_percentage: f64 = validator_voting_power / total_vp;

                // add to running totals
                cumulative_val_vp_percentage = cumulative_val_vp_percentage + val_vp_percentage;
                cumulative_number_of_validators = cumulative_number_of_validators + 1;
                // println!(
                //     "Total VP: {}, Val VP: {}, Val pc: {}",
                //     total_vp, validator_voting_power, val_vp_percentage
                // );

                if cumulative_val_vp_percentage >= 0.667 {
                    if takeover_threshold.is_none() {
                        takeover_threshold = Some(BlockVPResultData {
                            block_height: block_height,
                            block_voting_power: total_vp,
                            number_of_validators: cumulative_number_of_validators,
                            cumulative_validator_vp_percentage: cumulative_val_vp_percentage
                                * 100.0,
                        })
                    }
                } else if cumulative_val_vp_percentage >= 0.334 {
                    if nakamoto_coefficient.is_none() {
                        nakamoto_coefficient = Some(BlockVPResultData {
                            block_height: block_height,
                            block_voting_power: total_vp,
                            number_of_validators: cumulative_number_of_validators,
                            cumulative_validator_vp_percentage: cumulative_val_vp_percentage
                                * 100.0,
                        })
                    }
                }
            }

            if let (Some(nc), Some(tt)) = (nakamoto_coefficient, takeover_threshold) {
                println!(
                    "Block Height: {}, Total VP: {}",
                    nc.block_height, nc.block_voting_power
                );
                println!(
                    "{} Validators to exceed 33.3% of VP, with {}% of VP",
                    nc.number_of_validators, nc.cumulative_validator_vp_percentage
                );
                println!(
                    "{} Validators to exceed 66.6% of VP, with {}% of VP",
                    tt.number_of_validators, tt.cumulative_validator_vp_percentage
                );

                wtr.serialize(CsvBlockVpResultRecord {
                    block_height: nc.block_height,
                    block_voting_power: nc.block_voting_power,
                    nc_number_of_validators: nc.number_of_validators,
                    nc_cumulative_validator_vp_percentage: nc.cumulative_validator_vp_percentage,
                    tt_number_of_validators: tt.number_of_validators,
                    tt_cumulative_validator_vp_percentage: tt.cumulative_validator_vp_percentage,
                })
                .unwrap();
            }
        }

        wtr.flush().unwrap();
    }
    Ok(())
}
