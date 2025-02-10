use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1687570688: FileType = FileType {
    file_format: &FileFormat {
        id: 1_687_570_688,
        source_type: SourceType::Iana,
        name: "mp4",
        extensions: &[],
        media_types: &["video/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
