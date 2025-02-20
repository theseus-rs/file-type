use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1423901882: FileType = FileType {
    file_format: &FileFormat {
        id: 1_423_901_882,
        source_type: SourceType::Iana,
        name: "vnd.dolby.mps",
        extensions: &[],
        media_types: &["audio/vnd.dolby.mps"],
        signatures: &[],
        related_formats: &[],
    },
};
