use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117318924: FileType = FileType {
    file_format: &FileFormat {
        id: 117_318_924,
        source_type: SourceType::Wikidata,
        name: "WordPerfect Graphic 2.0",
        extensions: &["wp2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
