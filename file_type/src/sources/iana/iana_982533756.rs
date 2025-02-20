use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_982533756: FileType = FileType {
    file_format: &FileFormat {
        id: 982_533_756,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.custom-properties+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.custom-properties+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
