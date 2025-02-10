use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_380319: FileType = FileType {
    file_format: &FileFormat {
        id: 380_319,
        source_type: SourceType::Wikidata,
        name: "dynamic-link library",
        extensions: &["dll"],
        media_types: &[
            "application/vnd.microsoft.portable-executable",
            "application/x-msdownload",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
