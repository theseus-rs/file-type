use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113171368: FileType = FileType {
    file_format: &FileFormat {
        id: 113_171_368,
        source_type: SourceType::Wikidata,
        name: "Family Lawyer Document",
        extensions: &["pfl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
