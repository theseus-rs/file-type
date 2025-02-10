use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128693745: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_745,
        source_type: SourceType::Wikidata,
        name: "BNF file",
        extensions: &["bnf"],
        media_types: &["text/x-bnf"],
        signatures: &[],
        related_formats: &[],
    },
};
