use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3478496005: FileType = FileType {
    file_format: &FileFormat {
        id: 3_478_496_005,
        source_type: SourceType::Iana,
        name: "vnd.astraea-software.iota",
        extensions: &[],
        media_types: &["application/vnd.astraea-software.iota"],
        signatures: &[],
        related_formats: &[],
    },
};
