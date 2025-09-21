#[derive(Debug)]
pub struct FEDummyResult {
	pub status          : reqwest::StatusCode,
	pub app_server      : bool,
	pub db_server       : bool,
	pub auth_server     : bool,
	pub milis_respuesta : u128,
}