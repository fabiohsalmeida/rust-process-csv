use std::collections::HashMap;

use csv::Writer;

use crate::mapped_record::MappedInputRecord;
use crate::provider::Institution;
use crate::report::{HTTP_METHOD_GET, HTTP_METHOD_PATCH, HTTP_METHOD_POST, ItpReportRecord};

pub const PROVIDER_LIST_PATH_FILE: &str = "list_providers.csv";

const ALLOWED_GET_STATUS: [i16; 13] = [200, 201, 400, 401, 403, 404, 405, 406, 429, 500, 502, 503,
    529];
const ALLOWED_POST_STATUS: [i16; 15] = [200, 201, 400, 401, 403, 404, 405, 406, 415, 422, 429, 500,
502, 503, 529];
const ALLOWED_PATCH_STATUS: [i16; 14] = [200, 201, 400, 401, 403, 404, 405, 406, 422, 429, 500,
    502, 503, 529];

pub(crate) fn read_csv(api: &str, filename: &str) -> Vec<MappedInputRecord> {
    println!("---- Processing {}", api);
    let reader = csv::Reader::from_path(filename);
    let mut inputs: Vec<MappedInputRecord> = Vec::new();
    let mut number_of_records_removed = 0;

    for record in reader.unwrap().records() {
        let record = record.unwrap();
        let mapped_record = MappedInputRecord::new(record);

        if is_status_allowed(&mapped_record.http_method, mapped_record.http_status_code) {
            inputs.push(mapped_record);
        } else {
            number_of_records_removed += 1;
        };
    };

    println!(
        "Accepted {} of {} {} from input in {}",
        inputs.len(),
        inputs.len() + number_of_records_removed,
        api,
        filename
    );

    println!(
        "Number of records removed in {} after filtering allowed status: {}",
        api,
        number_of_records_removed
    );

    inputs
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
    println!("\n------- Loading datasets");
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

pub(crate) fn write_report_to_csv(report_map: HashMap<String, ItpReportRecord>) {
    let mut wtr = Writer::from_path("itp_report.csv").unwrap();

    for report in report_map.values() {
        wtr.serialize(report).expect("Something went wrong in the serialization to report");
    }

    wtr.flush().unwrap();
}