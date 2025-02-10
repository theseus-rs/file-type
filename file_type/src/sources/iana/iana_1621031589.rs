use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1621031589: FileType = FileType {
    file_format: &FileFormat {
        id: 1_621_031_589,
        source_type: SourceType::Iana,
        name: "vnd.dolby.mlp",
        extensions: &[],
        media_types: &["audio/vnd.dolby.mlp"],
        signatures: &[],
        related_formats: &[],
    },
};
