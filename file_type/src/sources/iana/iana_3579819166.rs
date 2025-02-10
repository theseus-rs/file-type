use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3579819166: FileType = FileType {
    file_format: &FileFormat {
        id: 3_579_819_166,
        source_type: SourceType::Iana,
        name: "vnd.kdl",
        extensions: &[],
        media_types: &["application/vnd.kdl"],
        signatures: &[],
        related_formats: &[],
    },
};
