use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111417217: FileType = FileType {
    file_format: &FileFormat {
        id: 111_417_217,
        source_type: SourceType::Wikidata,
        name: "Assembly Language Source Code File",
        extensions: &["asm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
