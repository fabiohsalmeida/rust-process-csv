use crate::payment_record::PaymentRecord;
use crate::provider::Institution;

const INITIATOR_INSTITUTION_NAME: &str = "PicPay";
const INITIATOR_ORGANIZATION_ID: &str = "c0f47d95-78e7-5f20-b0ea-ff8a75a44a76";
const API_CONSENT: &str = "consents";
const API_PAYMENTS: &str = "payments";
const GET_PAYMENT_ENDPOINT_DESCRIPTION: &str = "Pix - Consultar iniciação de pagamento";
const POST_PAYMENT_ENDPOINT_DESCRIPTION: &str = "Pix - Consultar iniciação de pagamento";
const PATCH_PAYMENT_ENDPOINT_DESCRIPTION: &str = "Pix - Cancelar iniciação de pagamento";
const GET_CONSENT_ENDPOINT_DESCRIPTION: &str = "Consultar consentimento para iniciação de pagamento";
const POST_CONSENT_ENDPOINT_DESCRIPTION: &str = "Criar consentimento para iniciação de pagamento";
const HTTP_METHOD_GET: &str = "GET";
const HTTP_METHOD_POST: &str = "POST";
const HTTP_METHOD_PATCH: &str = "PATCH";
const EMPTY: &str = "";

pub struct ItpReport {
    // #[serde(rename = "Iniciador")]
    pub initiator_institution: String,
    // #[serde(rename = "Detentora")]
    pub account_holder_institution: String,
    // #[serde(rename = "API")]
    pub api: String,
    // #[serde(rename = "Endpoint")]
    pub endpoint_description: String,
    // #[serde(rename = "Método")]
    pub http_method: String,
    // #[serde(rename = "Status")]
    pub http_status: i16,
    // #[serde(rename = "Quantidade")]
    pub quantity: i32,
    // #[serde(rename = "Versão da API")]
    pub api_version: String,
    // #[serde(rename = "OrganizationId Iniciador")]
    pub initiator_organization_id: String,
    // #[serde(rename = "OrganizationId Detentor")]
    pub account_holder_organization_id: String,
    // #[serde(rename = "ParentOrganizationReference Detentor")]
    pub account_holder_parent_organization_document: String,
    // #[serde(rename = "URI do endpoint")]
    pub endpoint_uri: String
}

impl ItpReport {

    pub fn new(
        payment: PaymentRecord,
        institution: &Institution
    ) -> ItpReport {
        let payment_http_method: &str = payment.http_method.as_ref();
        let endpoint_description = Self::get_endpoint_description(
            API_PAYMENTS.to_string(),
            payment_http_method.to_string()
        );

        ItpReport {
            initiator_institution: INITIATOR_INSTITUTION_NAME.to_string(),
            account_holder_institution: institution.main_institution_nice_name.to_string(),
            api: API_PAYMENTS.to_string(),
            endpoint_description,
            http_method: payment_http_method.to_string(),
            http_status: payment.http_status_code,
            quantity: payment.quantity,
            api_version: payment.api_version,
            initiator_organization_id: INITIATOR_ORGANIZATION_ID.to_string(),
            account_holder_organization_id: institution.organization_id.to_string(),
            account_holder_parent_organization_document: institution.organization_parent_document.to_string(),
            endpoint_uri: payment.http_path,
        }
    }

    fn get_endpoint_description(api: String, http_method: String) -> String {
        let mut endpoint_description: String = String::from("");

        if api.eq(&API_CONSENT.to_string()) {
            if http_method.eq(&HTTP_METHOD_GET.to_string()) {
                endpoint_description = GET_CONSENT_ENDPOINT_DESCRIPTION.to_string();
            } else if http_method.eq(&HTTP_METHOD_POST.to_string()) {
                endpoint_description = POST_CONSENT_ENDPOINT_DESCRIPTION.to_string();
            }
        } else if api.eq(&API_PAYMENTS.to_string()) {
            if http_method.eq(&HTTP_METHOD_GET.to_string()) {
                endpoint_description = GET_PAYMENT_ENDPOINT_DESCRIPTION.to_string();
            } else if http_method.eq(&HTTP_METHOD_POST.to_string()) {
                endpoint_description = POST_PAYMENT_ENDPOINT_DESCRIPTION.to_string();
            } else if http_method.eq(&HTTP_METHOD_PATCH.to_string()) {
                endpoint_description = PATCH_PAYMENT_ENDPOINT_DESCRIPTION.to_string();
            }
        };

        endpoint_description.parse().unwrap()
    }

}