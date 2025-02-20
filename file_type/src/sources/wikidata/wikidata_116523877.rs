use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116523877: FileType = FileType {
    file_format: &FileFormat {
        id: 116_523_877,
        source_type: SourceType::Wikidata,
        name: "CoffeeCup CD Info File",
        extensions: &["lav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
