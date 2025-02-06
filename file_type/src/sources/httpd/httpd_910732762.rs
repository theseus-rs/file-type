use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_910732762: FileFormat = FileFormat {
    id: 910_732_762,
    source_type: SourceType::Httpd,
    name: "kidspiration",
    extensions: &["kia"],
    media_types: &["application/vnd.kidspiration"],
    signatures: &[],
    related_formats: &[],
};
