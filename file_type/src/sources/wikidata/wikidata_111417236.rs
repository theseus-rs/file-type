use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
