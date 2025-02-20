use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111190435: FileType = FileType {
    file_format: &FileFormat {
        id: 111_190_435,
        source_type: SourceType::Wikidata,
        name: "JavaServer Page Document",
        extensions: &["jst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
