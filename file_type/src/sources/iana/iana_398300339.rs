use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_398300339: FileType = FileType {
    file_format: &FileFormat {
        id: 398_300_339,
        source_type: SourceType::Iana,
        name: "vnd.hns.audio",
        extensions: &[],
        media_types: &["audio/vnd.hns.audio"],
        signatures: &[],
        related_formats: &[],
    },
};
