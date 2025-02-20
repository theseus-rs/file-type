use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67172933: FileType = FileType {
    file_format: &FileFormat {
        id: 67_172_933,
        source_type: SourceType::Wikidata,
        name: "Alias alpha file format",
        extensions: &["alpha"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
