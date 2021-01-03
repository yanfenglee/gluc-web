#[macro_use]
use sailfish_macros;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index<'a> {
    pub val: &'a str,
    pub delta: &'a str,
    pub direction: &'a str,
}