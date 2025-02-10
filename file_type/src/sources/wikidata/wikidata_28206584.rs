use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206584: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_584,
        source_type: SourceType::Wikidata,
        name: "MGR bitmap",
        extensions: &["mgr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
