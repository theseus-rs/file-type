use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8488995837589784861: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oebps package xml",
    extensions: &["opf"],
    media_types: &["application/oebps-package+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
