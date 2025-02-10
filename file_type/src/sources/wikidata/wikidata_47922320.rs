use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47922320: FileType = FileType {
    file_format: &FileFormat {
        id: 47_922_320,
        source_type: SourceType::Wikidata,
        name: "AutoLISP File",
        extensions: &["lsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
