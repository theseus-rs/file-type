use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1249998770: FileType = FileType {
    file_format: &FileFormat {
        id: 1_249_998_770,
        source_type: SourceType::Httpd,
        name: "flac",
        extensions: &["flac"],
        media_types: &["audio/x-flac"],
        signatures: &[],
        related_formats: &[],
    },
};
