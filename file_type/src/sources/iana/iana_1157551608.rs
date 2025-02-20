use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1157551608: FileType = FileType {
    file_format: &FileFormat {
        id: 1_157_551_608,
        source_type: SourceType::Iana,
        name: "mipc",
        extensions: &[],
        media_types: &["application/mipc"],
        signatures: &[],
        related_formats: &[],
    },
};
