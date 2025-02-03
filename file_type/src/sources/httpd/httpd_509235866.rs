use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_509235866: FileFormat = FileFormat {
    id: 509_235_866,
    source_type: SourceType::Httpd,
    name: "ms wpl",
    extensions: &["wpl"],
    media_types: &["application/vnd.ms-wpl"],
    internal_signatures: &[],
    related_formats: &[],
};
