use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1233087473: FileType = FileType {
    file_format: &FileFormat {
        id: 1_233_087_473,
        source_type: SourceType::Iana,
        name: "vnd.ms-windows.devicepairing",
        extensions: &[],
        media_types: &["application/vnd.ms-windows.devicepairing"],
        signatures: &[],
        related_formats: &[],
    },
};
