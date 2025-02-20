use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27861474: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_474,
        source_type: SourceType::Wikidata,
        name: "Software Independent Archiving of Relational Databases, version 2.0",
        extensions: &["siard"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
