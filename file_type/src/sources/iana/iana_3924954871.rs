use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
