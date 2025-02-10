use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122170514: FileType = FileType {
    file_format: &FileFormat {
        id: 122_170_514,
        source_type: SourceType::Wikidata,
        name: "WireGuard profile",
        extensions: &["conf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
