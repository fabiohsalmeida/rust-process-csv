mod csv_interactor;

use csv::Error;

fn main() -> Result<(), Error> {
    return csv_interactor::read_csv("test.csv");
}
