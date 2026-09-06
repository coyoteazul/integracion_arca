pub(super) fn xml_make(pto_vta: i64, cbte_tipo: i64, auth_xml: String) -> String {
    format!(
        "<soap:Envelope xmlns:soap=\"http://www.w3.org/2003/05/soap-envelope\" \
            xmlns:ar=\"http://ar.gov.afip.dif.FEV1/\">\
            <soap:Header/>\
            <soap:Body>\
                <ar:FECompUltimoAutorizado>\
                    {auth_xml}\
                    <ar:PtoVta>{pto_vta}</ar:PtoVta>\
                    <ar:CbteTipo>{cbte_tipo}</ar:CbteTipo>\
                </ar:FECompUltimoAutorizado>\
            </soap:Body>\
        </soap:Envelope>"
    )
}