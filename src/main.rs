use std::collections::HashMap;

use csv::Error;

use crate::csv_interactor::write_report_to_csv;
use crate::mapped_record::MappedInputRecord;
use crate::provider::Institution;
use crate::report::{API_CONSENT, API_PAYMENTS, ItpReportRecord};

mod mapped_record;
mod csv_interactor;
mod provider;
mod report;

const PAYMENTS_FILE_PATH: &str = "payments.csv";
const CONSENTS_FILE_PATH: &str = "consents.csv";

fn main() -> Result<(), Error> {
    let report_list: Vec<ItpReportRecord> = map_report_record_list();

    println!("\n------- Initiating final data processing\n\
    Number of mapped records from consents and payments: {}", &report_list.len());

    let unplicated = remove_duplicated_records(&report_list);

    write_report_to_csv(unplicated);

    Ok(())
}

fn remove_duplicated_records(report_list: &Vec<ItpReportRecord>) -> HashMap<String, ItpReportRecord> {
    let mut z: HashMap<String, ItpReportRecord> = HashMap::new();
    let mut number_of_substitutions = 0;
    let mut total_insertions = 0;

    for report in report_list {
        let key = format!(
            "{}{}{}{}",
            &report.account_holder_institution,
            &report.http_method,
            &report.http_status,
            &report.api
        );

        if z.contains_key(&key) {
            let e = z.get(&key).cloned().expect("treta");
            let t = ItpReportRecord::clone_and_add_to_quantity(e, report.quantity);

            z.remove(&key);
            z.insert(key, t);
            number_of_substitutions += 1;
        } else {
            z.insert(key, ItpReportRecord::clone_from(report));

            total_insertions += 1;
        }
    }

    println!(
        "Number of duplicated records grouped {}, total records remaining {}",
        number_of_substitutions,
        total_insertions
    );

    z
}

fn map_report_record_list() -> Vec<ItpReportRecord> {
    let institutions = csv_interactor::load_institutions();

    println!("\n------- Initiating mapping from csv inputs to data class (from CSVs to CSV \
    struct)");

    let payments_input = csv_interactor::read_csv(API_PAYMENTS, PAYMENTS_FILE_PATH);
    let consents_input = csv_interactor::read_csv(API_CONSENT, CONSENTS_FILE_PATH);
    let mut report_list: Vec<ItpReportRecord> = Vec::new();

    println!("\n------- Initiating mapping from CSV Struct to Vec<ItpReportRecord>");

    map_to_report_record_and_insert_into_list(API_PAYMENTS, &institutions, payments_input, &mut
        report_list);
    map_to_report_record_and_insert_into_list(API_CONSENT, &institutions, consents_input, &mut
        report_list);

    report_list
}

fn map_to_report_record_and_insert_into_list(
    api: &str,
    institutions: &HashMap<String, Institution>,
    payments_input: Vec<MappedInputRecord>,
    report_list: &mut Vec<ItpReportRecord>
) {
    let report_list_initial_size = report_list.len();

    for payment in payments_input {
        let provider_id = &payment.provider_id.to_string();
        let institution_ref = institutions.get(provider_id).unwrap();
        let report_mapped = ItpReportRecord::new(api, payment, institution_ref);

        report_list.push(report_mapped);
    }

    println!(
        "Number of {} inserted: {}",
        api,
        report_list.len() - report_list_initial_size
    );
}
