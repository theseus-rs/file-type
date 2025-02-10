use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_219763: FileType = FileType {
    file_format: &FileFormat {
        id: 219_763,
        source_type: SourceType::Wikidata,
        name: "MPEG-4",
        extensions: &["mp4"],
        media_types: &["video/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
