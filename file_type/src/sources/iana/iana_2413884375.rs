use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2413884375: FileType = FileType {
    file_format: &FileFormat {
        id: 2_413_884_375,
        source_type: SourceType::Iana,
        name: "vnd.uvvu.mp4",
        extensions: &[],
        media_types: &["video/vnd.uvvu.mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
