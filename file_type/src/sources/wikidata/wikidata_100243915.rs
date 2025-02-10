use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100243915: FileType = FileType {
    file_format: &FileFormat {
        id: 100_243_915,
        source_type: SourceType::Wikidata,
        name: "Student Writing Center Journal",
        extensions: &["jn", "jnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
