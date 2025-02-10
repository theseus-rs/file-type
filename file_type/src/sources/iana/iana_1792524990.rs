use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1792524990: FileType = FileType {
    file_format: &FileFormat {
        id: 1_792_524_990,
        source_type: SourceType::Iana,
        name: "vnd.antix.game-component",
        extensions: &[],
        media_types: &["application/vnd.antix.game-component"],
        signatures: &[],
        related_formats: &[],
    },
};
