use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_46496687: FileType = FileType {
    file_format: &FileFormat {
        id: 46_496_687,
        source_type: SourceType::Wikidata,
        name: "Resource Adapter Archive",
        extensions: &["rar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
