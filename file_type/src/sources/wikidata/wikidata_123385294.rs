use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
