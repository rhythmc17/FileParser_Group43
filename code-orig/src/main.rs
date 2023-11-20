use std::fs::File;
use std::io::Write;
use std::error::Error;
use csv::StringRecord;

#[derive(Debug)]
struct Record {
    field1: String,
    field2: String,
    field3: String,
    field4: String,
    field5: String,
    field6: String,
    field7: String,
    field8: String,
    field9: String,
    // Add more fields as per your CSV structure
}

fn parse_line(record: StringRecord) -> Result<Record, Box<dyn Error>> {
    if record.len() == 9 {
        Ok(Record { 
            field1: record[0].to_string(), 
            field2: record[1].to_string(), 
            field3: record[2].to_string(), 
            field4: record[3].to_string(), 
            field5: record[4].to_string(), 
            field6: record[5].to_string(), 
            field7: record[6].to_string(), 
            field8: record[7].to_string(), 
            field9: record[8].to_string() 
        })
    } else {
        Err("Record does not have 9 fields".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("src/organizations-10000.csv")?;
    let mut wtr = File::create("output.txt")?;

    for result in rdr.records() {
        let record = result?;
        match parse_line(record) {
            Ok(record) => {
                write!(wtr, "{:?}\n", record)?;
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    Ok(())
}