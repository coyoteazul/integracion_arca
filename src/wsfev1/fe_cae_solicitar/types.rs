use chrono::NaiveDate;

#[derive(Debug)]
pub struct Wsfev1Ok {
	pub cae : String,
	pub vcto: NaiveDate,
	pub obs : Vec<Wsfev1Obs>,
}

#[derive(Debug)]
pub struct Wsfev1Obs {
	pub code: String,
	pub msg : String,
}