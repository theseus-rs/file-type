use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3699261783011718340: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mathml xml",
    extensions: &["mathml"],
    media_types: &["application/mathml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
