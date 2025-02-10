use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131620450: FileType = FileType {
    file_format: &FileFormat {
        id: 131_620_450,
        source_type: SourceType::Wikidata,
        name: "Ansys Fluent Data file format",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
