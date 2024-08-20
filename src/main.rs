use std::fmt::Error;
use serde::{Serialize, Deserialize};
use serde_json::{json, to_writer_pretty, Value};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

#[derive(Serialize, Debug)]
struct MyData {
    channel: String,
    target_channel_id: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "data.json";

    // Create the file if it doesn't exist
    create_file_if_needed(file_path)?;

    // Try to change the JSON value
    //change_json_value(file_path, "channel".to_string(), 15.to_string())?;

    Ok(())
}

fn create_file_if_needed(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = match File::open(file_path) {
        Ok(..) => {}, // Return the existing file
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let file = File::create(file_path)?;
            let data = MyData {
                channel: "channel".to_string(),
                target_channel_id: "channel_id".to_string(),
            };
            let mut writer = BufWriter::new(file);
            to_writer_pretty(&mut writer, &data)?;

        }
        Err(error) => return Err(error.into()),
    };

    Ok(file)
}

fn change_json_value(file_path: &str, key: String, value: String) -> Result<(), std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut data: Value = serde_json::from_reader(reader)?;

    data[&key] = json!(value);

    let mut file = File::create(file_path)?;
    to_writer_pretty(&mut file, &data)?;

    Ok(())
}