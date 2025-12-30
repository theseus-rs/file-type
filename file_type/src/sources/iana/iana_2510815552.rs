use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2510815552: FileType = FileType {
    file_format: &FileFormat {
        id: 2_510_815_552,
        source_type: SourceType::Iana,
        name: "vnd.vuq",
        extensions: &[],
        media_types: &["application/vnd.vuq"],
        signatures: &[],
        related_formats: &[],
    },
};
