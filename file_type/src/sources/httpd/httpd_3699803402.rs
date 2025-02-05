use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3699803402: FileFormat = FileFormat {
    id: 3_699_803_402,
    source_type: SourceType::Httpd,
    name: "wav",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    signatures: &[],
    related_formats: &[],
};
