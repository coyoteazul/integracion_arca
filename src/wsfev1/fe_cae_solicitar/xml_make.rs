use super::types::{
    ComprobAsoc, ComprobCabezal, ComprobCliente, ComprobIVA, ComprobOpcionales, ComprobTributos,
    ComprobValores, Comprobante,
};

/// Tipos de comprobante MiPyMEs (FCE): Factura/Debito/Credito para A, B y C (10040/10157).
const MIPYME_TIPOS: [i64; 9] = [201, 202, 203, 206, 207, 208, 211, 212, 213];

pub(super) fn xml_make(comp: &Comprobante, auth_xml: String) -> String {
    const COMP_TIPO_C: [i64; 3] = [11, 12, 13];
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

    if COMP_TIPO_C.contains(tipo_rg1415) {
        val_gravado = val_nogravado.clone();
        val_nogravado = 0.0;
    }

    let fe_cab_ret = format!(
        r#"<ar:FeCabReq>
	<ar:CantReg>1</ar:CantReg>
	<ar:PtoVta>{punto_venta}</ar:PtoVta>
	<ar:CbteTipo>{tipo_rg1415}</ar:CbteTipo>
</ar:FeCabReq>"#
    );

    let fecha_serv = match (servicio_desde, servicio_hasta) {
        (Some(desde), Some(hasta)) => {
            let desde = desde.format("%Y%m%d").to_string();
            let hasta = hasta.format("%Y%m%d").to_string();
            format!(
                r#"<ar:FchServDesde>{desde}</ar:FchServDesde>
<ar:FchServHasta>{hasta}</ar:FchServHasta>"#
            )
        }
        _ => String::new(),
    };

    let periodo_asoc = if let Some(periodo) = &comp.periodo_asociado {
        let desde = periodo.fecha_desde.format("%Y%m%d").to_string();
        let hasta = periodo.fecha_hasta.format("%Y%m%d").to_string();
        format!(
            r#"<ar:PeriodoAsoc>
		<ar:FchDesde>{desde}</ar:FchDesde>
		<ar:FchHasta>{hasta}</ar:FchHasta>
</ar:PeriodoAsoc>"#
        )
    } else {
        String::new()
    };

    let fecha_venc = if let Some(venci) = venci_pago {
        let venci = venci.format("%Y%m%d").to_string();
        format!(r#"<ar:FchVtoPago>{venci}</ar:FchVtoPago>"#)
    } else {
        String::new()
    };

    // CanMisMonExt solo tiene sentido (y ARCA lo rechaza si se envia, cod. 10241) para
    // comprobantes en moneda extranjera. Para PES no debe enviarse.
    let cancela_moneda_ext = if moneda.as_str() != "PES" {
        let flag = if *cancela_misma_moneda { 'S' } else { 'N' };
        format!(r#"<ar:CanMisMonExt>{flag}</ar:CanMisMonExt>"#)
    } else {
        String::new()
    };

    // CbteAsoc.Cuit es obligatorio al asociar un comprobante MiPyMEs (FCE) debito/credito
    // (10151/10122/10154/10155); en el resto de los casos no corresponde informarlo.
    let cuit_asoc = if MIPYME_TIPOS.contains(&tipo_rg1415) {
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
        r#"<ar:FeDetReq>
	<ar:FECAEDetRequest>
		<ar:Concepto>{concepto}</ar:Concepto>
		<ar:DocTipo>{tipo_doc}</ar:DocTipo>
		<ar:DocNro>{documento}</ar:DocNro>
		<ar:CbteDesde>{num_documento}</ar:CbteDesde>
		<ar:CbteHasta>{num_documento}</ar:CbteHasta>
		<ar:CbteFch>{fecha_emision}</ar:CbteFch>
		<ar:ImpTotal>{val_total}</ar:ImpTotal>
		<ar:ImpTotConc>{val_nogravado}</ar:ImpTotConc>
		<ar:ImpNeto>{val_gravado}</ar:ImpNeto>
		<ar:ImpOpEx>{val_exento}</ar:ImpOpEx>
		<ar:ImpTrib>{val_otros_trib}</ar:ImpTrib>
		<ar:ImpIVA>{val_iva}</ar:ImpIVA>
		<ar:MonId>{moneda}</ar:MonId>
		<ar:MonCotiz>{cotizacion}</ar:MonCotiz>
		{cancela_moneda_ext}
		<ar:CondicionIVAReceptorId>{cond_iva}</ar:CondicionIVAReceptorId>
		{fecha_serv}
		{fecha_venc}
		{comp_asoc}
		{tribut}
		{iva}
		{opcion}
		{periodo_asoc}
		{activid}
	</ar:FECAEDetRequest>
</ar:FeDetReq>"#
    );

    format!(
        r#"<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns:ar="http://ar.gov.afip.dif.FEV1/">
   <soap:Header/>
   <soap:Body>
      <ar:FECAESolicitar>
				{auth_xml}
				<ar:FeCAEReq>
					{fe_cab_ret}
					{det_request}
				</ar:FeCAEReq>
			</ar:FECAESolicitar>
   </soap:Body>
</soap:Envelope>"#
    )
}

fn cbte_asoc_xml(com: &Option<Vec<ComprobAsoc>>, cuit_asoc: Option<i64>) -> String {
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
            } = &f;
            let fecha_emision = fecha_emision.format("%Y%m%d").to_string();
            format!(
                r#"
<ar:CbteAsoc>
	<ar:Tipo>{tipo_rg1415}</ar:Tipo>
	<ar:PtoVta>{punto_venta}</ar:PtoVta>
	<ar:Nro>{num_documento}</ar:Nro>
	{cuit_tag}
	<ar:CbteFch>{fecha_emision}</ar:CbteFch>
</ar:CbteAsoc>"#
            )
        })
        .collect();

    format!(r#"<ar:CbtesAsoc>{ar}</ar:CbtesAsoc>"#)
}

