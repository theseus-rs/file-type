use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62445151: FileType = FileType {
    file_format: &FileFormat {
        id: 62_445_151,
        source_type: SourceType::Wikidata,
        name: "OWL Functional-Style Syntax",
        extensions: &["ofn"],
        media_types: &["text/owl-functional"],
        signatures: &[],
        related_formats: &[],
    },
};
