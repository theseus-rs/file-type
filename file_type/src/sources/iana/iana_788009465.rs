use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_788009465: FileType = FileType {
    file_format: &FileFormat {
        id: 788_009_465,
        source_type: SourceType::Iana,
        name: "vnd.nubaltec.nudoku-game",
        extensions: &[],
        media_types: &["application/vnd.nubaltec.nudoku-game"],
        signatures: &[],
        related_formats: &[],
    },
};
