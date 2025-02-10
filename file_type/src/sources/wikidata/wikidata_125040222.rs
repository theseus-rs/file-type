use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125040222: FileType = FileType {
    file_format: &FileFormat {
        id: 125_040_222,
        source_type: SourceType::Wikidata,
        name: "Syntorial file",
        extensions: &["syn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
