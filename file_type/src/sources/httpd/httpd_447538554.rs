use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_447538554: FileFormat = FileFormat {
    id: 447_538_554,
    source_type: SourceType::Httpd,
    name: "ms wmv",
    extensions: &["wmv"],
    media_types: &["video/x-ms-wmv"],
    signatures: &[],
    related_formats: &[],
};
