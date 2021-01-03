#[macro_use]
use sailfish_macros;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    pub val: String,
    pub delta: String,
    pub direction: String,
}