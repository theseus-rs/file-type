use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48694183: FileType = FileType {
    file_format: &FileFormat {
        id: 48_694_183,
        source_type: SourceType::Wikidata,
        name: "WordStar for MS-DOS Document, version 7",
        extensions: &["ws", "ws7"],
        media_types: &["application/x-wordstar"],
        signatures: &[],
        related_formats: &[],
    },
};
