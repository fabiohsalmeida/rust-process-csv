use csv::StringRecord;

const KNOWED_HOSTS_TO_REPLACE: [&str; 4] = ["/bancodobrasil", "/bradesco", "/itau-itaucard", "/iti-itaucard"];

#[derive(Debug)]
pub struct MappedInputRecord {
    pub time: String,
    pub api_version: String,
    pub http_host: String,
    pub http_method: String,
    pub http_path: String,
    pub http_status_code: i16,
    pub provider_id: i16,
    pub quantity: i32
}

impl MappedInputRecord {
    pub fn new(
        record: StringRecord
    ) -> MappedInputRecord {
        MappedInputRecord {
            time: (&record[0]).parse().unwrap(),
            api_version: Self::get_version(&record[1]),
            http_host: (&record[2]).parse().unwrap(),
            http_method: (&record[3]).parse().unwrap(),
            http_path: Self::get_endpoint_uri(&record[4]),
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

    // TODO make it work independent from explicitly pattern
    fn get_version(
        field: &str
    ) -> String {
        let pattern = "3.0.0";
        let to_pattern = "v3";

        if field.contains(pattern) {
            return field.replace(pattern, to_pattern).trim().parse().unwrap();
        }

        return field.parse().unwrap();
    }

    // TODO remove any host without list
    fn get_endpoint_uri(
        field: &str
    ) -> String {
        let to_pattern = "";

        for host in KNOWED_HOSTS_TO_REPLACE {
            if field.contains(host) {
                return field.replace(host, to_pattern).trim().parse().unwrap();
            }
        }

        return field.trim().parse().unwrap();
    }
}