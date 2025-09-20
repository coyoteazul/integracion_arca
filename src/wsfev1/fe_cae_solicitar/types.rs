use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Wsfev1Ok {
	pub cae:String,
	pub vcto:DateTime<Utc>,
	pub obs:Vec<Wsfev1Obs>,
}

#[derive(Debug)]
pub struct Wsfev1Obs {
	pub code:String,
	pub msg:String,
}