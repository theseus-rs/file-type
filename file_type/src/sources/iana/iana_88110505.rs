use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_88110505: FileType = FileType {
    file_format: &FileFormat {
        id: 88_110_505,
        source_type: SourceType::Iana,
        name: "vnd.motorola.flexsuite.kmr",
        extensions: &[],
        media_types: &["application/vnd.motorola.flexsuite.kmr"],
        signatures: &[],
        related_formats: &[],
    },
};
