use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_866342943: FileType = FileType {
    file_format: &FileFormat {
        id: 866_342_943,
        source_type: SourceType::Iana,
        name: "xcap-error+xml",
        extensions: &[],
        media_types: &["application/xcap-error+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
