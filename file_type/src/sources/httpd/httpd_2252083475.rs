use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2252083475: FileFormat = FileFormat {
    id: 2_252_083_475,
    source_type: SourceType::Httpd,
    name: "musician",
    extensions: &["mus"],
    media_types: &["application/vnd.musician"],
    internal_signatures: &[],
    related_formats: &[],
};
