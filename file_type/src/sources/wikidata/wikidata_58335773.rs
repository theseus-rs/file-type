use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58335773: FileType = FileType {
    file_format: &FileFormat {
        id: 58_335_773,
        source_type: SourceType::Wikidata,
        name: "Verity Collection Stop List",
        extensions: &["stp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
