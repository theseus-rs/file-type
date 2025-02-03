use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4066570451: FileFormat = FileFormat {
    id: 4_066_570_451,
    source_type: SourceType::Httpd,
    name: "epson quickanime",
    extensions: &["qam"],
    media_types: &["application/vnd.epson.quickanime"],
    internal_signatures: &[],
    related_formats: &[],
};
