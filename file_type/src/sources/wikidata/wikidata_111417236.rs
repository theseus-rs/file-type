use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111417236: FileType = FileType {
    file_format: &FileFormat {
        id: 111_417_236,
        source_type: SourceType::Wikidata,
        name: "C++ Keyboard Script",
        extensions: &["kb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
