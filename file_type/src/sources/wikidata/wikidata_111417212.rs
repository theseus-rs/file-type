use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111417212: FileType = FileType {
    file_format: &FileFormat {
        id: 111_417_212,
        source_type: SourceType::Wikidata,
        name: "Keyboard file",
        extensions: &["kb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
