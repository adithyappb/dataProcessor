use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike};
use anyhow::{Context, Result};
use log::info;

/// Represents a record in the input CSV file.
#[derive(Debug, Deserialize)]
pub struct Record {
    pub name: String,
    pub age: u32,
    pub date_of_birth: String,
}

/// Represents a processed record to be written to the output CSV file.
#[derive(Debug, Serialize)]
pub struct ProcessedRecord {
    pub name: String,
    pub age: u32,
    pub date_of_birth: String,
    pub year_of_birth: i32,
}

/// Reads records from a CSV file at the given path.
pub fn read_csv(file_path: &str) -> Result<Vec<Record>> {
    let mut rdr = ReaderBuilder::new()
        .from_path(file_path)
        .with_context(|| format!("Failed to open file at {}", file_path))?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result.with_context(|| "Failed to deserialize record")?;
        records.push(record);
    }
    info!("Successfully read {} records from {}", records.len(), file_path);
    Ok(records)
}

/// Processes the data by calculating the year of birth.
pub fn process_data(data: Vec<Record>) -> Result<Vec<ProcessedRecord>> {
    let mut processed_records = Vec::new();
    for record in data {
        let date_of_birth = NaiveDate::parse_from_str(&record.date_of_birth, "%Y-%m-%d")
            .with_context(|| format!("Failed to parse date: {}", record.date_of_birth))?;
        let year_of_birth = date_of_birth.year();
        let processed_record = ProcessedRecord {
            name: record.name,
            age: record.age,
            date_of_birth: record.date_of_birth,
            year_of_birth,
        };
        processed_records.push(processed_record);
    }
    info!("Successfully processed {} records", processed_records.len());
    Ok(processed_records)
}

/// Writes processed records to a CSV file at the given path.
pub fn write_csv(file_path: &str, data: &[ProcessedRecord]) -> Result<()> {
    let mut wtr = WriterBuilder::new()
        .from_path(file_path)
        .with_context(|| format!("Failed to open file at {}", file_path))?;

    for record in data {
        wtr.serialize(record).with_context(|| "Failed to serialize record")?;
    }
    wtr.flush().with_context(|| "Failed to flush writer")?;
    info!("Successfully wrote {} records to {}", data.len(), file_path);
    Ok(())
}



