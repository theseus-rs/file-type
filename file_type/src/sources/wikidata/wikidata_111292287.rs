use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111292287: FileType = FileType {
    file_format: &FileFormat {
        id: 111_292_287,
        source_type: SourceType::Wikidata,
        name: "linux shared library",
        extensions: &["so"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
