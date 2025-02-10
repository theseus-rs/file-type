use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113436221: FileType = FileType {
    file_format: &FileFormat {
        id: 113_436_221,
        source_type: SourceType::Wikidata,
        name: "OBO Flat File Format 1.2",
        extensions: &["obo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
