use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2677900451: FileFormat = FileFormat {
    id: 2_677_900_451,
    source_type: SourceType::Httpd,
    name: "acucorp",
    extensions: &["atc", "acutc"],
    media_types: &["application/vnd.acucorp"],
    signatures: &[],
    related_formats: &[],
};
