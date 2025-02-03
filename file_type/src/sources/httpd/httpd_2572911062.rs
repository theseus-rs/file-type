use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2572911062: FileFormat = FileFormat {
    id: 2_572_911_062,
    source_type: SourceType::Httpd,
    name: "kde kivio",
    extensions: &["flw"],
    media_types: &["application/vnd.kde.kivio"],
    internal_signatures: &[],
    related_formats: &[],
};
