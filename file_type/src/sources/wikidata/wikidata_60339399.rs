use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60339399: FileType = FileType {
    file_format: &FileFormat {
        id: 60_339_399,
        source_type: SourceType::Wikidata,
        name: "Open Project File",
        extensions: &["pod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
