use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2984238633: FileType = FileType {
    file_format: &FileFormat {
        id: 2_984_238_633,
        source_type: SourceType::Iana,
        name: "x3d+xml",
        extensions: &[],
        media_types: &["model/x3d+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
