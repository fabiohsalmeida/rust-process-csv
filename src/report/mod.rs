use crate::mapped_record::MappedInputRecord;
use crate::provider::Institution;

pub const API_CONSENT: &str = "consents";
pub const API_PAYMENTS: &str = "payments";
pub const HTTP_METHOD_GET: &str = "GET";
pub const HTTP_METHOD_POST: &str = "POST";
pub const HTTP_METHOD_PATCH: &str = "PATCH";
const INITIATOR_INSTITUTION_NAME: &str = "PicPay";
const INITIATOR_ORGANIZATION_ID: &str = "c0f47d95-78e7-5f20-b0ea-ff8a75a44a76";
const GET_PAYMENT_ENDPOINT_DESCRIPTION: &str = "Pix - Consultar iniciação de pagamento";
const POST_PAYMENT_ENDPOINT_DESCRIPTION: &str = "Pix - Consultar iniciação de pagamento";
const PATCH_PAYMENT_ENDPOINT_DESCRIPTION: &str = "Pix - Cancelar iniciação de pagamento";
const GET_CONSENT_ENDPOINT_DESCRIPTION: &str = "Consultar consentimento para iniciação de pagamento";
const POST_CONSENT_ENDPOINT_DESCRIPTION: &str = "Criar consentimento para iniciação de pagamento";
const EMPTY: &str = "";

#[derive(serde::Serialize, Clone)]
pub struct ItpReportRecord {
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

impl ItpReportRecord {

    pub fn new(
        api: &str,
        payment: MappedInputRecord,
        institution: &Institution
    ) -> ItpReportRecord {
        let payment_http_method: &str = payment.http_method.as_ref();
        let endpoint_description = Self::get_endpoint_description(
            api.to_string(),
            payment_http_method.to_string()
        );

        ItpReportRecord {
            initiator_institution: INITIATOR_INSTITUTION_NAME.to_string(),
            account_holder_institution: institution.main_institution_nice_name.to_string(),
            api: api.to_string(),
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

    pub fn clone_and_add_to_quantity(from: ItpReportRecord, quantity: i32) -> ItpReportRecord {
        ItpReportRecord {
            initiator_institution: from.initiator_institution.to_string(),
            account_holder_institution: from.account_holder_institution.to_string(),
            api: from.api.to_string(),
            endpoint_description: from.endpoint_description.to_string(),
            http_method: from.http_method.to_string(),
            http_status: from.http_status,
            quantity: from.quantity + quantity,
            api_version: from.api_version.to_string(),
            initiator_organization_id: from.initiator_organization_id.to_string(),
            account_holder_organization_id: from.account_holder_organization_id.to_string(),
            account_holder_parent_organization_document: from.account_holder_parent_organization_document.to_string(),
            endpoint_uri: from.endpoint_uri.to_string(),
        }
    }

    pub fn clone_from(from: &ItpReportRecord) -> ItpReportRecord {
        ItpReportRecord {
            initiator_institution: from.initiator_institution.to_string(),
            account_holder_institution: from.account_holder_institution.to_string(),
            api: from.api.to_string(),
            endpoint_description: from.endpoint_description.to_string(),
            http_method: from.http_method.to_string(),
            http_status: from.http_status,
            quantity: from.quantity,
            api_version: from.api_version.to_string(),
            initiator_organization_id: from.initiator_organization_id.to_string(),
            account_holder_organization_id: from.account_holder_organization_id.to_string(),
            account_holder_parent_organization_document: from.account_holder_parent_organization_document.to_string(),
            endpoint_uri: from.endpoint_uri.to_string(),
        }
    }

    fn get_endpoint_description(api: String, http_method: String) -> String {
        let mut endpoint_description: String = EMPTY.to_string();

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