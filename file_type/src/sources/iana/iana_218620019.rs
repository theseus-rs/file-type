use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_218620019: FileType = FileType {
    file_format: &FileFormat {
        id: 218_620_019,
        source_type: SourceType::Iana,
        name: "vnd.dolby.mobile.1",
        extensions: &[],
        media_types: &["application/vnd.dolby.mobile.1"],
        signatures: &[],
        related_formats: &[],
    },
};
