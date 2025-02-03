use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11344963921236519991: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "acucorp",
    extensions: &["atc", "acutc"],
    media_types: &["application/vnd.acucorp"],
    internal_signatures: &[],
    related_formats: &[],
};
