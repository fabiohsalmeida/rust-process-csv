use std::collections::HashMap;

use csv::Writer;

use crate::payment_record::PaymentRecord;
use crate::provider::Institution;
use crate::report::ItpReport;

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

pub(crate) fn write_report_to_csv(report_list: Vec<ItpReport>) {
    let mut wtr = Writer::from_path("itp_report.csv").unwrap();

    for report in report_list {
        wtr.serialize(report).expect("Something went wrong in the serialization to report");
    }

    wtr.flush().unwrap();
}