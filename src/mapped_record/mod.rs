use csv::StringRecord;

#[derive(Debug)]
pub struct MappedRecord {
    pub time: String,
    pub api_version: String,
    pub http_host: String,
    pub http_method: String,
    pub http_path: String,
    pub http_status_code: i16,
    pub provider_id: i16,
    pub quantity: i32
}

impl MappedRecord {
    pub fn new(
        record: StringRecord
    ) -> MappedRecord {
        MappedRecord {
            time: (&record[0]).parse().unwrap(),
            api_version: (&record[1]).parse().unwrap(),
            http_host: (&record[2]).parse().unwrap(),
            http_method: (&record[3]).parse().unwrap(),
            http_path: (&record[4]).parse().unwrap(),
            http_status_code: Self::get_status(&record[5]),
            provider_id: (&record[6]).parse().unwrap(),
            quantity: (&record[7]).parse().unwrap()
        }
    }

    fn get_status(
        field: &str
    ) -> i16 {
        let pattern = " OK";
        let to_empty = "";

        if field.contains(pattern) {
            return field.replace(pattern, to_empty).trim().parse().unwrap();
        }

        return field.parse().unwrap();
    }
}