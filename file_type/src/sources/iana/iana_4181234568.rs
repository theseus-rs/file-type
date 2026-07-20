use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4181234568: FileType = FileType {
    file_format: &FileFormat {
        id: 4_181_234_568,
        source_type: SourceType::Iana,
        name: "syslog-msg",
        extensions: &[],
        media_types: &["application/syslog-msg"],
        signatures: &[],
        related_formats: &[],
    },
};
