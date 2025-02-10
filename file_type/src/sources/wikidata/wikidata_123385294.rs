use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123385294: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_294,
        source_type: SourceType::Wikidata,
        name: "Material library file",
        extensions: &["mtl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
