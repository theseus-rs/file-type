use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113956223: FileType = FileType {
    file_format: &FileFormat {
        id: 113_956_223,
        source_type: SourceType::Wikidata,
        name: "Software-Independent Archiving of Relational Databases, version 2.1",
        extensions: &["siard"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
