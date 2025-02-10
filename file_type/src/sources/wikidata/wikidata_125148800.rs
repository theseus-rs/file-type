use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125148800: FileType = FileType {
    file_format: &FileFormat {
        id: 125_148_800,
        source_type: SourceType::Wikidata,
        name: "YAM Users",
        extensions: &["users"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
