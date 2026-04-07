use derive_more::Deref;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deref)]
pub struct HTML(String);

// going to need this in the binary eventually anyway
const FRONTEND_TEMPLATE: &str = include_str!("../templates/index.html");

pub fn load_template() -> Result<HTML, Box<dyn Error>> {
    // let mut buf: String = String::new();

    // let mut file = File::open(path)?;
    // let contents = file.read_to_string(&mut buf)?;

    // Ok(HTML(contents.to_string()))

    Ok(HTML(FRONTEND_TEMPLATE.to_string()))
}

pub fn render_template(html: HTML, map: HashMap<String, String>) -> HTML {
    let mut update_me = html.clone();
    map.iter().for_each(|(key, value)| {
        // { is escaped with {, so to replace {{ }} you need {{{{ {key} }}}}
        update_me = update_me.replace(&format!("{{{{ {key} }}}}"), value);
    });

    return HTML(update_me);
}
