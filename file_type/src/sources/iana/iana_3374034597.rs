use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3374034597: FileType = FileType {
    file_format: &FileFormat {
        id: 3_374_034_597,
        source_type: SourceType::Iana,
        name: "dicom+json",
        extensions: &[],
        media_types: &["application/dicom+json"],
        signatures: &[],
        related_formats: &[],
    },
};
