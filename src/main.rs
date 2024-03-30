use std::env;

use csv::Error;

use crate::report::ItpReport;

mod payment_record;
mod csv_interactor;
mod provider;
mod report;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let institutions = csv_interactor::load_institutions();
    let payments_input = csv_interactor::read_csv_payments_input(file_path);

    for payment in payments_input {
        let provider_id = &payment.provider_id.to_string();
        let institution_ref = institutions.get(provider_id).unwrap();

        //.expect(format!("Provider \
        //         {provider_id} not mapped, insert in {PROVIDER_LIST_PATH_FILE}"))

        let report_mapped = ItpReport::new(payment, institution_ref);

        println!(
            "Iniciador: {}, Detentora: {}, API: {}, Endpoint: {}, Método: {}, Status: {}, \
            Quantidade: {}, Versão da API: {}, OrganizationId Iniciador: {}, OrganizationId \
            Detentor: {}, ParentOrganizationReference Detentor: {}, URI do endpoint: {}",
            report_mapped.initiator_institution,
            report_mapped.account_holder_institution,
            report_mapped.api,
            report_mapped.endpoint_description,
            report_mapped.http_method,
            report_mapped.http_status,
            report_mapped.quantity,
            report_mapped.api_version,
            report_mapped.initiator_organization_id,
            report_mapped.account_holder_organization_id,
            report_mapped.account_holder_parent_organization_document,
            report_mapped.endpoint_uri
        )
    }

    Ok(())
}
