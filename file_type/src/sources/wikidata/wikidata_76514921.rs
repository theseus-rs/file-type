use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_76514921: FileType = FileType {
    file_format: &FileFormat {
        id: 76_514_921,
        source_type: SourceType::Wikidata,
        name: "WinDev Window",
        extensions: &["wdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
