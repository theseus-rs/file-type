use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_579055640: FileType = FileType {
    file_format: &FileFormat {
        id: 579_055_640,
        source_type: SourceType::Iana,
        name: "dsr-es201108",
        extensions: &[],
        media_types: &["audio/dsr-es201108"],
        signatures: &[],
        related_formats: &[],
    },
};
