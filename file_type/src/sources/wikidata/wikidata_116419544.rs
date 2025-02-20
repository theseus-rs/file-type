use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116419544: FileType = FileType {
    file_format: &FileFormat {
        id: 116_419_544,
        source_type: SourceType::Wikidata,
        name: "CoffeeCup Website Color Schemer file",
        extensions: &["ccs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
