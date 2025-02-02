use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_368739558323144978: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mswrite",
    extensions: &["wri"],
    media_types: &["application/x-mswrite"],
    internal_signatures: &[],
    related_formats: &[],
};
