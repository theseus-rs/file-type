use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207369: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_369,
        source_type: SourceType::Wikidata,
        name: "Technicolor Dream LUM",
        extensions: &["lum"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
