use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85621860: FileType = FileType {
    file_format: &FileFormat {
        id: 85_621_860,
        source_type: SourceType::Wikidata,
        name: "PFS:First Choice Database",
        extensions: &["fol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
