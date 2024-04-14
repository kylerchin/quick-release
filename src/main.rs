use std::{error::Error, io, process};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("bill_of_materials.csv")?;
    /*
    let mut records =
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        match let record: RawRow = result?;
        //println!("{:?}", record);
    }*/

    let raw_records: Vec<RawRow> = rdr
        .deserialize::<RawRow>()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();

    

        let cleaned_row: Vec<CleanedRow> = raw_records
        .iter(|row: RawRow| {
            let key = row.key.clone().split('-').collect::<Vec<String>>();

            CleanedRow {
                level: key.len(),
                key: key,
                subsystem: row.subsystem.clone(),
                part_number: row.part_number.clone(),
                part_title: row.part_title.clone(),
                responsible_engineer: row.responsible_engineer.clone(),
                procurement_code: row.procurement_code.clone(),
                sub_component_code: row.sub_component_code.clone(),
                variant: row.variant.clone(),
            }
        })
        .collect();

    Ok(())
}

#[derive(Debug, Clone)]
struct CleanedRow {
    level: u8,
    key: Vec<String>,
    subsystem: String,
    part_number: String,
    part_title: String,
    responsible_engineer: String,
    procurement_code: String,
    sub_component_code: String,
    variant: String,
}

#[derive(Debug, serde::Deserialize)]
struct RawRow {
    #[serde(rename = "Level")]
    level: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Subsystem")]
    subsystem: String,
    #[serde(rename = "Part Number")]
    part_number: String,
    #[serde(rename = "Part Title")]
    part_title: String,
    #[serde(rename = "Parent")]
    parent: String,
    #[serde(rename = "Responsible Engineer")]
    responsible_engineer: String,
    #[serde(rename = "Procurement Code")]
    procurement_code: String,
    #[serde(rename = "Sub Component Code")]
    sub_component_code: String,
    #[serde(rename = "Variant")]
    variant: String,
}
