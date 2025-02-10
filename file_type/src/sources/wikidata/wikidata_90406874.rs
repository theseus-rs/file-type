use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_90406874: FileType = FileType {
    file_format: &FileFormat {
        id: 90_406_874,
        source_type: SourceType::Wikidata,
        name: "QuickTake format",
        extensions: &["qtk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
