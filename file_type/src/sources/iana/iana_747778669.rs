use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_747778669: FileType = FileType {
    file_format: &FileFormat {
        id: 747_778_669,
        source_type: SourceType::Iana,
        name: "vnd.visionary",
        extensions: &[],
        media_types: &["application/vnd.visionary"],
        signatures: &[],
        related_formats: &[],
    },
};
