use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1353763: FileType = FileType {
    file_format: &FileFormat {
        id: 1_353_763,
        source_type: SourceType::Wikidata,
        name: "WMV HD",
        extensions: &["wmv"],
        media_types: &["video/x-ms-wmv"],
        signatures: &[],
        related_formats: &[],
    },
};
