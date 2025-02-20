use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207503: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_503,
        source_type: SourceType::Wikidata,
        name: "WinMiPS",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
