use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2426304290: FileType = FileType {
    file_format: &FileFormat {
        id: 2_426_304_290,
        source_type: SourceType::Iana,
        name: "vnd.nokia.n-gage.data",
        extensions: &[],
        media_types: &["application/vnd.nokia.n-gage.data"],
        signatures: &[],
        related_formats: &[],
    },
};
