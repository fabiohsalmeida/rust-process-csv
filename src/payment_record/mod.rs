use csv::StringRecord;

#[derive(Debug)]
pub struct PaymentRecord {
    pub time: String,
    pub api_version: String,
    pub http_host: String,
    pub http_method: String,
    pub http_path: String,
    pub http_status_code: i16,
    pub provider_id: i16,
    pub quantity: i32
}

impl PaymentRecord {
    pub fn new(
        record: StringRecord
    ) -> PaymentRecord {
        PaymentRecord {
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

/*
pub struct ItpReport {
    #[serde(rename = "Iniciador")]
    pub initiator_institution: String,
    #[serde(rename = "Detentora")]
    pub account_holder_institution: String,
    #[serde(rename = "API")]
    pub api: String,
    #[serde(rename = "Endpoint")]
    pub endpoint_description: String,
    #[serde(rename = "Método")]
    pub http_method: String,
    #[serde(rename = "Status")]
    pub http_status: i16,
    #[serde(rename = "Quantidade")]
    pub quantity: i32,
    #[serde(rename = "Versão da API")]
    pub api_version: String,
    #[serde(rename = "OrganizationId Iniciador")]
    pub initiator_organization_id: String,
    #[serde(rename = "OrganizationId Detentor")]
    pub account_holder_organization_id: String,
    #[serde(rename = "ParentOrganizationReference Detentor")]
    pub account_holder_parent_organization_document: String,
    #[serde(rename = "URI do endpoint")]
    pub endpoint_uri: String
}
 */