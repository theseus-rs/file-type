use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28049476: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_476,
        source_type: SourceType::Wikidata,
        name: "RGB Intermediate Format",
        extensions: &["rgb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
