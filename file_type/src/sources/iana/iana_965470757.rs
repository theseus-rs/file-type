use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_965470757: FileType = FileType {
    file_format: &FileFormat {
        id: 965_470_757,
        source_type: SourceType::Iana,
        name: "vnd.bzip3",
        extensions: &[],
        media_types: &["application/vnd.bzip3"],
        signatures: &[],
        related_formats: &[],
    },
};
