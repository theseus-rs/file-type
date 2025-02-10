use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1211430670: FileType = FileType {
    file_format: &FileFormat {
        id: 1_211_430_670,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.text",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.text"],
        signatures: &[],
        related_formats: &[],
    },
};
