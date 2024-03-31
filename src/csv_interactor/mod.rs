use std::collections::HashMap;

use csv::Writer;

use crate::payment_record::PaymentRecord;
use crate::provider::Institution;
use crate::report::{HTTP_METHOD_GET, HTTP_METHOD_PATCH, HTTP_METHOD_POST, ItpReport};

pub const PROVIDER_LIST_PATH_FILE: &str = "list_providers.csv";

const ALLOWED_GET_STATUS: [i16; 13] = [200, 201, 400, 401, 403, 404, 405, 406, 429, 500, 502, 503,
    529];
const ALLOWED_POST_STATUS: [i16; 15] = [200, 201, 400, 401, 403, 404, 405, 406, 415, 422, 429, 500,
502, 503, 529];
const ALLOWED_PATCH_STATUS: [i16; 14] = [200, 201, 400, 401, 403, 404, 405, 406, 422, 429, 500,
    502, 503, 529];

pub(crate) fn read_csv_payments_input(filename: &str) -> Vec<PaymentRecord> {
    let reader = csv::Reader::from_path(filename);
    let mut payment_inputs: Vec<PaymentRecord> = Vec::new();
    let mut number_of_records_removed = 0;

    for record in reader.unwrap().records() {
        let record = record.unwrap();
        let payment_record = PaymentRecord::new(record);

        if is_status_allowed(&payment_record.http_method, payment_record.http_status_code) {
            payment_inputs.push(payment_record);
        } else {
            number_of_records_removed += 1;
        };
    };

    println!(
        "Number of Payments from input in {}: {}",
        filename,
        payment_inputs.len()
    );

    println!(
        "Number of records removed in payments: {}",
        number_of_records_removed
    );

    payment_inputs
}

fn is_status_allowed(method: &String, status: i16) -> bool {
    is_get_status_allowed(method, status) ||
        is_post_status_allowed(method, status) ||
        is_patch_status_allowed(method, status)
}

fn is_get_status_allowed(method: &String, status: i16) -> bool {
    method.eq(HTTP_METHOD_GET) && ALLOWED_GET_STATUS.contains(&status)
}

fn is_post_status_allowed(method: &String, status: i16) -> bool {
    method.eq(HTTP_METHOD_POST) && ALLOWED_POST_STATUS.contains(&status)
}

fn is_patch_status_allowed(method: &String, status: i16) -> bool {
    method.eq(HTTP_METHOD_PATCH) && ALLOWED_PATCH_STATUS.contains(&status)
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