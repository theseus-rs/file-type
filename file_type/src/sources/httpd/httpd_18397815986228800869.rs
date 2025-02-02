use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18397815986228800869: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "unity",
    extensions: &["unityweb"],
    media_types: &["application/vnd.unity"],
    internal_signatures: &[],
    related_formats: &[],
};
