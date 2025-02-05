use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1875755222: FileFormat = FileFormat {
    id: 1_875_755_222,
    source_type: SourceType::Httpd,
    name: "h263",
    extensions: &["h263"],
    media_types: &["video/h263"],
    signatures: &[],
    related_formats: &[],
};
