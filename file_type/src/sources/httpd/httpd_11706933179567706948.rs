use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11706933179567706948: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msschedule",
    extensions: &["scd"],
    media_types: &["application/x-msschedule"],
    internal_signatures: &[],
    related_formats: &[],
};
