use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7708383: FileType = FileType {
    file_format: &FileFormat {
        id: 7_708_383,
        source_type: SourceType::Wikidata,
        name: "textClipping",
        extensions: &["textClipping"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
