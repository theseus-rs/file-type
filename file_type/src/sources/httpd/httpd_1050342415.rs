use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1050342415: FileFormat = FileFormat {
    id: 1_050_342_415,
    source_type: SourceType::Httpd,
    name: "vcard",
    extensions: &["vcard"],
    media_types: &["text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
