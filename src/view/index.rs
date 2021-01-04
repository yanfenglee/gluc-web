#[macro_use]
use sailfish_macros;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    pub bg: String,
    pub delta: f32,
    pub direction: String,
    pub time: String,
}

impl Default for Index {
    fn default() -> Self {
        Index {
            bg: "".to_string(),
            delta: 0.0,
            direction: "".to_string(),
            time: "".to_string(),
        }
    }
}