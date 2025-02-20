use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48551893: FileType = FileType {
    file_format: &FileFormat {
        id: 48_551_893,
        source_type: SourceType::Wikidata,
        name: "WordStar for MS-DOS Document, version 5",
        extensions: &["ws", "ws5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
