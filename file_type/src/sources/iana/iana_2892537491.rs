use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2892537491: FileType = FileType {
    file_format: &FileFormat {
        id: 2_892_537_491,
        source_type: SourceType::Iana,
        name: "vnd.smaf",
        extensions: &[],
        media_types: &["application/vnd.smaf"],
        signatures: &[],
        related_formats: &[],
    },
};
