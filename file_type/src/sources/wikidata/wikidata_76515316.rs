use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_76515316: FileType = FileType {
    file_format: &FileFormat {
        id: 76_515_316,
        source_type: SourceType::Wikidata,
        name: "MapInfo Workspace",
        extensions: &["wor"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
