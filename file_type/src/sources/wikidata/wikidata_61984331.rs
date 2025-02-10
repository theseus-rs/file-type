use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61984331: FileType = FileType {
    file_format: &FileFormat {
        id: 61_984_331,
        source_type: SourceType::Wikidata,
        name: "FoxPro Project",
        extensions: &["pjx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
