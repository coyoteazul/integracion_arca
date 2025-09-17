use chrono::{Days, FixedOffset, NaiveDate, Utc};

use crate::{types::errors::{ErrType, SoapFault}, wsfev1::types::{wsfev1obs::Wsfev1Obs, wsfev1ok::Wsfev1Ok}, xml_utils::{get_xml_tag, get_xml_vec}};

pub fn parse_response(
	respuesta : &str,
) -> Result<Wsfev1Ok, ErrType> {

	if respuesta.contains("<soap:Fault>"){
		return Err(SoapFault::from_xml(respuesta).into());
	}

	let mut obs = Vec::<Wsfev1Obs>::new();
	if let Some(obs_tag) = get_xml_tag(respuesta, "Observaciones") {
		for ele in get_xml_vec(&obs_tag, "Obs") {
			obs.push(Wsfev1Obs{ 
				code: get_xml_tag(&ele, "Code").unwrap_or("No se encontro Code al buscar observaciones".to_string()), 
				msg: get_xml_tag(&ele, "Msg").unwrap_or("No se encontro Msg al buscar observaciones".to_string())
			});
		};
	};

	if let Some(err_tag) = get_xml_tag(respuesta, "Errors") {
		for ele in get_xml_vec(&err_tag, "Err") {
			obs.push(Wsfev1Obs{ 
				code: get_xml_tag(&ele, "Code").unwrap_or("No se encontro Code al buscar observaciones".to_string()),
				msg: get_xml_tag(&ele, "Msg").unwrap_or("No se encontro Msg al buscar observaciones".to_string()) 
			});
		};
	};

	match get_xml_tag(respuesta, "Resultado") {
		None => {
			dbg!(respuesta);
			return Err(SoapFault::new("", "Estado de transmision desconocido. No se encontro el tag Resultado en la respuesta").into());
		},
		Some(estado) => {
			dbg!(&estado);
			if estado != "R" {
				let cae_opt = get_xml_tag(respuesta, "CAE");
				let cae_vto_opt = get_xml_tag(respuesta, "CAEFchVto");
				
				match(cae_opt, cae_vto_opt) {
					(Some(cae), Some(vcto_str)) => {
						let tz = FixedOffset::west_opt(3600*3).unwrap();
						match NaiveDate::parse_from_str(&vcto_str, "%Y%m%d") {
								Ok(vcto) => {
									let vcto = vcto
									.and_hms_micro_opt(0, 0, 0, 0).unwrap()
									.and_local_timezone(tz).unwrap().to_utc();

									return Ok(Wsfev1Ok{cae, vcto, obs});
								},
								Err(err) => {
									dbg!("No se pudo parsear bien la fecha. Se asume que es 10 dias mayor a hoy");
									dbg!(err);
									let vcto = Utc::now().checked_add_days(Days::new(10)).unwrap();
									return Ok(Wsfev1Ok{cae, vcto, obs});
								},
						};
					},
					_ => {
						return Err(SoapFault::new("", "La factura fue aprobada pero no se encontro el CAE o su fecha de vencimiento").into());
					}
				}
			} else {
				if obs.len() > 0 {
					if let Some(er) =  obs.iter().find(|x| x.code=="10016") {
						return Err(SoapFault::new(&er.code, &er.msg).into());
					} else {
						return Err(SoapFault::new(&obs[0].code, &obs[0].msg).into());
					}
				} else {
					return Err(SoapFault::new("###", "La factura fue rechazada, pero no sabemos por que").into());
				}
			}
		},
	}
}