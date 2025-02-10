use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3083546734: FileType = FileType {
    file_format: &FileFormat {
        id: 3_083_546_734,
        source_type: SourceType::Iana,
        name: "dicom+xml",
        extensions: &[],
        media_types: &["application/dicom+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
