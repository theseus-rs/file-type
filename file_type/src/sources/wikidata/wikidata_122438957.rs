use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122438957: FileType = FileType {
    file_format: &FileFormat {
        id: 122_438_957,
        source_type: SourceType::Wikidata,
        name: "TurboTax 2009 Tax Return",
        extensions: &["tax2009"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
