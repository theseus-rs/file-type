use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12318121939912865041: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "flac",
    extensions: &["flac"],
    media_types: &["audio/x-flac"],
    internal_signatures: &[],
    related_formats: &[],
};
