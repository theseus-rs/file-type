use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130240779: FileType = FileType {
    file_format: &FileFormat {
        id: 130_240_779,
        source_type: SourceType::Wikidata,
        name: "Literate Haskell source code file",
        extensions: &["lhs"],
        media_types: &["text/x-literate-haskell"],
        signatures: &[],
        related_formats: &[],
    },
};
