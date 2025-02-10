use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85415600: FileType = FileType {
    file_format: &FileFormat {
        id: 85_415_600,
        source_type: SourceType::Wikidata,
        name: "Tweet JSON",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
