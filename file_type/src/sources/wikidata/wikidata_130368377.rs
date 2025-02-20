use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130368377: FileType = FileType {
    file_format: &FileFormat {
        id: 130_368_377,
        source_type: SourceType::Wikidata,
        name: "nesC source code file",
        extensions: &["nc"],
        media_types: &["text/x-nescsrc"],
        signatures: &[],
        related_formats: &[],
    },
};
