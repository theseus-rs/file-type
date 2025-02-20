use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1078078001: FileType = FileType {
    file_format: &FileFormat {
        id: 1_078_078_001,
        source_type: SourceType::Iana,
        name: "vnd.groove-help",
        extensions: &[],
        media_types: &["application/vnd.groove-help"],
        signatures: &[],
        related_formats: &[],
    },
};
