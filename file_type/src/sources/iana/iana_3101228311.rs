use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3101228311: FileType = FileType {
    file_format: &FileFormat {
        id: 3_101_228_311,
        source_type: SourceType::Iana,
        name: "lost+xml",
        extensions: &[],
        media_types: &["application/lost+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
