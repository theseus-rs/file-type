use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_496402490: FileType = FileType {
    file_format: &FileFormat {
        id: 496_402_490,
        source_type: SourceType::Iana,
        name: "vnd.dece.audio",
        extensions: &[],
        media_types: &["audio/vnd.dece.audio"],
        signatures: &[],
        related_formats: &[],
    },
};
