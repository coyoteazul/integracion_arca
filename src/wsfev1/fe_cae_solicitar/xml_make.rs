use super::types::{
    ComprobAsoc, ComprobCabezal, ComprobCliente, ComprobIVA, ComprobOpcionales, ComprobTributos,
    ComprobValores, Comprobante,
};

pub(super) fn xml_make(comp: &Comprobante, auth_xml: String) -> String {

    let ComprobCabezal {
        punto_venta,
        num_documento,
        tipo_rg1415,
        concepto,
        fecha_emision,
        moneda,
        cotizacion,
        cancela_misma_moneda,
        servicio_desde,
        servicio_hasta,
        venci_pago,
    } = &comp.cabezal;

    let ComprobCliente {
        tipo_doc,
        documento,
        cond_iva,
    } = comp.cliente;

    let &ComprobValores {
        ref val_total,
        mut val_nogravado,
        mut val_gravado,
        ref val_exento,
        ref val_iva,
        ref val_otros_trib,
        ref tributos,
        ref alicuotas_iva,
    } = &comp.valores;

    let fecha_emision = fecha_emision.format("%Y%m%d").to_string();

    if tipo_rg1415.get_info().letra == 'C' {
        val_gravado = val_nogravado.clone();
        val_nogravado = 0.0;
    }

    let fe_cab_ret = format!(
        "<ar:FeCabReq>\
            <ar:CantReg>1</ar:CantReg>\
            <ar:PtoVta>{punto_venta}</ar:PtoVta>\
            <ar:CbteTipo>{tipo_rg1415}</ar:CbteTipo>\
        </ar:FeCabReq>"
    );

    let fecha_serv = match (servicio_desde, servicio_hasta) {
        (Some(desde), Some(hasta)) => {
            let desde = desde.format("%Y%m%d").to_string();
            let hasta = hasta.format("%Y%m%d").to_string();

            format!(
                "<ar:FchServDesde>{desde}</ar:FchServDesde>\
                <ar:FchServHasta>{hasta}</ar:FchServHasta>"
            )
        }
        _ => String::new(),
    };

    let periodo_asoc = if let Some(periodo) = &comp.periodo_asociado {
        let desde = periodo.fecha_desde.format("%Y%m%d").to_string();
        let hasta = periodo.fecha_hasta.format("%Y%m%d").to_string();

        format!(
            "<ar:PeriodoAsoc>\
                <ar:FchDesde>{desde}</ar:FchDesde>\
                <ar:FchHasta>{hasta}</ar:FchHasta>\
            </ar:PeriodoAsoc>"
        )
    } else {
        String::new()
    };

    let fecha_venc = if let Some(venci) = venci_pago {
        let venci = venci.format("%Y%m%d").to_string();

        format!("<ar:FchVtoPago>{venci}</ar:FchVtoPago>")
    } else {
        String::new()
    };

    // CanMisMonExt solo tiene sentido (y ARCA lo rechaza si se envia, cod. 10241)
    // para comprobantes en moneda extranjera. Para PES no debe enviarse.
    let cancela_moneda_ext = if moneda.as_str() != "PES" {
        let flag = if *cancela_misma_moneda { 'S' } else { 'N' };

        format!("<ar:CanMisMonExt>{flag}</ar:CanMisMonExt>")
    } else {
        String::new()
    };

    // CbteAsoc.Cuit es obligatorio al asociar un comprobante MiPyMEs (FCE)
    // debito/credito (10151/10122/10154/10155); en el resto de los casos
    // no corresponde informarlo.
    let cuit_asoc = if tipo_rg1415.get_info().es_pyme {
        Some(documento)
    } else {
        None
    };

    let comp_asoc = cbte_asoc_xml(&comp.comprob_asociados, cuit_asoc);
    let tribut = tributos_xml(tributos);
    let iva = ivaalic_xml(alicuotas_iva);
    let opcion = opcion_xml(&comp.opcionales);
    let activid = actividades_xml(&comp.actividades);

    let det_request = format!(
        "<ar:FeDetReq>\
            <ar:FECAEDetRequest>\
                <ar:Concepto>{concepto}</ar:Concepto>\
                <ar:DocTipo>{tipo_doc}</ar:DocTipo>\
                <ar:DocNro>{documento}</ar:DocNro>\
                <ar:CbteDesde>{num_documento}</ar:CbteDesde>\
                <ar:CbteHasta>{num_documento}</ar:CbteHasta>\
                <ar:CbteFch>{fecha_emision}</ar:CbteFch>\
                <ar:ImpTotal>{val_total}</ar:ImpTotal>\
                <ar:ImpTotConc>{val_nogravado}</ar:ImpTotConc>\
                <ar:ImpNeto>{val_gravado}</ar:ImpNeto>\
                <ar:ImpOpEx>{val_exento}</ar:ImpOpEx>\
                <ar:ImpTrib>{val_otros_trib}</ar:ImpTrib>\
                <ar:ImpIVA>{val_iva}</ar:ImpIVA>\
                <ar:MonId>{moneda}</ar:MonId>\
                <ar:MonCotiz>{cotizacion}</ar:MonCotiz>\
                {cancela_moneda_ext}\
                <ar:CondicionIVAReceptorId>{cond_iva}</ar:CondicionIVAReceptorId>\
                {fecha_serv}\
                {fecha_venc}\
                {comp_asoc}\
                {tribut}\
                {iva}\
                {opcion}\
                {periodo_asoc}\
                {activid}\
            </ar:FECAEDetRequest>\
        </ar:FeDetReq>"
    );

    format!(
        "<soap:Envelope xmlns:soap=\"http://www.w3.org/2003/05/soap-envelope\" \
            xmlns:ar=\"http://ar.gov.afip.dif.FEV1/\">\
            <soap:Header/>\
            <soap:Body>\
                <ar:FECAESolicitar>\
                    {auth_xml}\
                    <ar:FeCAEReq>\
                        {fe_cab_ret}\
                        {det_request}\
                    </ar:FeCAEReq>\
                </ar:FECAESolicitar>\
            </soap:Body>\
        </soap:Envelope>"
    )
}

