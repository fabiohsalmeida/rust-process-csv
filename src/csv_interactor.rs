use csv::Error;

pub(crate) fn read_csv(filename: &str) -> Result<(), Error> {
    let reader = csv::Reader::from_path(filename);

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