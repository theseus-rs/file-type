use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121815720: FileType = FileType {
    file_format: &FileFormat {
        id: 121_815_720,
        source_type: SourceType::Wikidata,
        name: "HMM Packfile",
        extensions: &["pak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
