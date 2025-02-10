use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113486423: FileType = FileType {
    file_format: &FileFormat {
        id: 113_486_423,
        source_type: SourceType::Wikidata,
        name: "Persuasion Mac Document 1.0",
        extensions: &["pr1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
