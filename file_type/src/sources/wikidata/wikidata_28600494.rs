use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600494: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_494,
        source_type: SourceType::Wikidata,
        name: "Dev-Cpp project",
        extensions: &["dev"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
