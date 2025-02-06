use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3679311052: FileFormat = FileFormat {
    id: 3_679_311_052,
    source_type: SourceType::Httpd,
    name: "fly",
    extensions: &["fly"],
    media_types: &["text/vnd.fly"],
    signatures: &[],
    related_formats: &[],
};
