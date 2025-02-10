use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125692058: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_058,
        source_type: SourceType::Wikidata,
        name: "OpenDocument HTML Template file",
        extensions: &["oth"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
