use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650301: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_301,
        source_type: SourceType::Wikidata,
        name: "Pack",
        extensions: &["z"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
