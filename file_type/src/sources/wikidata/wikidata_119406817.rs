use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119406817: FileType = FileType {
    file_format: &FileFormat {
        id: 119_406_817,
        source_type: SourceType::Wikidata,
        name: "ACT! Database Pointer File",
        extensions: &["pad"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
