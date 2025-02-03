use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_372858196: FileFormat = FileFormat {
    id: 372_858_196,
    source_type: SourceType::Httpd,
    name: "shf xml",
    extensions: &["shf"],
    media_types: &["application/shf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
