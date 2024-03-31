use std::collections::HashMap;

use csv::Error;

use crate::csv_interactor::write_report_to_csv;
use crate::mapped_record::MappedRecord;
use crate::provider::Institution;
use crate::report::{API_CONSENT, API_PAYMENTS, ItpReport};

mod mapped_record;
mod csv_interactor;
mod provider;
mod report;

const PAYMENTS_FILE_PATH: &str = "payments.csv";
const CONSENTS_FILE_PATH: &str = "consents.csv";

fn main() -> Result<(), Error> {
    let institutions = csv_interactor::load_institutions();
    let payments_input = csv_interactor::read_csv(API_PAYMENTS, PAYMENTS_FILE_PATH);
    let consents_input = csv_interactor::read_csv(API_CONSENT, CONSENTS_FILE_PATH);
    let mut report_list: Vec<ItpReport> = Vec::new();

    map_to_report_record_and_insert_into_list(API_PAYMENTS, &institutions, payments_input, &mut
        report_list);
    map_to_report_record_and_insert_into_list(API_CONSENT, &institutions, consents_input, &mut
        report_list);

    println!("Total number of mapped reports: {}", report_list.len());

    write_report_to_csv(report_list);

    Ok(())
}

fn map_to_report_record_and_insert_into_list(
    api: &str,
    institutions: &HashMap<String, Institution>,
    payments_input: Vec<MappedRecord>,
    report_list: &mut Vec<ItpReport>
) {
    let report_list_initial_size = report_list.len();

    for payment in payments_input {
        let provider_id = &payment.provider_id.to_string();
        let institution_ref = institutions.get(provider_id).unwrap();
        let report_mapped = ItpReport::new(api, payment, institution_ref);

        report_list.push(report_mapped);
    }

    println!(
        "Number of {} inserted: {}",
        api,
        report_list.len() - report_list_initial_size
    );
}
