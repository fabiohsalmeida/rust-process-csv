use csv::{Error};
use crate::payment_record::PaymentRecord;

pub(crate) fn read_csv(filename: &str) -> Result<(), Error> {
    let reader = csv::Reader::from_path(filename);
    let mut number_of_payments: i8 = 0;

    for record in reader.unwrap().records() {
        let record = record?;

        let payment_record = PaymentRecord::new(record);

        println!(
            "Time: {}, Version: {}, Host: {}, Method: {}, Path: {}, Status_Code: {}, Provider: {}, Quantity: {}",
            payment_record.time,
            payment_record.api_version,
            payment_record.http_host,
            payment_record.http_method,
            payment_record.http_path,
            payment_record.http_status_code,
            payment_record.provider_id,
            payment_record.quantity
        );

        number_of_payments += 1;
    }

    println!(
        "Number of Payments: {}",
        number_of_payments
    );

    Ok(())
}