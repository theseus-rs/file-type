use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2437831108: FileFormat = FileFormat {
    id: 2_437_831_108,
    source_type: SourceType::Httpd,
    name: "kde karbon",
    extensions: &["karbon"],
    media_types: &["application/vnd.kde.karbon"],
    internal_signatures: &[],
    related_formats: &[],
};
