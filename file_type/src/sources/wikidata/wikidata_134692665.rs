use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134692665: FileType = FileType {
    file_format: &FileFormat {
        id: 134_692_665,
        source_type: SourceType::Wikidata,
        name: "NooJ text file",
        extensions: &["not"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
