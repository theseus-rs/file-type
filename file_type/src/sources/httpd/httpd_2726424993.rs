use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2726424993: FileFormat = FileFormat {
    id: 2_726_424_993,
    source_type: SourceType::Httpd,
    name: "oebps package xml",
    extensions: &["opf"],
    media_types: &["application/oebps-package+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
