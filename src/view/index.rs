#[macro_use]
use sailfish_macros;
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default)]
#[template(path = "index.stpl")]
pub struct Index {
    pub bg: String,
    pub delta: f32,
    pub direction: String,
    pub time: String,
}