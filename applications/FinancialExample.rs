use ckks_engine::utils::{encode, decode};
use ckks_engine::{CKKSEncryptor, CKKSDecryptor, CkksParameters, Polynomial};
use std::error::Error;
use csv::Reader;
use chrono::NaiveDate;
use chrono::Datelike;


fn main() -> Result<(), Box<dyn Error>> {
    // Initialize CKKS parameters
    let params = CkksParameters::new(4096, 1000000000000007);
    let keygen = ckks_engine::KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    const SCALE: f64 = 1e7; // Scaling factor for encoding

    println!("Current working directory: {:?}", std::env::current_dir()?);

    // File path to the dataset
    let file_path = "transactions_2024.csv";


    // Specify the year and month to analyze
    let year = 2024;
    let month = 7; // For July

    // Step 1: Read and filter dataset
    let (filtered_amounts, filtered_types) = read_and_filter_csv(file_path, year, month)?;

    // Step 2: Encrypt amounts
    let encrypted_transactions: Vec<Polynomial> = filtered_amounts
        .iter()
        .map(|&amount| encryptor.encrypt_value(amount)) // Encrypt directly using `f64`
        .collect();


    // Step 3: Calculate total Debit and Credit sums
    let mut total_debit_encrypted = encode(&[0.0], SCALE);
    let mut total_credit_encrypted = encode(&[0.0], SCALE);

    for (encrypted_amount, transaction_type) in encrypted_transactions.iter().zip(filtered_types.iter()) {
        if transaction_type == "Debit" {
            total_debit_encrypted = encryptor.homomorphic_add(&total_debit_encrypted, encrypted_amount);
        } else if transaction_type == "Credit" {
            total_credit_encrypted = encryptor.homomorphic_add(&total_credit_encrypted, encrypted_amount);
        }
    }

    // Step 4: Decrypt results
    let total_debit: Vec<f64> = decryptor.decrypt(&total_debit_encrypted); // Decrypt
    let total_credit: Vec<f64> = decryptor.decrypt(&total_credit_encrypted); // Decrypt

    // Step 5: Count transactions
    let debit_count = filtered_types.iter().filter(|&t| t == "Debit").count();
    let credit_count = filtered_types.iter().filter(|&t| t == "Credit").count();

    // Step 6: Print results
    println!("Transaction Summary for {}/{}:", month, year);
    println!("Total Debit Transactions: {}, Sum: {:?}", debit_count, total_debit);
    println!("Total Credit Transactions: {}, Sum: {:?}", credit_count, total_credit);

    Ok(())
}

// Function to read and filter CSV data for a given year and month
// fn read_and_filter_csv(file_path: &str, year: i32, month: u32) -> Result<(Vec<f64>, Vec<String>), Box<dyn Error>> {
//     let mut reader = Reader::from_path(file_path)?;
//     let mut filtered_amounts = Vec::new();
//     let mut filtered_types = Vec::new();

//     for result in reader.records() {
//         let record = result?;
//         let date = NaiveDate::parse_from_str(&record[0], "%d/%m/%Y")?;
//         let transaction_type = record[1].to_string();
//         let amount: f64 = record[2].parse()?;

//         if date.year() == year && date.month() == month {
//             filtered_amounts.push(amount);
//             filtered_types.push(transaction_type);
//         }
//     }

//     Ok((filtered_amounts, filtered_types))
// }


fn read_and_filter_csv(file_path: &str, year: i32, month: u32) -> Result<(Vec<f64>, Vec<String>), Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut filtered_amounts = Vec::new();
    let mut filtered_types = Vec::new();

    for result in reader.records() {
        let record = result?;
        println!("Processing record: {:?}", record); // Debug statement

        // Corrected date format
        if let (Ok(date), Ok(amount)) = (
            NaiveDate::parse_from_str(&record[0], "%Y-%m-%d"),
            record[2].parse::<f64>(),
        ) {
            if date.year() == year && date.month() == month {
                filtered_amounts.push(amount);
                filtered_types.push(record[1].to_string());
            }
        } else {
            eprintln!("Skipping invalid row: {:?}", record); // Log invalid rows
        }
    }

    Ok((filtered_amounts, filtered_types))
}
