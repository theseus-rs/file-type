use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117537130: FileType = FileType {
    file_format: &FileFormat {
        id: 117_537_130,
        source_type: SourceType::Wikidata,
        name: "WordPerfect Presentations 2",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
