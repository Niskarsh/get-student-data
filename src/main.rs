use csv::{ReaderBuilder, WriterBuilder};
use get_student_stats::{
    date_of_admission, last_session_date, total_attended_sessions, total_scheduled_sessions,
};
use std::fs::File;
use std::{env, error::Error};
use tokio_postgres::{
    // Error,
    NoTls,
};
// use chrono::{NaiveDateTime, DateTime, Utc};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a connection to the database
    let (client, connection) = tokio_postgres::connect(
        "host=host user=user dbname=dbname password=password port=5432",
        NoTls,
    ).await?;

    // Spawn a task to process the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Get the current working directory
    let current_dir = env::current_dir()?;

    // Construct the path to the CSV file relative to the current working directory
    let csv_path = current_dir.join("src").join("data.csv");

    // Open the CSV file
    let file = File::open(&csv_path)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Construct the path to the CSV file relative to the current working directory
    let output_csv_path = current_dir.join("src").join("output.csv");

    // Create or open the CSV file for writing
    let file = File::create(&output_csv_path)?;

    // Create a CSV writer
    let mut wtr = WriterBuilder::new().from_writer(file);

    // Write rows to the CSV file
    wtr.write_record(&[
        "Email",
        "Date of Admission",
        "Last Session Date",
        "Total Sessions scheduled",
        "Total Sessions Attended",
    ])?;

    // Iterate over each record
    for result in rdr.records() {
        // Extract the record
        let record = result?;

        // Process each field in the record
        for field in record.iter() {
            let email = field;

            // Perform database operations using the client
            let rows_date_of_admission = client.query(&date_of_admission(email), &[]).await?;
            let rows_last_session_date = client.query(&last_session_date(email), &[]).await?;
            let rows_total_scheduled_sessions =
                client.query(&total_scheduled_sessions(email), &[]).await?;
            let rows_total_attended_sessions =
                client.query(&total_attended_sessions(email), &[]).await?;

            println!(
                "{} {} {} {} {}",
                email,
                rows_date_of_admission.len(),
                rows_last_session_date.len(),
                rows_total_scheduled_sessions.len(),
                rows_total_attended_sessions.len()
            );

            let mut current_record = vec![];

            current_record.push(field.to_string());
            if !(rows_date_of_admission.len() == 0) {
                let date_of_admission: chrono::DateTime<chrono::offset::Utc> =
                    rows_date_of_admission[0].get(0);
                current_record.push(date_of_admission.to_string());
            } else {
                current_record.push("".to_string());
            }

            if !(rows_last_session_date.len() == 0) {
                let last_session_date: chrono::DateTime<chrono::offset::Utc> =
                    rows_last_session_date[0].get(0);
                current_record.push(last_session_date.to_string());
            } else {
                current_record.push("".to_string());
                current_record.push("".to_string());
                current_record.push("".to_string());
                wtr.write_record(&current_record)?;

                drop(current_record);
                continue;
            }

            if !(rows_total_scheduled_sessions.len() == 0) {
                let total_scheduled_sessions: i64 = rows_total_scheduled_sessions[0].get(0);
                current_record.push(total_scheduled_sessions.to_string());
            } else {
                current_record.push("".to_string());
            }

            if !(rows_total_attended_sessions.len() == 0) {
                let total_attended_session: i64 = rows_total_attended_sessions[0].get(0);
                current_record.push(total_attended_session.to_string());
            } else {
                current_record.push("".to_string());
            }

            wtr.write_record(&current_record)?;

            drop(current_record);
        }
    }

    wtr.flush()?;
    // for row in &rows_date_of_admission {
    //     // println!("{:?}", row);
    //     // let id: i32 = row.get(0);
    //     let date_of_admission: chrono::DateTime<chrono::offset::Utc> = row.get(0);
    //     // let formatted_time = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
    //     println!(" name: {:?}, {:?}", date_of_admission, rows_last_session_date[0].get(0));
    // }

    Ok(())
}
