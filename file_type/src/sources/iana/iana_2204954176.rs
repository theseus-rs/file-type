use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2204954176: FileType = FileType {
    file_format: &FileFormat {
        id: 2_204_954_176,
        source_type: SourceType::Iana,
        name: "vnd.1ob",
        extensions: &[],
        media_types: &["application/vnd.1ob"],
        signatures: &[],
        related_formats: &[],
    },
};
