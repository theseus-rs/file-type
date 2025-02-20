use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63036114: FileType = FileType {
    file_format: &FileFormat {
        id: 63_036_114,
        source_type: SourceType::Wikidata,
        name: "8-bit ANSI Text",
        extensions: &["ans"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
