use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8533502177668954884: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pls xml",
    extensions: &["pls"],
    media_types: &["application/pls+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
