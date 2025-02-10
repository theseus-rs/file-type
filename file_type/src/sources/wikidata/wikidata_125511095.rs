use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125511095: FileType = FileType {
    file_format: &FileFormat {
        id: 125_511_095,
        source_type: SourceType::Wikidata,
        name: "Zoner Photo Studio file",
        extensions: &["zps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
