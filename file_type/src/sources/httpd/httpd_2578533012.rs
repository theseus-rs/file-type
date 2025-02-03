use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2578533012: FileFormat = FileFormat {
    id: 2_578_533_012,
    source_type: SourceType::Httpd,
    name: "kde kontour",
    extensions: &["kon"],
    media_types: &["application/vnd.kde.kontour"],
    internal_signatures: &[],
    related_formats: &[],
};
