use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_212633: FileType = FileType {
    file_format: &FileFormat {
        id: 212_633,
        source_type: SourceType::Wikidata,
        name: "Advanced Video Coding",
        extensions: &["mp4"],
        media_types: &["video/H264"],
        signatures: &[],
        related_formats: &[],
    },
};
