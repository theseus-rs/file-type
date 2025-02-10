use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125592921: FileType = FileType {
    file_format: &FileFormat {
        id: 125_592_921,
        source_type: SourceType::Wikidata,
        name: "Raw CMYK",
        extensions: &["cmyk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
