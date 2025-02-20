use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2091321435: FileType = FileType {
    file_format: &FileFormat {
        id: 2_091_321_435,
        source_type: SourceType::Iana,
        name: "vnd.windows.devicepairing",
        extensions: &[],
        media_types: &["application/vnd.windows.devicepairing"],
        signatures: &[],
        related_formats: &[],
    },
};
