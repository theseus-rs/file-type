use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4077824776: FileFormat = FileFormat {
    id: 4_077_824_776,
    source_type: SourceType::Httpd,
    name: "commonspace",
    extensions: &["csp"],
    media_types: &["application/vnd.commonspace"],
    internal_signatures: &[],
    related_formats: &[],
};
