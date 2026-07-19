use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1983747776: FileType = FileType {
    file_format: &FileFormat {
        id: 1_983_747_776,
        source_type: SourceType::Iana,
        name: "vnd.abdalsecuritygroup.lockbox",
        extensions: &[],
        media_types: &["application/vnd.abdalsecuritygroup.lockbox"],
        signatures: &[],
        related_formats: &[],
    },
};
