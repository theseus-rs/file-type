use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206095: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_095,
        source_type: SourceType::Wikidata,
        name: "Fuckpaint PI9",
        extensions: &["PI9"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
