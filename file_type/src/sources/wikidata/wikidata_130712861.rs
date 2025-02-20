use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130712861: FileType = FileType {
    file_format: &FileFormat {
        id: 130_712_861,
        source_type: SourceType::Wikidata,
        name: "Relation Query Language file format",
        extensions: &["rql"],
        media_types: &["text/x-rql"],
        signatures: &[],
        related_formats: &[],
    },
};
