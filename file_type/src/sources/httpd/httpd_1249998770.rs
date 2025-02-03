use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1249998770: FileFormat = FileFormat {
    id: 1_249_998_770,
    source_type: SourceType::Httpd,
    name: "flac",
    extensions: &["flac"],
    media_types: &["audio/x-flac"],
    internal_signatures: &[],
    related_formats: &[],
};
