use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27349974: FileType = FileType {
    file_format: &FileFormat {
        id: 27_349_974,
        source_type: SourceType::Wikidata,
        name: "TOPSAR Incidence Angle Map",
        extensions: &["incgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
