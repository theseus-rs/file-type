use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_616714: FileType = FileType {
    file_format: &FileFormat {
        id: 616_714,
        source_type: SourceType::Wikidata,
        name: "Initial Graphics Exchange Specification",
        extensions: &["iges", "igs"],
        media_types: &["application/iges", "model/iges"],
        signatures: &[],
        related_formats: &[],
    },
};
