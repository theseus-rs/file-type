use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27349984: FileType = FileType {
    file_format: &FileFormat {
        id: 27_349_984,
        source_type: SourceType::Wikidata,
        name: "TOPSAR Correlation Map",
        extensions: &["corgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
