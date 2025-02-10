use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919168: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_168,
        source_type: SourceType::Wikidata,
        name: "GHS Part Maker",
        extensions: &["pm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
