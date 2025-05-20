use base64::decode;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::{read, read_dir, write, File};
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

#[derive(Serialize)]
struct ValidatorVotingPowerData {
    address: String,
    total_vp: i64,
    validator_vp: i64,
    rank: i64,
}

// this is a mapping of height -> data for each validator
type ResultMap = HashMap<i64, Vec<ValidatorVotingPowerData>>;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    // ../../dydx
    let voting_power_files = &args[1];
    let _output_file = &args[2];

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

        for path in paths {
            let vp_file_path = path.path();
            println!("Name: {}", vp_file_path.clone().display());
            let vp_file = File::open(vp_file_path).expect("Error: file not found");

            let vd_json: ValidatorsData =
                serde_json::from_reader(vp_file).expect("Error: error while reading");

            let mut total_vp: i64 = 0;

            let validators_at_height: ValidatorsAtHeight = vd_json.result;
            let block_height = validators_at_height.block_height;
            let validators: Vec<Validator> = validators_at_height.validators;

            for validator in validators {
                let validator_voting_power: i64 = validator.voting_power.parse().unwrap();
                total_vp = total_vp + validator_voting_power;
            }

            println!("Block Height: {}, Total VP: {}", block_height, total_vp);
        }
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

        for path in paths {
            let vp_file_path = path.path();
            println!("Name: {}", vp_file_path.clone().display());
            let vp_file = File::open(vp_file_path).expect("Error: file not found");

            let validators_at_height: ValidatorsAtHeight =
                serde_json::from_reader(vp_file).expect("Error: error while reading");

            let mut total_vp: i64 = 0;

            let block_height = validators_at_height.block_height;
            let validators: Vec<Validator> = validators_at_height.validators;

            for validator in validators {
                let validator_voting_power: i64 = validator.voting_power.parse().unwrap();
                total_vp = total_vp + validator_voting_power;
            }

            println!("Block Height: {}, Total VP: {}", block_height, total_vp);
        }
    }
    Ok(())
}
