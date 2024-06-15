use data_tool::{read_csv, process_data, write_csv};
use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    env_logger::init();
    let input_path = "input.csv";
    let output_path = "output.csv";

    info!("Starting the data processing tool");
    let data = read_csv(input_path)?;
    info!("Read {} records from {}", data.len(), input_path);

    let processed_data = process_data(data)?;
    info!("Processed {} records", processed_data.len());

    write_csv(output_path, &processed_data)?;
    info!("Wrote processed records to {}", output_path);

    Ok(())
}
