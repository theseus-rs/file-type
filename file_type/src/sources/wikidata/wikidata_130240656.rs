use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130240656: FileType = FileType {
    file_format: &FileFormat {
        id: 130_240_656,
        source_type: SourceType::Wikidata,
        name: "Literate Cryptol source code file",
        extensions: &["lcry"],
        media_types: &["text/x-literate-cryptol"],
        signatures: &[],
        related_formats: &[],
    },
};
