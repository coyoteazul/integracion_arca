use chrono::{DateTime, Utc};

use super::wsfev1obs::Wsfev1Obs;

#[derive(Debug)]
pub struct Wsfev1Ok {
	pub cae:String,
	pub vcto:DateTime<Utc>,
	pub obs:Vec<Wsfev1Obs>,
}