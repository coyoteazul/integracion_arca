///Obtiene un unico tag del XML
pub fn get_xml_tag(xml: &str, tag: &str) -> Option<String> {
    get_xml_vec(xml, tag).into_iter().next()
}

///Busca un tag dentro del XML y devuelve todos los elementos de ese tag como Vec
pub fn get_xml_vec(xml: &str, tag: &str) -> Vec<String> {
    let mut start_tag: String = format!("<{tag}>");
    let mut end_tag: String = format!("</{tag}>");
    if !xml.contains(&start_tag) {
        start_tag = format!("&lt;{tag}&gt;");
        end_tag = format!("&lt;/{tag}&gt;");
    }

    let slice = xml
        .split(&start_tag)
        .skip(1)
        .filter_map(|x| x.split_once(&end_tag).map(|(value, _)| value.to_owned()))
        .collect();

    //dbg!(&xml, &start_tag, &end_tag, &slice);
    return slice;
}
