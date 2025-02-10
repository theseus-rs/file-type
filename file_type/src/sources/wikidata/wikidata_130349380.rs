use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130349380: FileType = FileType {
    file_format: &FileFormat {
        id: 130_349_380,
        source_type: SourceType::Wikidata,
        name: "Modula-2 source code file",
        extensions: &["def", "mod"],
        media_types: &["text/x-modula2"],
        signatures: &[],
        related_formats: &[],
    },
};
