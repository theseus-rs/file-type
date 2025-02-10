use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116573999: FileType = FileType {
    file_format: &FileFormat {
        id: 116_573_999,
        source_type: SourceType::Wikidata,
        name: "CoffeeCup Google Site Mapper Profile",
        extensions: &["csm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
