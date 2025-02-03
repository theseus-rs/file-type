use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1195271443: FileFormat = FileFormat {
    id: 1_195_271_443,
    source_type: SourceType::Httpd,
    name: "ms modi",
    extensions: &["mdi"],
    media_types: &["image/vnd.ms-modi"],
    internal_signatures: &[],
    related_formats: &[],
};
