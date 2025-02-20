use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115037903: FileType = FileType {
    file_format: &FileFormat {
        id: 115_037_903,
        source_type: SourceType::Wikidata,
        name: "Software-Independent Archiving of Relational Databases 2.2",
        extensions: &["siard"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
