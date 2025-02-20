use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3980809349: FileType = FileType {
    file_format: &FileFormat {
        id: 3_980_809_349,
        source_type: SourceType::Iana,
        name: "vnd.nebumind.line",
        extensions: &[],
        media_types: &["application/vnd.nebumind.line"],
        signatures: &[],
        related_formats: &[],
    },
};
