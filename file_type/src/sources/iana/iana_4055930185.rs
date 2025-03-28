use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4055930185: FileType = FileType {
    file_format: &FileFormat {
        id: 4_055_930_185,
        source_type: SourceType::Iana,
        name: "ace+json",
        extensions: &[],
        media_types: &["application/ace+json"],
        signatures: &[],
        related_formats: &[],
    },
};
