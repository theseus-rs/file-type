use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6389577377495108203: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "docbook xml",
    extensions: &["dbk"],
    media_types: &["application/docbook+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
