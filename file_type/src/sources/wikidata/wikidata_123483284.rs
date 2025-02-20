use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123483284: FileType = FileType {
    file_format: &FileFormat {
        id: 123_483_284,
        source_type: SourceType::Wikidata,
        name: "Python type stub file",
        extensions: &["pyi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
