use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111190469: FileType = FileType {
    file_format: &FileFormat {
        id: 111_190_469,
        source_type: SourceType::Wikidata,
        name: "Adobe Extension Data Markup Language Document",
        extensions: &["edml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