fn tributos_xml(trib: &Option<Vec<ComprobTributos>>) -> String {
    if let Some(trib) = trib {
        if trib.len() > 0 {
            let ar = trib
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
                        r#"<ar:Tributo>
	<ar:Id>{id_tributo}</ar:Id>
	<ar:Desc>{desc}</ar:Desc>
	<ar:BaseImp>{base}</ar:BaseImp>
	<ar:Alic>{alicuota}</ar:Alic>
	<ar:Importe>{importe}</ar:Importe>
</ar:Tributo>"#
                    )
                })
                .reduce(|acc, val| acc + &val)
                .unwrap();

            return format!(r#"<ar:Tributos>{ar}</ar:Tributos>"#);
        }
    };

    String::new()
}

fn ivaalic_xml(iva: &Option<Vec<ComprobIVA>>) -> String {
    if let Some(iva) = iva {
        if iva.len() > 0 {
            let ar = iva
                .iter()
                .map(|f| {
                    let ComprobIVA {
                        id_alicuota,
                        base,
                        importe,
                    } = f;
                    format!(
                        r#"<ar:AlicIva>
	<ar:Id>{id_alicuota}</ar:Id>
	<ar:BaseImp>{base}</ar:BaseImp>
	<ar:Importe>{importe}</ar:Importe>
</ar:AlicIva>"#
                    )
                })
                .reduce(|acc, val| acc + &val)
                .unwrap();

            return format!(r#"<ar:Iva>{ar}</ar:Iva>"#);
        }
    };

    String::new()
}

fn opcion_xml(iva: &Option<Vec<ComprobOpcionales>>) -> String {
    if let Some(iva) = iva {
        if iva.len() > 0 {
            let ar = iva
                .iter()
                .map(|f| {
                    let ComprobOpcionales { id, valor } = f;
                    format!(
                        r#"<ar:Opcional>
	<ar:Id>{id}</ar:Id>
	<ar:Valor>{valor}</ar:Valor>
</ar:Opcional>"#
                    )
                })
                .reduce(|acc, val| acc + &val)
                .unwrap();

            return format!(r#"<ar:Opcionales>{ar}</ar:Opcionales>"#);
        }
    };

    String::new()
}

fn actividades_xml(activ: &Option<Vec<String>>) -> String {
    if let Some(activ) = activ {
        if activ.len() > 0 {
            let ar = activ
                .iter()
                .map(|f| {
                    format!(
                        r#"<ar:Actividad>
	<ar:Id>{f}</ar:Id>
</ar:Actividad>"#
                    )
                })
                .reduce(|acc, val| acc + &val)
                .unwrap();

            return format!(r#"<ar:Actividades>{ar}</ar:Actividades>"#);
        }
    };

    String::new()
}
