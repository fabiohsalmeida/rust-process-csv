use std::env;

use csv::Error;

mod payment_record;
mod csv_interactor;
mod provider;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    csv_interactor::load_institutions();
    csv_interactor::read_csv_payments_input(file_path);

    Ok(())
}
