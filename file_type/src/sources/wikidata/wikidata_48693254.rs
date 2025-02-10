use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48693254: FileType = FileType {
    file_format: &FileFormat {
        id: 48_693_254,
        source_type: SourceType::Wikidata,
        name: "WordStar for MSDOS Document, version 4",
        extensions: &["ws", "ws4"],
        media_types: &["application/x-wordstar"],
        signatures: &[],
        related_formats: &[],
    },
};
