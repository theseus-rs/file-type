use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
