use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125041198: FileType = FileType {
    file_format: &FileFormat {
        id: 125_041_198,
        source_type: SourceType::Wikidata,
        name: "ZynAddSubFX Instrument File",
        extensions: &["xiz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
