use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2738674935: FileType = FileType {
    file_format: &FileFormat {
        id: 2_738_674_935,
        source_type: SourceType::Iana,
        name: "EmergencyCallData.VEDS+xml",
        extensions: &[],
        media_types: &["application/EmergencyCallData.VEDS+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
