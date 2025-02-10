use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118669802: FileType = FileType {
    file_format: &FileFormat {
        id: 118_669_802,
        source_type: SourceType::Wikidata,
        name: "Shade To Manga Studio file",
        extensions: &["stc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
