use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1013763948: FileType = FileType {
    file_format: &FileFormat {
        id: 1_013_763_948,
        source_type: SourceType::Iana,
        name: "vnd.openblox.game+xml",
        extensions: &[],
        media_types: &["application/vnd.openblox.game+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
