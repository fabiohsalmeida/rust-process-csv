use csv::Error;

fn main() -> Result<(), Error> {
    let reader = csv::Reader::from_path("test.csv");

    for record in reader.unwrap().records() {
        let record = record?;

        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }

    Ok(())
}
