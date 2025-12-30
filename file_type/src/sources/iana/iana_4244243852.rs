use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4244243852: FileType = FileType {
    file_format: &FileFormat {
        id: 4_244_243_852,
        source_type: SourceType::Iana,
        name: "asyncapi+yaml",
        extensions: &[],
        media_types: &["application/asyncapi+yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
