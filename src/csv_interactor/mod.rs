use std::collections::HashMap;

use crate::payment_record::PaymentRecord;
use crate::provider::Institution;

pub const PROVIDER_LIST_PATH_FILE: &str = "list_providers.csv";

pub(crate) fn read_csv_payments_input(filename: &str) -> Vec<PaymentRecord> {
    let reader = csv::Reader::from_path(filename);
    let mut payment_inputs: Vec<PaymentRecord> = Vec::new();

    for record in reader.unwrap().records() {
        let record = record.unwrap();
        let payment_record = PaymentRecord::new(record);

        payment_inputs.push(payment_record);
    }

    println!(
        "Number of Payments from input in {}: {}",
        filename,
        payment_inputs.len()
    );

    payment_inputs
}

pub(crate) fn load_institutions() -> HashMap<String, Institution> {
    let reader = csv::Reader::from_path(PROVIDER_LIST_PATH_FILE);
    let mut institutions = HashMap::new();

    for record in reader.unwrap().records() {
        let record = record.unwrap();
        let institution = Institution::new(record);

        institutions.insert(
            institution.provider_id.to_string(),
            institution
        );
    }

    println!(
        "Number of Institutions loaded in {}: {}",
        PROVIDER_LIST_PATH_FILE,
        institutions.len()
    );

    institutions
}