use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3356811342: FileType = FileType {
    file_format: &FileFormat {
        id: 3_356_811_342,
        source_type: SourceType::Iana,
        name: "dicom",
        extensions: &[],
        media_types: &["application/dicom"],
        signatures: &[],
        related_formats: &[],
    },
};
