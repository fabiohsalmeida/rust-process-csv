mod payment_record;
mod csv_interactor;
mod provider;

use std::env;
use csv::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    return csv_interactor::read_csv(file_path);
}
