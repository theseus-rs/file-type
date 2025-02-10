use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48902661: FileType = FileType {
    file_format: &FileFormat {
        id: 48_902_661,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Presentation Slideshow",
        extensions: &["shw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
