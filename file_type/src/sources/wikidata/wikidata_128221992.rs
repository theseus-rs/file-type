use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128221992: FileType = FileType {
    file_format: &FileFormat {
        id: 128_221_992,
        source_type: SourceType::Wikidata,
        name: "Zimbu file",
        extensions: &["zu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
