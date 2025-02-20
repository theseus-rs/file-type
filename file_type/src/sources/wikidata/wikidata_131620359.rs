use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131620359: FileType = FileType {
    file_format: &FileFormat {
        id: 131_620_359,
        source_type: SourceType::Wikidata,
        name: "Ansys Fluent file format",
        extensions: &["cas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
