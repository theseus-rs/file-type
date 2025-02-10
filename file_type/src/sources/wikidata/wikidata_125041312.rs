use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125041312: FileType = FileType {
    file_format: &FileFormat {
        id: 125_041_312,
        source_type: SourceType::Wikidata,
        name: "ZynAddSubFX Presets File",
        extensions: &["xpz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
