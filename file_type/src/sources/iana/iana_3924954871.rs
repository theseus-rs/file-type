use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3924954871: FileType = FileType {
    file_format: &FileFormat {
        id: 3_924_954_871,
        source_type: SourceType::Iana,
        name: "prs.nprend",
        extensions: &[],
        media_types: &["application/prs.nprend"],
        signatures: &[],
        related_formats: &[],
    },
};
