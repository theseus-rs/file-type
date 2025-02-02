use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17622281248786972968: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pskc xml",
    extensions: &["pskcxml"],
    media_types: &["application/pskc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
