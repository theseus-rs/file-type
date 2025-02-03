use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3408491255: FileFormat = FileFormat {
    id: 3_408_491_255,
    source_type: SourceType::Httpd,
    name: "msschedule",
    extensions: &["scd"],
    media_types: &["application/x-msschedule"],
    internal_signatures: &[],
    related_formats: &[],
};
