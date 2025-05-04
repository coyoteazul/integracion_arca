///Obtiene un unico tag del XML
pub fn get_xml_tag(xml: &str, tag:&str) -> Option<String> {
	let result = get_xml_vec(xml, tag);
	if result.len() == 0 {
		return None
	} else {
		return Some(result[0].clone());
	}
}

///Busca un tag dentro del XML y devuelve todos los elementos de ese tag como Vec
pub fn get_xml_vec(xml: &str, tag:&str) -> Vec<String> {
	let mut start_tag:String = format!("<{tag}>");
	let mut end_tag:String   = format!("</{tag}>");
	if !xml.contains(&start_tag) {
		start_tag = format!("&lt;{tag}&gt;");
		end_tag   = format!("&lt;/{tag}&gt;");
	}
	
	let slice = String::from(xml)
	.split(&start_tag)
	.skip(1)
	.map(|x| x.split(&end_tag).nth(0).unwrap().to_owned())
	.collect();

	//dbg!(&xml, &start_tag, &end_tag, &slice);
	return slice;
}
