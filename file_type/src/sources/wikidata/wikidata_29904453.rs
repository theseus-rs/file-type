use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29904453: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_453,
        source_type: SourceType::Wikidata,
        name: "PowerPacker",
        extensions: &["pp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
