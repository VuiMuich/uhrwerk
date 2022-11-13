use crate::template::Template;
use std::{fs, path::Path};

pub(crate) fn load_template(template_path: Option<String>) -> Template {
    load_from_file(&template_path.unwrap())
        .map_err(|err| println!("ERROR loading template: {:?}", err))
        .unwrap_or_default()
}

pub(crate) fn load_from_file(p: &str) -> Result<Template, Option<ron::de::SpannedError>> {
    if Path::new(&p).exists() {
        let contents = fs::read_to_string(p);
        let template: Result<Template, ron::de::SpannedError> =
            ron::from_str(contents.unwrap().as_str());
        return match template {
            Ok(t) => Ok(t),
            Err(e) => Err(Some(e)),
        };
    }
    // This is a ugly hack until proper Errors get implemented
    Err(None)
}
