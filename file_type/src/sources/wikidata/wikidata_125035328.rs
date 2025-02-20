use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125035328: FileType = FileType {
    file_format: &FileFormat {
        id: 125_035_328,
        source_type: SourceType::Wikidata,
        name: "TinkerPlots document",
        extensions: &["tp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
