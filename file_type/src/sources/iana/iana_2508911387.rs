use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2508911387: FileType = FileType {
    file_format: &FileFormat {
        id: 2_508_911_387,
        source_type: SourceType::Iana,
        name: "tamp-status-response",
        extensions: &[],
        media_types: &["application/tamp-status-response"],
        signatures: &[],
        related_formats: &[],
    },
};
