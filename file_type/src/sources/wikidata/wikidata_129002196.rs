use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129002196: FileType = FileType {
    file_format: &FileFormat {
        id: 129_002_196,
        source_type: SourceType::Wikidata,
        name: "EBNF file format",
        extensions: &["ebnf"],
        media_types: &["text/x-ebnf"],
        signatures: &[],
        related_formats: &[],
    },
};
