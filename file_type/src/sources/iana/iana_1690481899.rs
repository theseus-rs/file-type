use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1690481899: FileType = FileType {
    file_format: &FileFormat {
        id: 1_690_481_899,
        source_type: SourceType::Iana,
        name: "vnd.shootproof+json",
        extensions: &[],
        media_types: &["application/vnd.shootproof+json"],
        signatures: &[],
        related_formats: &[],
    },
};
