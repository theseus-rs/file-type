use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111354852: FileType = FileType {
    file_format: &FileFormat {
        id: 111_354_852,
        source_type: SourceType::Wikidata,
        name: "Yamaha Tyros 2 custom voice file",
        extensions: &["tvn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
