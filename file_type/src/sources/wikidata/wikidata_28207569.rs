use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207569: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_569,
        source_type: SourceType::Wikidata,
        name: "Zeiss BIVAS",
        extensions: &["dta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
