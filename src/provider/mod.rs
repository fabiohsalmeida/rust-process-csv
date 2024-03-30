use csv::StringRecord;

#[derive(Debug)]
pub struct Institution {
    pub provider_id: i16,
    pub main_institution_nice_name: String,
    pub display_name: String,
    pub organization_id: String,
    pub organization_parent_document: String,
    pub ispb_code: String,
    pub document_number: String
}

impl Institution {
    pub fn new(
        record: StringRecord
    ) -> Institution {
        Institution {
            provider_id: (&record[0]).parse().unwrap(),
            main_institution_nice_name: (&record[1]).parse().unwrap(),
            display_name: (&record[2]).parse().unwrap(),
            organization_id: (&record[3]).parse().unwrap(),
            organization_parent_document: (&record[4]).parse().unwrap(),
            ispb_code: (&record[5]).parse().unwrap(),
            document_number: (&record[6]).parse().unwrap(),
        }
    }
}