fn cbte_asoc_xml(
    com: &Option<Vec<ComprobAsoc>>,
    cuit_asoc: Option<i64>,
) -> String {
    let Some(asoc) = com else {
        return String::new();
    };

    if asoc.is_empty() {
        return String::new();
    }

    let cuit_tag = cuit_asoc
        .map(|c| format!("<ar:Cuit>{c}</ar:Cuit>"))
        .unwrap_or_default();

    let ar: String = asoc
        .iter()
        .map(|f| {
            let ComprobAsoc {
                punto_venta,
                num_documento,
                tipo_rg1415,
                fecha_emision,
            } = f;

            let fecha_emision = fecha_emision.format("%Y%m%d").to_string();

            format!(
                "<ar:CbteAsoc>\
                    <ar:Tipo>{tipo_rg1415}</ar:Tipo>\
                    <ar:PtoVta>{punto_venta}</ar:PtoVta>\
                    <ar:Nro>{num_documento}</ar:Nro>\
                    {cuit_tag}\
                    <ar:CbteFch>{fecha_emision}</ar:CbteFch>\
                </ar:CbteAsoc>"
            )
        })
        .collect();

    format!(
        "<ar:CbtesAsoc>\
            {ar}\
        </ar:CbtesAsoc>"
    )
}

fn tributos_xml(trib: &Option<Vec<ComprobTributos>>) -> String {
    let Some(trib) = trib else {
        return String::new();
    };

    if trib.is_empty() {
        return String::new();
    }

    let ar: String = trib
        .iter()
        .map(|f| {
            let ComprobTributos {
                id_tributo,
                desc,
                base,
                alicuota,
                importe,
            } = f;

            format!(
                "<ar:Tributo>\
                    <ar:Id>{id_tributo}</ar:Id>\
                    <ar:Desc>{desc}</ar:Desc>\
                    <ar:BaseImp>{base}</ar:BaseImp>\
                    <ar:Alic>{alicuota}</ar:Alic>\
                    <ar:Importe>{importe}</ar:Importe>\
                </ar:Tributo>"
            )
        })
        .collect();

    format!(
        "<ar:Tributos>\
            {ar}\
        </ar:Tributos>"
    )
}

fn ivaalic_xml(iva: &Option<Vec<ComprobIVA>>) -> String {
    let Some(iva) = iva else {
        return String::new();
    };

    if iva.is_empty() {
        return String::new();
    }

    let ar: String = iva
        .iter()
        .map(|f| {
            let ComprobIVA {
                id_alicuota,
                base,
                importe,
            } = f;

            format!(
                "<ar:AlicIva>\
                    <ar:Id>{id_alicuota}</ar:Id>\
                    <ar:BaseImp>{base}</ar:BaseImp>\
                    <ar:Importe>{importe}</ar:Importe>\
                </ar:AlicIva>"
            )
        })
        .collect();

    format!(
        "<ar:Iva>\
            {ar}\
        </ar:Iva>"
    )
}

fn opcion_xml(opcionales: &Option<Vec<ComprobOpcionales>>) -> String {
    let Some(opcionales) = opcionales else {
        return String::new();
    };

    if opcionales.is_empty() {
        return String::new();
    }

    let ar: String = opcionales
        .iter()
        .map(|f| {
            let ComprobOpcionales { id, valor } = f;

            format!(
                "<ar:Opcional>\
                    <ar:Id>{id}</ar:Id>\
                    <ar:Valor>{valor}</ar:Valor>\
                </ar:Opcional>"
            )
        })
        .collect();

    format!(
        "<ar:Opcionales>\
            {ar}\
        </ar:Opcionales>"
    )
}

fn actividades_xml(activ: &Option<Vec<String>>) -> String {
    let Some(activ) = activ else {
        return String::new();
    };

    if activ.is_empty() {
        return String::new();
    }

    let ar: String = activ
        .iter()
        .map(|f| {
            format!(
                "<ar:Actividad>\
                    <ar:Id>{f}</ar:Id>\
                </ar:Actividad>"
            )
        })
        .collect();

    format!(
        "<ar:Actividades>\
            {ar}\
        </ar:Actividades>"
    )
}