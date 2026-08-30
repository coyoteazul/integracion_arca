use chrono::{Days, NaiveDate, Utc};
use reqwest::StatusCode;
use tracing::{debug, error, warn};

use crate::{
    types::errors::{ErrType, SoapFault},
    wsfev1::fe_cae_solicitar::types::{Wsfev1Obs, Wsfev1Ok},
    xml_utils::{get_xml_tag, get_xml_vec},
};

pub fn parse_response(respuesta: &str, status: StatusCode) -> Result<Wsfev1Ok, ErrType> {
    if respuesta.contains("<soap:Fault>") {
        error!(respuesta = %respuesta, "FECAESolicitar devolvio un SOAP Fault");
        return Err(SoapFault::from_xml(respuesta).into());
    }

    let mut obs = Vec::<Wsfev1Obs>::new();
    if let Some(obs_tag) = get_xml_tag(respuesta, "Observaciones") {
        for ele in get_xml_vec(&obs_tag, "Obs") {
            obs.push(Wsfev1Obs {
                code: get_xml_tag(&ele, "Code")
                    .unwrap_or("No se encontro Code al buscar observaciones".to_string()),
                msg: get_xml_tag(&ele, "Msg")
                    .unwrap_or("No se encontro Msg al buscar observaciones".to_string()),
            });
        }
    };

    if let Some(err_tag) = get_xml_tag(respuesta, "Errors") {
        for ele in get_xml_vec(&err_tag, "Err") {
            obs.push(Wsfev1Obs {
                code: get_xml_tag(&ele, "Code")
                    .unwrap_or("No se encontro Code al buscar observaciones".to_string()),
                msg: get_xml_tag(&ele, "Msg")
                    .unwrap_or("No se encontro Msg al buscar observaciones".to_string()),
            });
        }
    };

    if !obs.is_empty() {
        warn!(?obs, "Observaciones/Errores encontrados en la respuesta");
    }

    match get_xml_tag(respuesta, "Resultado") {
        None => {
            error!(respuesta = %respuesta, %status, "No se encontro el tag Resultado en la respuesta");
            return Err(SoapFault::new("", format!("Estado de transmision desconocido (status:{status}). No se encontro el tag Resultado en la respuesta").as_str()).into());
        }
        Some(estado) => {
            debug!(estado = %estado, "Resultado de FECAESolicitar");
            if estado != "R" {
                let cae_opt = get_xml_tag(respuesta, "CAE");
                let cae_vto_opt = get_xml_tag(respuesta, "CAEFchVto");

                match (cae_opt, cae_vto_opt) {
                    (Some(cae), Some(vcto_str)) => {
                        match NaiveDate::parse_from_str(&vcto_str, "%Y%m%d") {
                            Ok(vcto) => {
                                debug!(cae = %cae, vcto = %vcto, "CAE obtenido correctamente");
                                return Ok(Wsfev1Ok { cae, vcto, obs });
                            }
                            Err(err) => {
                                warn!(
                                    error = ?err,
                                    vcto_str = %vcto_str,
                                    "No se pudo parsear la fecha de vencimiento del CAE. Se asume que es 10 dias mayor a hoy"
                                );
                                let vcto = Utc::now()
                                    .checked_add_days(Days::new(10))
                                    .unwrap()
                                    .date_naive();
                                return Ok(Wsfev1Ok { cae, vcto, obs });
                            }
                        };
                    }
                    _ => {
                        error!(respuesta = %respuesta, "El documento fue aprobado pero no se encontro el CAE o su fecha de vencimiento");
                        return Err(SoapFault::new("", "El documento fue aprobado pero no se encontro el CAE o su fecha de vencimiento").into());
                    }
                }
            } else {
                if obs.len() > 0 {
                    warn!(?obs, "Documento rechazado por ARCA");
                    if let Some(er) = obs.iter().find(|x| x.code == "10016") {
                        return Err(SoapFault::new(&er.code, &er.msg).into());
                    } else {
                        return Err(SoapFault::new(&obs[0].code, &obs[0].msg).into());
                    }
                } else {
                    error!(respuesta = %respuesta, "El documento fue rechazado, pero no sabemos por que");
                    return Err(SoapFault::new(
                        "###",
                        "El documento fue rechazado, pero no sabemos por que",
                    )
                    .into());
                }
            }
        }
    }
}
