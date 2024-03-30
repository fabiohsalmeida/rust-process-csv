use std::env;

use csv::Error;
use crate::csv_interactor::write_report_to_csv;

use crate::report::ItpReport;

mod payment_record;
mod csv_interactor;
mod provider;
mod report;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let institutions = csv_interactor::load_institutions();
    let payments_input = csv_interactor::read_csv_payments_input(file_path);
    let mut report_list: Vec<ItpReport> = Vec::new();

    for payment in payments_input {
        let provider_id = &payment.provider_id.to_string();
        let institution_ref = institutions.get(provider_id).unwrap();
        let report_mapped = ItpReport::new(payment, institution_ref);

        report_list.push(report_mapped);
    }

    println!("Number of mapped payments: {}", report_list.len());

    write_report_to_csv(report_list);

    Ok(())
}
