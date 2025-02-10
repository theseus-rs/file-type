use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1631935144: FileType = FileType {
    file_format: &FileFormat {
        id: 1_631_935_144,
        source_type: SourceType::Iana,
        name: "vnd.freelog.comic",
        extensions: &[],
        media_types: &["application/vnd.freelog.comic"],
        signatures: &[],
        related_formats: &[],
    },
};
