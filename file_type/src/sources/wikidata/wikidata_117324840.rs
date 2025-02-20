use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117324840: FileType = FileType {
    file_format: &FileFormat {
        id: 117_324_840,
        source_type: SourceType::Wikidata,
        name: "Function Tree file",
        extensions: &["fp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
